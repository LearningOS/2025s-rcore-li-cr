//! Implementation of [`TaskContext`]
use super::SYSCALL_MAX_ID;

#[derive(Copy, Clone)]
#[repr(C)]
/// task context structure containing some registers
pub struct TaskContext {
    /// Ret position after task switching
    ra: usize,
    /// Stack pointer
    sp: usize,
    /// s0-11 register, callee saved
    s: [usize; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
/// task context structure containing some informations
pub struct TaskInformation{
    /// The syscall count of the task
    pub task_syscall_count : [usize; SYSCALL_MAX_ID],
    /// User_State_time, Kernel_State_time
    pub task_time: [usize; 2],
    /// Time_flag
    pub task_time_flag: [usize; 2],
}

impl TaskContext {
    /// Create a new empty task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    /// Create a new task context with a trap return addr and a kernel stack pointer
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}

impl TaskInformation {
    /// new task information
    pub fn new() -> Self {
        Self {
            task_syscall_count: [0; SYSCALL_MAX_ID],
            task_time: [0; 2],
            task_time_flag: [0; 2],
        }
    }
    /// add syscall count
    /// 0: user time, 1: kernel time
    /// 00 01 10 11
    pub fn timer(&mut self, flag: usize) {
        // 0: user time, 1: kernel time
        
        let fl = &mut self.task_time_flag[0];
        
        let f: usize = crate::timer::get_time();
        
        if flag <= 1{
            self.task_time[flag] += f - *fl;
        }
        *fl = f;
    }
}