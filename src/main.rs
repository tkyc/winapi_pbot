use std::mem;
use std::ffi::CStr;
use std::thread::sleep;
use std::time::Duration;
use winapi;
use winapi::ctypes::{ wchar_t };
use winapi::um::winnt::{ HANDLE, LPCWSTR, WCHAR, CHAR };
use winapi::um::winuser::{ WNDENUMPROC, EnumWindows, FindWindowW, GetWindowThreadProcessId,
                           PostThreadMessageW, PostMessageW, SendMessageW, SetForegroundWindow, WM_KEYDOWN, VK_LEFT, WM_KEYUP, INPUT, INPUT_u, INPUT_KEYBOARD,
                           KEYBDINPUT, PostMessageA, PostThreadMessageA, SendMessageA, GUITHREADINFO, GetGUIThreadInfo, GetWindowTextA, FindWindowExW };
use winapi::shared::minwindef::{ MAX_PATH, DWORD, LPARAM, BOOL, TRUE, FALSE };
use winapi::shared::windef::{ HWND, RECT };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{ CreateToolhelp32Snapshot, PROCESSENTRY32, Process32Next, TH32CS_SNAPPROCESS };
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::memoryapi::ReadProcessMemory;



const PROCESS_WM_READ: u32 = 0x010;

pub struct TargetWindow {
    dw_proc_id: DWORD, //u32
    dw_thread_id: DWORD, //u32
    hwnd: HWND, //*mut HWND__
}

unsafe extern "system" fn get_window_by_proc(id: DWORD) -> HWND {
    FindWindowW(
        0x0 as *const WCHAR as LPCWSTR, 
        0x0 as *const WCHAR as LPCWSTR,
    )
}

unsafe extern "system" fn enum_wnd_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {

    //Process id of current hwnd
    let mut dw_proc_id: DWORD = 0x0;

    let target_wnd_ptr = l_param as *mut TargetWindow;

    //let dw_proc_id_ptr: *mut DWORD = &mut dw_proc_id; //--- Raw pointer

    //Gets the current process id of hwnd --- LPDWORD -> DWORD
    //GetWindowThreadProcessId(hwnd, dw_proc_id_ptr);

    //Gets the current process id of hwnd ---- LPDWORD -> DWORD
    (*target_wnd_ptr).dw_thread_id = GetWindowThreadProcessId(hwnd, &mut dw_proc_id);

    if (*target_wnd_ptr).dw_proc_id == dw_proc_id {

        (*target_wnd_ptr).hwnd = hwnd;

        println!("HWND found with PID: {:?} --- TID: {:?}", dw_proc_id, (*target_wnd_ptr).dw_thread_id);

        //Found hwnd --- FALSE -> i32
        return FALSE

    }

    //println!("HWND not found: {:?}", dw_proc_id);

    //Continue enumeration w/ EnumWindows --- TRUE -> i32
    TRUE

}

pub unsafe extern "system" fn send_key_to(window: &TargetWindow) {

    //Set focus to window
    SetForegroundWindow(window.hwnd);

    //Account for window focusing delay
    //sleep(Duration::from_millis(1000));

    //Checking window name
    let mut window_name_buffer: Vec<CHAR> = Vec::with_capacity(1024);

    match GetWindowTextA(window.hwnd, window_name_buffer.as_mut_ptr(), 1024) {

        0 => {

            println!("Window was not found...");

            return

        },

        _ => println!("Found window with the following name: {:?}", CStr::from_ptr(window_name_buffer.as_mut_ptr())),

    };

    let hwnd: HWND = FindWindowW(
        0x0 as *const WCHAR as LPCWSTR,
        0x0 as *const WCHAR as LPCWSTR,
    );

    //Get thread that owns the hwnd to the window
    let mut thread_info: GUITHREADINFO = GUITHREADINFO {
        cbSize: 1024,
        flags: 0x0,
        hwndActive: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        hwndFocus: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        hwndCapture: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        hwndMenuOwner: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        hwndMoveSize: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        hwndCaret: FindWindowW(
            0x0 as *const WCHAR as LPCWSTR,
            0x0 as *const WCHAR as LPCWSTR,
        ),
        rcCaret: RECT {
            left: 0x0,
            top: 0x0,
            right: 0x0,
            bottom: 0x0,
        }
    };

    match GetGUIThreadInfo(window.dw_thread_id, &mut thread_info as *mut GUITHREADINFO) {

        0 => println!("Thread info not found..."),

        _ => println!("Thread info found -- focusing on hwnd: {:?}", thread_info.hwndFocus),

    };

    //PostThreadMessageA(window.dw_thread_id, WM_KEYDOWN, 0x31, 0x2);
    PostMessageA(window.hwnd, WM_KEYDOWN, 0x31, 0);
    PostMessageA(window.hwnd, WM_KEYUP, 0x31, 0);
    //SendMessageA(window.hwnd, WM_KEYDOWN, 0x31, 0x2);
    //println!("Sending key for PID: {:?} --- TID: {:?}", window.dw_proc_id, window.dw_thread_id);

}

