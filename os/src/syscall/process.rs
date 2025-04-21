//! Process management syscalls


use core::slice;


use crate::mm::{translated_byte_buffer, MapPermission, PTEFlags};
use crate::task::{change_program_brk, current_user_token, delete_framed_area, exit_current_and_run_next, get_syscount, insert_framed_area, suspend_current_and_run_next};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let token = current_user_token();
    // 先检查用户指针可写、可用户访问
    let siz = core::mem::size_of::<TimeVal>();
    let start = _ts as usize;
    let end   = start + siz - 1;
    if !crate::mm::addr_flag(token, start, PTEFlags::W | PTEFlags::U)
       || !crate::mm::addr_flag(token, end,   PTEFlags::W | PTEFlags::U)
    {
        return -1;
    }

    // 准备要写回用户的结构
    let now = get_time_us();
    let tmp = TimeVal {
        sec:  now / 1_000_000,
        usec: now % 1_000_000,
    };

    let src: &[u8] = unsafe {
        slice::from_raw_parts((&tmp as *const TimeVal) as *const u8, siz)
    };
    // 4. 拿到所有需要的用户页
    let mut pages = translated_byte_buffer(token, _ts as *const u8, siz);
    // 5. 分段拷贝
    let mut copied = 0;
    for (_i, page) in pages.iter_mut().enumerate() {
        // 本页能写多少字节
        let write_len = core::cmp::min(page.len(), siz - copied);
        // 拷贝
        page[..write_len]
            .copy_from_slice(&src[copied..copied + write_len]);
        copied += write_len;
        if copied >= siz {
            break;
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// 这个系统调用有三种功能，根据 trace_request 的值不同，执行不同的操作：

// 如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值。此时应忽略 data 参数。返回值为 id 地址处的值。

// 如果 trace_request 为 1，则 id 应被视作 *mut u8 ，表示写入 data （作为 u8，即只考虑最低位的一个字节）到该用户程序 id 地址处。返回值应为0。

// 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数，返回值为这个调用次数。本次调用也计入统计 。

// 否则，忽略其他参数，返回值为 -1。
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    match _trace_request {
        0 => {
            println!("{:x}", _id);
            if crate::mm::addr_flag(current_user_token(), _id, PTEFlags::R | PTEFlags::U){
                if let Some(buffer) = translated_byte_buffer(current_user_token(), _id as *const u8, 1).get(0).unwrap().get(0) {
                    println!("{:x}", buffer);
                    return *buffer as isize;
                }
            }
        }
        1 => {
            
            if crate::mm::addr_flag(current_user_token(), _id, PTEFlags::W | PTEFlags::U){
                if let Some(buffer) = translated_byte_buffer(current_user_token(), _id as *const u8, 1).get_mut(0).unwrap().get_mut(0) {
                    *buffer = (_data & 0xFF) as u8;
                    return 0;
                }
            }
        }
        2 =>{
            return get_syscount(_id) as isize;
        }
        _ => {
            return -1;
        }
    }
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    
    insert_framed_area(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    delete_framed_area(_start,_len, MapPermission::U)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
