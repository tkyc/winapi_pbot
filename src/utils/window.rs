use std::mem;
use std::ffi::CStr;
use std::thread::sleep;
use std::time::Duration;
use ntapi::ntpebteb::TEB;
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::libloaderapi::GetModuleHandleA;
use ntapi::ntpsapi::{
    NtQueryInformationThread,
    ThreadQuerySetWin32StartAddress,
    THREAD_BASIC_INFORMATION,
    THREADINFOCLASS,
};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot,
    Module32First,
    Thread32First,
    Thread32Next,
    MODULEENTRY32,
    THREADENTRY32,
    TH32CS_SNAPMODULE,
    TH32CS_SNAPTHREAD,
};
use winapi::um::psapi::{
    GetModuleInformation,
    MODULEINFO,
};
use winapi::um::processthreadsapi::{
    OpenProcess,
    OpenThread,
};
use winapi::um::handleapi::{
    CloseHandle,
};
use winapi::shared::ntdef::{
    NTSTATUS,
    PVOID,
};
use winapi::shared::windef::{
    HWND,
};
use winapi::um::winnt::{
    CHAR,
    HANDLE,
    NT_TIB,
    LPCSTR,
    LPCWSTR,
    PROCESS_ALL_ACCESS,
    THREAD_GET_CONTEXT,
    THREAD_QUERY_INFORMATION,
};
use winapi::shared::minwindef::{
    DWORD,
    BOOL,
    TRUE,
    FALSE,
    LPARAM,
    LPCVOID,
    LPVOID,
};
use winapi::um::winuser::{
    GetWindowTextA,
    GetWindowThreadProcessId,
    GetForegroundWindow,
    EnumWindows,
};



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

    unsafe fn get_base_addr_module_entry(&self, module_name: &str) -> DWORD {

        //Variable to hold the base address
        let mut base_addr: DWORD = 0x0;

        //Get info (snapshot) of modules on the process specified by pid
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, self.dw_pid);

        //The module being looked at -- getting the base address for this
        let mut me32: MODULEENTRY32 = mem::zeroed();
        me32.dwSize = mem::size_of::<MODULEENTRY32>() as DWORD;

        //Find the module that matches module_name and get the address
        while Module32First(h_snapshot, &mut me32) == TRUE {

            match CStr::from_ptr(me32.szModule.as_mut_ptr()).to_str() {

                Ok(me32_name) => {

                    println!("Found module [{:?}] --- searching for module [{:?}]", me32_name, module_name);

                    if me32_name.eq(module_name) {

                        base_addr = me32.modBaseAddr as DWORD;

                        break;

                    }

                },

                Err(_e) => (),

            };
    
        };

        CloseHandle(h_snapshot);

        base_addr

    }

    pub unsafe fn get_base_addr_thread_entry(&self) -> DWORD {

        //Variable to hold the base address
        let mut base_addr: DWORD = 0x0;

        //Handle to process -- handles are abstract references/pointers to memory or files
        let h_proc: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, FALSE, self.dw_pid);

        //The thread being looked at -- getting the base address for this
        let mut te32: THREADENTRY32 = mem::zeroed();
        te32.dwSize = mem::size_of::<THREADENTRY32>() as DWORD;

        //Get info (snapshot) of threads on the process specified by pid
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0x0);

        //Enumeration of threads start at index 1
        while Thread32Next(h_snapshot, &mut te32) == TRUE {

            if te32.th32OwnerProcessID == self.dw_pid {
                
                //Handle to thread -- handles are abstract references/pointers to memory or files
                let h_thread: HANDLE = OpenThread(THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION, FALSE, te32.th32ThreadID);

                let mut tbi: THREAD_BASIC_INFORMATION = mem::zeroed();

                //Thread information block
                let mut tib: NT_TIB = mem::zeroed();

                //Getting thread information
                let nt_status: NTSTATUS = NtQueryInformationThread(h_thread,
                                                                   0x0 as THREADINFOCLASS,
                                                                   &mut tbi as *mut THREAD_BASIC_INFORMATION as PVOID,
                                                                   mem::size_of::<THREAD_BASIC_INFORMATION>() as u32,
                                                                   std::ptr::null_mut());

                ReadProcessMemory(h_proc,
                                  //TebBaseAddress is a *mut
                                  tbi.TebBaseAddress as LPCVOID,
                                  &mut tib as *mut NT_TIB as LPVOID,
                                  mem::size_of::<NT_TIB>(),
                                  std::ptr::null_mut());

                println!("[PID: {:?}] --- [Thread handle: {:?}] --- [TID: {:?}] --- [Page address: {:?}] --- [NTSTATUS: {:?}]",
                         te32.th32OwnerProcessID, h_thread, te32.th32ThreadID, tib.StackBase, nt_status == STATUS_SUCCESS);

                let mut module_info: MODULEINFO = mem::zeroed();

                GetModuleInformation(h_proc, GetModuleHandleW(String::from("kernel32.dll").as_mut_ptr() as LPCWSTR), &mut module_info, mem::size_of::<MODULEINFO>() as DWORD);
                
                let mut buffer: Vec<DWORD> = Vec::with_capacity(4096);
                
                //TODO: Use FFI to C to resovle
                let mut k32dll = String::from("kernel32.dll");

                println!("MODULE_HANDLE: {:?}", GetModuleHandleA(&mut k32dll as *mut String as LPCSTR));

                CloseHandle(h_thread);

            }

        };

        CloseHandle(h_snapshot);

        CloseHandle(h_proc);

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