//TODO: Free memory
fn main() {

    const TARGET_DW_PROC_ID: DWORD = 0xD3C;

    let mut target: TargetWindow = unsafe {
        TargetWindow {
            dw_proc_id: 3776,
            dw_thread_id: 0x0,
            hwnd: FindWindowW(
                0x0 as *const WCHAR as LPCWSTR,
                0x0 as *const WCHAR as LPCWSTR,
            ),
        }
    };

    //let target_hwnd: *mut TargetWindow = &mut designated_hwnd; //--- Raw pointer
    let window_name: Vec<char> = "Untitled - Notepad".chars().collect();

    let hwnd: HWND = unsafe {
        FindWindowExW(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            window_name.as_ptr() as LPCWSTR,
            std::ptr::null(),
        )
    };

    println!("WINDOW -- {:?}", hwnd);

    unsafe { EnumWindows(Some(enum_wnd_proc), &mut target as *mut TargetWindow as LPARAM) };

    unsafe { send_key_to(&target) };

    //loop {

    //    unsafe { send_key_to(&target) };

    //}
    //let snapshots: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    //if snapshots == INVALID_HANDLE_VALUE {
    //    
    //    println!("Invalid handle value...");
    //    
    //    println!("Handle value: {:?}", snapshots);
    //    
    //    return
    //    
    //}

    //let pe32: *mut PROCESSENTRY32 = &mut PROCESSENTRY32 {
    //    dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
    //    cntUsage: 0,
    //    th32ProcessID: 0,
    //    th32DefaultHeapID: 0,
    //    th32ModuleID: 0,
    //    cntThreads: 0,
    //    th32ParentProcessID: 0,
    //    pcPriClassBase: 0,
    //    dwFlags: 0,
    //    szExeFile: [0; MAX_PATH],
    //};

    ////let pe32_ptr: *mut PROCESSENTRY32 = &mut pe32; //--- Raw pointer
    
    //while unsafe { Process32Next(snapshots, pe32) == 1 } {

    //    let mut pe32_name: String = String::new();
    //    
    //    //TODO: Data parallelization (chunks_exact)
    //    for c in unsafe { (*pe32).szExeFile.iter() } {

    //        match *c {

    //            0 => break,

    //            _ => pe32_name.push(*c as u8 as char),

    //        }
    //            
    //    }

    //    println!("{:?} --- {:?}", pe32_name, unsafe { (*pe32).th32ProcessID });

    //}


   //TODO: https://codingvision.net/security/c-read-write-another-process-memory 
   //TODO: https://www.12ghast.com/code/c-process-name-to-pid/
   //TODO: https://users.rust-lang.org/t/comparing-a-string-to-an-array-of-i8/5120/4
   //let process_handle: HANDLE = OpenProcess(PROCESS_WM_READ, 0, dwProcessId: DWORD);
   //https://stackoverflow.com/questions/12099957/how-to-send-a-keystroke-to-an-other-process-ex-a-notepad
   //https://stackoverflow.com/questions/20162359/c-best-way-to-get-window-handle-of-the-only-window-from-a-process-by-process
   //https://github.com/retep998/winapi-rs/issues/746
   //https://dailyoverdose.wordpress.com/2009/10/28/postmessage-and-sendmessage/
   //https://stackoverflow.com/questions/11890972/simulating-key-press-with-postmessage-only-works-in-some-applications
}