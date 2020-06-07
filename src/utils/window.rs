use std::mem;
use winapi::shared::windef::{
    HWND,
};
use winapi::shared::minwindef::{
    DWORD,
};
use winapi::um::winuser::{
};



pub struct TargetHwnd {
    dw_pid: DWORD,
    dw_tid: DWORD,
    hwnd: HWND,
}



impl TargetHwnd {

    pub fn new(dw_pid: DWORD, dw_tid: DWORD, hwnd: HWND) -> TargetHwnd {
        return TargetHwnd {
            dw_pid,
            dw_tid,
            hwnd,
        };
    }

}



impl Default for TargetHwnd {

    fn default() -> TargetHwnd {
        return TargetHwnd {
            dw_pid: 0x0,
            dw_tid: 0x0,
            hwnd: std::ptr::null_mut(),
        }
    }

}