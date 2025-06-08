# 日志6

## 调度模块优化（src/sched）

### 优化点1：CpuRunQueue 初始化锁状态管理优化（基于 sched/mod.rs ）

观察 CpuRunQueue::new 方法中， lock （ SpinLock ）和 lock_on_who （ AtomicUsize ）用于记录锁状态及持有者。高频调度操作（如进程入队/出队）中，这两个字段常被 同时访问 （获取锁时需检查 lock_on_who 是否为当前CPU），导致多次原子操作。 

优化方案 ：将 lock 和 lock_on_who 合并为一个原子结构（如 AtomicU64 ），通过位域分别表示锁状态（1位）和持有者CPU ID（剩余位）。访问时仅需一次原子操作，减少锁竞争概率。 

优化后代码示例：
~~~
pub struct CpuRunQueue {
    // 合并锁状态与持有者为原子字段（假设CPU ID不超过63位）
    lock_state: AtomicU64,  // 低1位：锁状态（0=未锁，1=已锁）；剩余位：持有者CPU ID
    // ... existing code ...
    cfs: Arc<CfsRunQueue>,
    // ... existing code ...
}

impl CpuRunQueue {
    pub fn new(cpu: ProcessorId) -> Self {
        Self {
            // 初始状态：未锁，持有者为无效值（usize::MAX）
            lock_state: AtomicU64::new(0),
            // ... existing code ...
        }
    }
}
~~~
优化说明 ：合并后，锁获取/释放操作仅需一次原子指令（如 compare_exchange ），降低高频调度场景下的锁竞争开销。

### 优化点2：do_sched_yield 中断禁用时间优化（基于 sched/syscall.rs ）

在 do_sched_yield 函数中，通过 save_and_disable_irq 禁用中断后，持有 rq 锁的时间较长（包括 CompletelyFairScheduler::yield_task 等操作）。过长的中断禁用会降低系统响应性（如无法及时处理硬件中断）。 

优化方案 ：在获取 rq 锁后，立即恢复中断（若锁操作本身已保证原子性），仅保留必要的中断禁用窗口。 

优化后代码示例：
~~~
pub fn do_sched_yield() -> Result<usize, SystemError> {
    // 仅禁用中断至获取锁前
    let irq_guard = unsafe { CurrentIrqArch::save_and_disable_irq() };
    let pcb = ProcessManager::current_pcb();
    let rq = cpu_rq(pcb.sched_info().on_cpu().unwrap_or(current_cpu_id()).data() as usize);
    let (rq, guard) = rq.self_lock();  // 获取锁时中断已禁用
    drop(irq_guard);  // 提前恢复中断（锁已持有，后续操作无需中断禁用）

    // ... existing code (yield_task、preempt_disable等) ...

    Ok(0)
}
~~~
优化说明 ：缩短中断禁用时间，提升系统对外部中断（如定时器、I/O）的响应速度，同时保证 rq 锁操作的原子性。
