# 日志5

## 进程管理代码逻辑优化（src/process/mod.rs）

### 优化点1：减少 ProcessControlBlock 锁竞争（基于 mod.rs ）

当前 ProcessControlBlock 结构体中使用了大量独立的 RwLock 和 SpinLock （如 basic 、 syscall_stack 、 sig_info 等），高频访问时可能导致锁竞争。观察到进程基本信息（ ProcessBasicInfo ）和系统调用栈（ KernelStack ）在进程切换、上下文保存等操作中常被 同时访问 ，可将这两个字段合并为一个 RwLock 保护的复合结构，减少锁操作次数。

优化后代码：
~~~
// 合并基础信息和系统调用栈为一个复合结构
struct ProcessBasicAndStackInfo {
    basic: ProcessBasicInfo,
    syscall_stack: KernelStack,
}

pub struct ProcessControlBlock {
    // ... existing code ...
    // 使用单个 RwLock 保护关联字段
    basic_and_stack: RwLock<ProcessBasicAndStackInfo>,
    // ... existing code ...
}
~~~
优化说明 ：合并后，对 basic 和 syscall_stack 的访问只需获取一次锁，减少了锁竞争概率，提升并发性能（尤其在多线程调度场景下）。

### 优化点2：信号挂起状态检查的原子性优化（基于 mod.rs ）

当前 has_pending_signal 方法通过获取 sig_info 的锁来检查信号状态（ sig_pending().has_pending() ），而 has_pending_signal_fast 直接读取标志位（ ProcessFlags::HAS_PENDING_SIGNAL ）。但标志位可能因更新延迟与实际信号状态不一致，导致 fast 方法不可靠。可通过 原子更新标志位 ，确保 fast 方法的准确性，同时减少锁的使用。

优化后代码：
~~~// 在信号状态更新处（如信号发送时）增加原子标志位更新
impl ProcessSignalInfo {
    pub fn set_sig_pending(&mut self, pending: bool) {
        // ... existing code ...
        // 原子更新标志位（假设 flags 支持原子操作）
        if pending {
            self.pcb.flags.insert(ProcessFlags::HAS_PENDING_SIGNAL);
        } else {
            self.pcb.flags.remove(ProcessFlags::HAS_PENDING_SIGNAL);
        }
    }
}

// 优化 has_pending_signal，减少锁持有时间
pub fn has_pending_signal(&self) -> bool {
    // 先检查快速标志位，避免频繁获取锁
    if self.has_pending_signal_fast() {
        return true;
    }
    // 标志位为 false 时，再通过锁确认
    let sig_info = self.sig_info_irqsave();
    let has_pending = sig_info.sig_pending().has_pending();
    drop(sig_info);
    return has_pending;
}

// 保持快速方法不变，但依赖原子更新的标志位
pub fn has_pending_signal_fast(&self) -> bool {
    self.flags.get().contains(ProcessFlags::HAS_PENDING_SIGNAL)
}
~~~
优化说明 ：通过在信号状态更新时原子更新标志位， has_pending_signal_fast 可安全用于快速路径（如调度器快速判断是否需要处理信号），而 has_pending_signal 仅在标志位不可靠时通过锁确认，降低了锁的使用频率。

以上优化针对进程管理中高频操作（上下文切换、信号处理）的性能瓶颈，通过减少锁竞争和优化标志位原子性，提升了系统并发效率。
