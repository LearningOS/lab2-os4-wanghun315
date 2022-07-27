//! Process management syscalls

use crate::config::*;
use crate::task::*;
use crate::timer::get_time_us;
use crate::mm::PageTable;


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let current_user_page_table = PageTable::from_token(current_user_token());
    let pa_ts = current_user_page_table.translate_va_to_pa((_ts as usize).into()).unwrap().0;
    unsafe {
        *(pa_ts as *mut TimeVal) = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    crate::task::mmap(_start, _len, _port)
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    crate::task::munmap(_start, _len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let current_user_page_table = PageTable::from_token(current_user_token());
    let pa_ti = current_user_page_table.translate_va_to_pa((ti as usize).into()).unwrap().0;
    // unsafe {println!("debug Kernel: info.status {:?}", *(pa_ti as *mut TaskInfo));}
    crate::task::get_task_info(pa_ti as *mut TaskInfo);
    // unsafe {println!("debug Kernel: info.status {:?}", *(pa_ti as *mut TaskInfo));}
    0
}

pub fn increase_syscall_time(syscall_number: usize){
    crate::task::increase_syscall_time(syscall_number);
}
