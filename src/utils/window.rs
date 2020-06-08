use std::mem;
use std::ffi::CStr;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot,
    Module32First,
    Thread32First,
    MODULEENTRY32,
    THREADENTRY32,
    TH32CS_SNAPMODULE,
    TH32CS_SNAPTHREAD,
};
use winapi::um::handleapi::{
    CloseHandle,
};
use winapi::shared::windef::{
    HWND,
};
use winapi::um::winnt::{
    CHAR,
    HANDLE,
};
use winapi::shared::minwindef::{
    DWORD,
    BOOL,
    TRUE,
    FALSE,
    LPARAM,
};
use winapi::um::winuser::{
    GetWindowTextA,
    GetWindowThreadProcessId,
    GetForegroundWindow,
    EnumWindows,
};



enum Entry {

    Module,

    Thread,

}



pub struct HwndTarget {
    dw_pid: DWORD,
    dw_tid: DWORD,
    hwnd: HWND,
}



impl HwndTarget {

    pub fn from_pid(target_pid: DWORD) -> HwndTarget {

        let mut h_target = HwndTarget {
            dw_pid: target_pid,
            dw_tid: 0x0,
            hwnd: std::ptr::null_mut(),
        };

        unsafe { EnumWindows(Some(HwndTarget::enum_processes_callback), &mut h_target as *mut HwndTarget as LPARAM) };

        h_target

    }

    unsafe fn get_base_addr(&self, module_name: String, entry: Entry) -> DWORD {

        //Variable to hold the base address
        let mut base_addr: DWORD = 0x0;

        //Get info (snapshot) of heaps, modules and threads on the process specified by pid
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPTHREAD, self.dw_pid);

        match entry {

            Entry::Module => {

                //The module being looked at -- getting the base address for this
                let mut me32: MODULEENTRY32 = mem::zeroed();
                me32.dwSize = mem::size_of::<MODULEENTRY32>() as DWORD;

                //Find the module that matches module_name and get the address
                while Module32First(h_snapshot, &mut me32) == TRUE {

                    let me32_name = me32.szModule.into_iter().map(|&c| { c as u8 as char});
                    let me32_name: String = me32_name.into_iter().collect();

                    println!("Found module [{:?}] --- searching for module [{:?}]", me32_name, module_name);

                    if me32_name.eq(&module_name) {

                        base_addr = me32.modBaseAddr as DWORD;

                        break;

                    }
                    
                };

            },

            Entry::Thread => {

                //The thread being looked at -- getting the base address for this
                let mut te32: THREADENTRY32 = mem::zeroed();
                te32.dwSize = mem::size_of::<THREADENTRY32>() as DWORD;

                //Find the module that matches module_name and get the address
                while Thread32First(h_snapshot, &mut te32) == TRUE {

                    let me32_name = me32.szModule.into_iter().map(|&c| { c as u8 as char});
                    let me32_name: String = me32_name.into_iter().collect();

                    println!("Found module [{:?}] --- searching for module [{:?}]", me32_name, module_name);

                    if me32_name.eq(&module_name) {

                        base_addr = me32.modBaseAddr as DWORD;

                        break;

                    }
                    
                };

            },

        };

        CloseHandle(h_snapshot);

        base_addr

    }

    unsafe extern "system" fn enum_processes_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {

        //Process id of current hwnd
        let mut dw_pid: DWORD = 0x0;

        //LPARAM is a raw pointer ti a HwndTarget -- need to cast
        let h_target_ptr = l_param as *mut HwndTarget;

        //GetWindowThreadProcessId sets PID to pointer and returns TID
        //Raw pointers (*mut and *const) are the same as references -- for raw pointers, mut and const serve only as linting
        (*h_target_ptr).dw_tid = GetWindowThreadProcessId(hwnd, &mut dw_pid);

        HwndTarget::print_hwnd_info(hwnd, dw_pid);

        if (*h_target_ptr).dw_pid == dw_pid {

            (*h_target_ptr).hwnd = hwnd;

            println!("Found [HWND: {:?}] with --- [PID: {:?}] --- [TID: {:?}]", hwnd, dw_pid, (*h_target_ptr).dw_tid);

            //Sleep is just for debug -- remove later
            sleep(Duration::from_millis(2000));

            println!("Focused [HWND: {:?}]", GetForegroundWindow());

            //Found hwnd --- FALSE is an i32
            return FALSE

        }

        //Continue enumeration w/ EnumWindows --- TRUE is an i32
        TRUE

    }

    //TODO: Create a logger
    unsafe fn print_hwnd_info(hwnd: HWND, dw_pid: DWORD) {

        let mut hwnd_name: Vec<CHAR> = Vec::with_capacity(1024);

        match GetWindowTextA(hwnd, hwnd_name.as_mut_ptr(), 1024) {

            0 => (),

            _ => println!("[HWND: {:?}] --- [PID: {:?}] --- [Window: {:?}]", hwnd, dw_pid, CStr::from_ptr(hwnd_name.as_mut_ptr())),

        };

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