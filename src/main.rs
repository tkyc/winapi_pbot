use std::mem;
use winapi::ctypes::{ wchar_t };
use winapi::um::winnt::{ HANDLE, LPCWSTR, WCHAR };
use winapi::um::winuser::{ WNDENUMPROC, EnumWindows, FindWindowW, GetWindowThreadProcessId };
use winapi::shared::minwindef::{ MAX_PATH, DWORD, LPARAM, BOOL, TRUE, FALSE };
use winapi::shared::windef::{ HWND };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{ CreateToolhelp32Snapshot, PROCESSENTRY32, Process32Next, TH32CS_SNAPPROCESS };
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::ReadProcessMemory;



const PROCESS_WM_READ: u32 = 0x010;

pub struct DesignatedHwnd {
    dw_proc_id: DWORD, //u32
    hwnd: HWND, //*mut HWND__
}

unsafe extern "system" fn get_window_by_proc(id: DWORD) -> HWND {
    FindWindowW(
        0x0 as *const WCHAR as LPCWSTR, 
        0x0 as *const WCHAR as LPCWSTR,
    )
}

//BOOL -> c_int -> i32 
unsafe extern "system" fn enum_wnd_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {

    //Process id of current hwnd
    let mut dw_proc_id: DWORD = 0x0;

    let designated_hwnd_ptr = l_param as *mut DesignatedHwnd;

    //let dw_proc_id_ptr: *mut DWORD = &mut dw_proc_id; //--- Raw pointer

    //Gets the current process id of hwnd
    //GetWindowThreadProcessId(hwnd, dw_proc_id_ptr);
    GetWindowThreadProcessId(hwnd, &mut dw_proc_id);

    if (*designated_hwnd_ptr).dw_proc_id == dw_proc_id {

        (*designated_hwnd_ptr).hwnd = hwnd;

        return FALSE
    }

    //Continue
    TRUE

}

//TODO: Free memory
fn main() {

    let mut designated_hwnd: DesignatedHwnd = unsafe {
        DesignatedHwnd {
            dw_proc_id: 0x0 as u32,
            hwnd: FindWindowW(
                0x0 as *const WCHAR as LPCWSTR,
                0x0 as *const WCHAR as LPCWSTR,
            ),
        }
    };

    //let designated_hwnd_ptr: *mut DesignatedHwnd = &mut designated_hwnd; //--- Raw pointer

    let hwnd: HWND = unsafe {
        FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        )
    };

    unsafe { enum_wnd_proc(hwnd, &mut designated_hwnd as *mut DesignatedHwnd as LPARAM) };

    let snapshots: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if snapshots == INVALID_HANDLE_VALUE {
        
        println!("Invalid handle value...");
        
        println!("Handle value: {:?}", snapshots);
        
        return
        
    }

    let pe32: *mut PROCESSENTRY32 = &mut PROCESSENTRY32 {
        dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; MAX_PATH],
    };

    //let pe32_ptr: *mut PROCESSENTRY32 = &mut pe32; //--- Raw pointer
    
    while unsafe { Process32Next(snapshots, pe32) == 1 } {

        let mut pe32_name: String = String::new();
        
        //TODO: Data parallelization (chunks_exact)
        for c in unsafe { (*pe32).szExeFile.iter() } {

            match *c {

                0 => break,

                _ => pe32_name.push(*c as u8 as char),

            }
                
        }

        println!("{:?} --- {:?}", pe32_name, unsafe { (*pe32).th32ProcessID });

    }


   //TODO: https://codingvision.net/security/c-read-write-another-process-memory 
   //TODO: https://www.12ghast.com/code/c-process-name-to-pid/
   //TODO: https://users.rust-lang.org/t/comparing-a-string-to-an-array-of-i8/5120/4
   //let process_handle: HANDLE = OpenProcess(PROCESS_WM_READ, 0, dwProcessId: DWORD);
   //https://stackoverflow.com/questions/12099957/how-to-send-a-keystroke-to-an-other-process-ex-a-notepad
}