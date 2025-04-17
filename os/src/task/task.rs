//! Types related to task management

use super::TaskContext;
use super::SYSCALL_MAX_ID;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The syscall count of the task
    pub task_syscall_count : [usize; SYSCALL_MAX_ID],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}