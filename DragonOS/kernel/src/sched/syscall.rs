use system_error::SystemError;

use crate::arch::cpu::current_cpu_id;
use crate::exception::InterruptArch;
use crate::process::ProcessManager;
use crate::sched::CurrentIrqArch;
use crate::sched::Scheduler;
use crate::syscall::Syscall;

use super::fair::CompletelyFairScheduler;
use super::{cpu_rq, schedule, SchedMode};

/// 修改！！！
impl Syscall {
    pub fn do_sched_yield() -> Result<usize, SystemError> {
        // 仅禁用中断至获取锁前
        let irq_guard = unsafe { CurrentIrqArch::save_and_disable_irq() };

        let pcb = ProcessManager::current_pcb();
        let rq = cpu_rq(pcb.sched_info().on_cpu().unwrap_or(current_cpu_id()).data() as usize);
        let (rq, guard) = rq.self_lock();  // 获取锁时中断已禁用
        drop(irq_guard);  // 提前恢复中断（锁已持有，后续操作无需中断禁用）

        // TODO: schedstat_inc(rq->yld_count);

        CompletelyFairScheduler::yield_task(rq);

        pcb.preempt_disable();

        drop(guard);

        pcb.preempt_enable(); // sched_preempt_enable_no_resched();

        schedule(SchedMode::SM_NONE);

        Ok(0)
    }
}
