mod utils;

use utils::keyboard;
use utils::keyboard::VirtualKey;
use utils::window;

use std::mem;
use std::ffi::CStr;
use std::thread::sleep;
use std::time::Duration;
use winapi;
use winapi::ctypes::{ wchar_t, c_int };
use winapi::um::winnt::{ HANDLE, LPCWSTR, WCHAR, CHAR };
use winapi::um::winuser::{ WNDENUMPROC, EnumWindows, FindWindowW, GetWindowThreadProcessId,
                           PostThreadMessageW, PostMessageW, SendMessageW, SetForegroundWindow, WM_KEYDOWN, VK_LEFT, WM_KEYUP, INPUT, INPUT_u, INPUT_KEYBOARD,
                           KEYBDINPUT, PostMessageA, PostThreadMessageA, SendMessageA, GUITHREADINFO, GetGUIThreadInfo, GetWindowTextA, FindWindowExW, SendInput, SetFocus,
                           SetActiveWindow, ShowWindow, FindWindowA, FindWindowExA, GetForegroundWindow, KEYEVENTF_UNICODE, KEYEVENTF_SCANCODE, KEYEVENTF_KEYUP,
                           LPINPUT, MapVirtualKeyW, AttachThreadInput, VK_RETURN, };
use winapi::shared::minwindef::{ MAX_PATH, DWORD, LPARAM, BOOL, TRUE, FALSE, WPARAM };
use winapi::shared::windef::{ HWND, RECT };
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{ CreateToolhelp32Snapshot, PROCESSENTRY32, Process32Next, TH32CS_SNAPPROCESS };
use winapi::um::processthreadsapi::{ GetCurrentThreadId };
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

        println!("Found HWND {:?} with --- PID: {:?} --- TID: {:?}", hwnd, dw_proc_id, (*target_wnd_ptr).dw_thread_id);

        sleep(Duration::from_millis(1000));

        println!("Focused HWND is {:?}", GetForegroundWindow());

        //Found hwnd --- FALSE -> i32
        return FALSE

    }

    //println!("HWND not found: {:?}", dw_proc_id);

    //Continue enumeration w/ EnumWindows --- TRUE -> i32
    TRUE

}

pub unsafe extern "system" fn send_key_to(window: &TargetWindow) {

    //let pkmn = std::ffi::CString::new("Untitled - Notepad").unwrap();
    let pkmn = std::ffi::CString::new("PokeMMO").unwrap();

    //FindWindowEx maybe?
    let wnd: HWND = FindWindowW(std::ptr::null_mut(), pkmn.as_ptr() as *const u16);
    let wnd0: HWND = FindWindowExA( std::ptr::null_mut(), std::ptr::null_mut(), pkmn.as_ptr(), std::ptr::null());
    sleep(std::time::Duration::from_millis(2000));
    let wnd1: HWND = GetForegroundWindow();
    println!("Got hwnd----------------");
    sleep(std::time::Duration::from_millis(3000));

    println!("First: {:?}", wnd);
    println!("Second: {:?}", wnd0);
    println!("Third: {:?}", wnd1);

    println!("window.hwnd: {:?}", window.hwnd);

    //sleep(Duration::from_millis(2000));
    //Set focus to window
    //ShowWindow(window.hwnd, 1);
    //SetFocus(window.hwnd);
    //SetActiveWindow(window.hwnd);
    //SetForegroundWindow(window.hwnd);

    let mut wnd1name: Vec<CHAR> = Vec::with_capacity(1024);

    match GetWindowTextA(wnd1, wnd1name.as_mut_ptr(), 1024) {

        0 => {

            println!("wnd1 was not found...");

            return

        },

        _ => println!("Found wnd1 with the following name: {:?}", CStr::from_ptr(wnd1name.as_mut_ptr())),

    };


    //PostThreadMessageA(tid, WM_KEYDOWN, 0x41, 0x1);
    //PostMessageA(wnd1, WM_KEYDOWN, 0x41, 0);
    //PostMessageA(wnd1, WM_KEYUP, 0x41, 0);

    //PostMessageA(wnd1, WM_KEYDOWN, 0x25, 0);

    //let scan_key: u32 = MapVirtualKeyW(0x41, 0x0);
    //let key: u32 = 0x41;

    //PostMessageA(wnd1, WM_KEYDOWN, 0x41 as usize, 0);
    //println!("Sending key for PID: {:?} --- TID: {:?}", window.dw_proc_id, window.dw_thread_id);

    let dw_target_tid: DWORD = GetWindowThreadProcessId(wnd1, std::ptr::null_mut());
    let dw_calling_tid: DWORD = GetCurrentThreadId();

    let is_attached: BOOL = AttachThreadInput(dw_calling_tid, dw_target_tid, -1);

    //keyboard::key_down(VirtualKey::A);
    //sleep(Duration::from_millis(2000));
    //keyboard::key_up(VirtualKey::A);

    println!("IS_ATTACHED: {:?}", is_attached);

    //SendMessageA(wnd1, WM_KEYDOWN, 0x41, 0x1);
    //SendMessageW(wnd1, WM_KEYDOWN, 0x41, 0x1);
    //PostThreadMessageA(dw_target_tid, WM_KEYDOWN, 0x41, 0x1);

}

//TODO: Free memory
fn main() {

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

    //Target is Zulu Platform x64 Architecture
    let h_target = window::HwndTarget::from_pid(7988);

    unsafe { h_target.get_base_addr_thread_entry() };


    //unsafe { send_key_to(&target) };

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


   //https://github.com/enigo-rs/enigo
   //https://github.com/enigo-rs/enigo/blob/master/src/win/win_impl.rs
   //https://stackoverflow.com/questions/9503027/pinvoke-setfocus-to-a-particular-control/9547099#9547099
   //https://www.youtube.com/watch?v=Mm3ZK3uAeuo
   //https://forum.cheatengine.org/viewtopic.php?t=582604
   //https://stackoverflow.com/questions/11147846/how-to-retrieve-starting-address-of-a-thread-in-windows
   //https://stackoverflow.com/questions/32297431/getting-the-tib-teb-of-a-thread-by-its-thread-handle-2015
}