use std::mem;
use winapi::ctypes::{
    c_int 
};
use winapi::shared::windef::{
    HWND,
};
use winapi::shared::minwindef::{
    UINT,
    DWORD,
};
use winapi::um::winuser::{
    MapVirtualKeyW,
    KEYBDINPUT, 
    KEYEVENTF_SCANCODE,
    KEYEVENTF_KEYUP, 
    INPUT_KEYBOARD, 
    INPUT,
    SendInput,
};



//Refer to MSDN for more virtual key codes
pub enum VirtualKey {

    Backspace, //0x08

    Tab, //0x09

    Enter, //0x0D

    LShift, //0xA0

    LControl, //0xA2

    LAlt, //0xA4 --- TODO: double check docs

    CapsLock, //0x14

    Escape, //0x1B

    Space, //0x20

    PageUp, //0x21

    PageDown, //0x22

    End, //0x23

    Home, //0x24

    Left, //0x25

    Up, //0x26

    Right, //0x27

    Down, //0x28

    Insert, //0x2D

    Delete, //0x2E

    Zero, //0x30

    One, //0x31

    Two, //0x32

    Three, //0x33

    Four, //0x34

    Five, //0x35

    Six, //0x36

    Seven, //0x37

    Eight, //0x38

    Nine, //0x39

    A, //0x41

    B, //0x42

    C, //0x43

    D, //0x44

    E, //0x45

    F, //0x46

    G, //0x47

    H, //0x48

    I, //0x49

    J, //0x4A

    K, //0x4B

    L, //0x4C

    M, //0x4D

    N, //0x4E

    O, //0x4F

    P, //0x50

    Q, //0x51

    R, //0x52

    S, //0x53

    T, //0x54

    U, //0x55

    V, //0x56

    W, //0x57

    X, //0x58

    Y, //0x59

    Z, //0x5A

    F1, //0x70

    F2, //0x71

    F3, //0x72

    F4, //0x73

    F5, //0x74

    F6, //0x75

    F7, //0x76

    F8, //0x77

    F9, //0x78

    F10, //0x79

    F11, //0x7A

    F12, //0x7B

}




pub unsafe fn key_down(vk: VirtualKey) {

    send_key(vk, KEYEVENTF_SCANCODE);

}



pub unsafe fn key_up(vk: VirtualKey) {

    send_key(vk, KEYEVENTF_KEYUP);

}



unsafe fn send_key(vk: VirtualKey, flags: DWORD) {

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: mem::transmute_copy(&KEYBDINPUT {
            wVk: 0x0,
            wScan: get_scan_code(vk),
            dwFlags: flags,
            time: 0x0,
            dwExtraInfo: 0x0,
        }),
    };

    SendInput(1, &mut input, mem::size_of::<INPUT>() as c_int);

}



unsafe fn send_key_to_hwnd(hwnd: HWND) {
    //TODO: PostMessage or AttachThreadInput?
}



unsafe fn get_scan_code(vk: VirtualKey) -> u16 {

    let keycode: UINT = match vk {

        VirtualKey::Backspace => 0x08,

        VirtualKey::Tab => 0x09,

        VirtualKey::Enter => 0x0D,

        VirtualKey::LShift => 0xA0,

        VirtualKey::LControl => 0xA2,

        VirtualKey::LAlt => 0xA4, //0xA4 --- TODO: double check docs

        VirtualKey::CapsLock => 0x14,

        VirtualKey::Escape => 0x1B,

        VirtualKey::Space => 0x20,

        VirtualKey::PageUp => 0x21,

        VirtualKey::PageDown => 0x22,

        VirtualKey::End => 0x23,

        VirtualKey::Home => 0x24,

        VirtualKey::Left => 0x25,

        VirtualKey::Up => 0x26,

        VirtualKey::Right => 0x27,

        VirtualKey::Down => 0x28,

        VirtualKey::Insert => 0x2D,

        VirtualKey::Delete => 0x2E,

        VirtualKey::Zero => 0x30,

        VirtualKey::One => 0x31,

        VirtualKey::Two => 0x32,

        VirtualKey::Three => 0x33,

        VirtualKey::Four => 0x34,

        VirtualKey::Five => 0x35,

        VirtualKey::Six => 0x36,

        VirtualKey::Seven => 0x37,

        VirtualKey::Eight => 0x38,

        VirtualKey::Nine => 0x39,

        VirtualKey::A => 0x41,

        VirtualKey::B => 0x42,

        VirtualKey::C => 0x43,

        VirtualKey::D => 0x44,

        VirtualKey::E => 0x45,

        VirtualKey::F => 0x46,

        VirtualKey::G => 0x47,

        VirtualKey::H => 0x48,

        VirtualKey::I => 0x49,

        VirtualKey::J => 0x4A,

        VirtualKey::K => 0x4B,

        VirtualKey::L => 0x4C,

        VirtualKey::M => 0x4D,

        VirtualKey::N => 0x4E,

        VirtualKey::O => 0x4F,

        VirtualKey::P => 0x50,

        VirtualKey::Q => 0x51,

        VirtualKey::R => 0x52,

        VirtualKey::S => 0x53,

        VirtualKey::T => 0x54,

        VirtualKey::U => 0x55,

        VirtualKey::V => 0x56,

        VirtualKey::W => 0x57,

        VirtualKey::X => 0x58,

        VirtualKey::Y => 0x59,

        VirtualKey::Z => 0x5A,

        VirtualKey::F1 => 0x70,

        VirtualKey::F2 => 0x71,

        VirtualKey::F3 => 0x72,

        VirtualKey::F4 => 0x73,

        VirtualKey::F5 => 0x74,

        VirtualKey::F6 => 0x75,

        VirtualKey::F7 => 0x76,

        VirtualKey::F8 => 0x77,

        VirtualKey::F9 => 0x78,

        VirtualKey::F10 => 0x79,

        VirtualKey::F11 => 0x7A,

        VirtualKey::F12 => 0x7B,
       
    };

    return MapVirtualKeyW(keycode, 0x0) as u16;

}