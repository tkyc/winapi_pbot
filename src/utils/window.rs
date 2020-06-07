use std::mem;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::windef::{
    HWND,
};
use winapi::shared::minwindef::{
    DWORD,
    BOOL,
    TRUE,
    FALSE,
    LPARAM,
};
use winapi::um::winuser::{
    GetWindowThreadProcessId,
    GetForegroundWindow,
};



pub struct HwndTarget {
    dw_pid: DWORD,
    dw_tid: DWORD,
    hwnd: HWND,
}



impl HwndTarget {

    pub fn from_pid(target_pid: DWORD) -> HwndTarget {

        let mut h_target: HwndTarget = Default::default();

        unsafe { };

        h_target
    }

    unsafe fn enum_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {

        //Process id of current hwnd
        let mut dw_pid: DWORD = 0x0;

        //LPARAM is a raw pointer ti a HwndTarget -- need to cast
        let h_target_ptr = l_param as *mut HwndTarget;

        //GetWindowThreadProcessId sets PID to pointer and returns TID
        //Raw pointers (*mut and *const) are the same as references -- for raw pointers, mut and const serve only as linting
        (*h_target_ptr).dw_tid = GetWindowThreadProcessId(hwnd, &mut dw_pid);

        if (*h_target_ptr).dw_pid == dw_pid {

            (*h_target_ptr).hwnd = hwnd;

            println!("Found HWND {:?} with --- PID: {:?} --- TID: {:?}", hwnd, dw_pid, (*h_target_ptr).dw_tid);

            //TODO: sleep is just for debug -- remove later
            sleep(Duration::from_millis(1000));

            println!("Focused HWND is {:?}", GetForegroundWindow());

            //Found hwnd --- FALSE is an i32
            return FALSE

        }

        //Continue enumeration w/ EnumWindows --- TRUE is an i32
        TRUE

    }

}



impl Default for HwndTarget {

    fn default() -> HwndTarget {
        HwndTarget {
            dw_pid: 0x0,
            dw_tid: 0x0,
            hwnd: std::ptr::null_mut(),
        }
    }

}