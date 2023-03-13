#![allow(unused)]

#[allow(non_snake_case)]
pub mod UI {

    use windows::Win32::{
        Foundation::{BOOL, HINSTANCE, HWND, LPARAM, WPARAM},
        UI::{
            Input::KeyboardAndMouse::{
                GetAsyncKeyState, GetKeyState, GetKeyboardLayout, GetKeyboardState, MapVirtualKeyW,
                ToAscii, ToAsciiEx, MAP_VIRTUAL_KEY_TYPE,
            },
            TextServices::HKL,
            WindowsAndMessaging::{
                GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK,
                HOOKPROC, WINDOWS_HOOK_ID,
            },
        },
    };

    #[allow(non_upper_case_globals)]
    pub const VK_SHIFTi32: i32 = 0x10;
    #[allow(non_upper_case_globals)]
    pub const VK_CONTROLi32: i32 = 0x11;
    #[allow(non_upper_case_globals)]
    pub const VK_ALTi32: i32 = 0x12;

    /// Determines whether a key is up or down at the time the function is called, and whether the key was pressed after a previous call to GetAsyncKeyState.
    ///
    /// **Parameters**
    ///
    /// The virtual-key code. For more information, see [Virtual Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).
    /// You can use left- and right-distinguishing constants to specify certain keys. See the Remarks section for further information.
    ///
    /// **Return value**
    ///
    /// If the function succeeds, the return value specifies whether the key was pressed since the last call to GetAsyncKeyState, and whether the key is currently up or down. If the most significant bit is set, the key is down, and if the least significant bit is set, the key was pressed after the previous call to GetAsyncKeyState. However, you should not rely on this last behavior; for more information, see the Remarks.
    ///
    /// The return value is zero for the following cases:
    /// - The current desktop is not the active desktop
    /// - The foreground thread belongs to another process and the desktop does not allow the hook or the journal record.
    pub fn get_async_key_state(key_id: i32) -> i16 {
        assert!(key_id > 0); // The key code can not be negative

        unsafe { GetAsyncKeyState(key_id) }
    }

    /// Retrieves the status of the specified virtual key. The status specifies whether the key is up, down, or toggled (on, off—alternating each time the key is pressed).
    ///
    /// **Parameters**
    ///
    /// A virtual key. If the desired virtual key is a letter or digit (A through Z, a through z, or 0 through 9),
    /// `key_id` must be set to the ASCII value of that character. For other keys, it must be a virtual-key code.
    /// If a non-English keyboard layout is used, virtual keys with values in the range ASCII A through Z and 0
    /// through 9 are used to specify most of the character keys.
    /// For example, for the German keyboard layout, the virtual key of value ASCII O (0x4F) refers to the "o" key,
    /// whereas VK_OEM_1 refers to the "o with umlaut" key.
    ///
    /// **Return value**
    ///
    /// The return value specifies the status of the specified virtual key, as follows:
    /// If the high-order bit is 1, the key is down; otherwise, it is up.
    /// If the low-order bit is 1, the key is toggled. A key, such as the CAPS LOCK key, is toggled if it is turned on.
    /// The key is off and untoggled if the low-order bit is 0. A toggle key's indicator light (if any) on the keyboard will be
    /// on when the key is toggled, and off when the key is untoggled.
    pub fn get_key_state(key_id: i32) -> i16 {
        assert!(key_id > 0); // The key code can not be negative

        unsafe { GetKeyState(key_id) }
    }

    /// `MAPVK_VK_TO_VSC`: Virtual Key to scan code
    ///
    /// `MAPVK_VSC_TO_VK`: Scan code to Virtual Key
    ///
    /// `MAPVK_VK_TO_CHAR`: Virtual Key to unshifted character value
    ///
    /// `MAPVK_VSC_TO_VK_EX`: Scan code to Virtual Key (distinguishing left and right keys)
    ///
    /// `MAPVK_VK_TO_VSC_EX`: Virtual Key to Scan code (distinguishing left and right keys)
    pub fn map_virtual_key(key_id: i32, map_type: MAP_VIRTUAL_KEY_TYPE) -> u32 {
        assert!(key_id > 0); // Should never be triggered as their is no negative Key code
                             // So why ? Whyyyy Microsoft it's i32 in GetAsyncKeyState

        unsafe { MapVirtualKeyW(key_id as u32, map_type) }
    }

    /// **Return value**
    ///
    /// 0: The specified virtual key has no translation for the current state of the keyboard.
    ///
    /// 1: One character was copied to the buffer.
    ///
    /// 2: Two characters were copied to the buffer. This usually happens when a dead-key character (accent or diacritic) stored in the keyboard layout cannot be composed with the specified virtual key to form a single character.
    pub fn to_ascii(
        uvirtkey: u32,
        uscancode: u32,
        lpkeystate: Option<&[u8; 256]>,
        lpchar: *mut u16,
        uflags: u32,
    ) -> i32 {
        unsafe { ToAscii(uvirtkey, uscancode, lpkeystate, lpchar, uflags) }
    }

    /// **Return value**
    ///
    /// 0: The specified virtual key has no translation for the current state of the keyboard.
    ///
    /// 1: One character was copied to the buffer.
    ///
    /// 2: Two characters were copied to the buffer. This usually happens when a dead-key character (accent or diacritic) stored in the keyboard layout cannot be composed with the specified virtual key to form a single character.
    pub fn to_ascii_ex(
        uvirtkey: u32,
        uscancode: u32,
        lpkeystate: Option<&[u8; 256]>,
        lpchar: *mut u16,
        uflags: u32,
        dwhkl: HKL,
    ) -> i32 {
        unsafe { ToAsciiEx(uvirtkey, uscancode, lpkeystate, lpchar, uflags, dwhkl) }
    }

    pub fn get_keyboard_state(key_state: &mut [u8; 256]) -> bool {
        unsafe { GetKeyboardState(key_state).as_bool() }
    }

    /// **Parameters**
    ///
    /// The identifier of the thread to query, or 0 for the current thread.
    ///
    /// **Return value**
    ///
    /// The return value is the input locale identifier for the thread. The low word contains a Language Identifier for the input language and the high word contains a device handle to the physical layout of the keyboard.
    pub fn get_keyboard_layout(id_thread: u32) -> HKL {
        unsafe { GetKeyboardLayout(id_thread) }
    }

    pub fn set_windows_hook_exw(
        id_hook: WINDOWS_HOOK_ID,
        hook_callback: HOOKPROC,
        h_mod: HINSTANCE,
        thread_id: u32,
    ) -> Result<HHOOK, windows::core::Error> {
        unsafe { SetWindowsHookExW(id_hook, hook_callback, h_mod, thread_id) }
    }

    pub fn get_message_w() {
        unsafe {
            GetMessageW(
                &mut std::mem::MaybeUninit::zeroed().assume_init(),
                HWND(0),
                0,
                0,
            )
        };
    }

    pub fn unhook_windows_hook_ex(ptr_hook: HHOOK) -> BOOL {
        unsafe { UnhookWindowsHookEx(ptr_hook) }
    }

    pub fn post_thread_message_w(
        thread_id: u32,
        msg: u32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> BOOL {
        unsafe { PostThreadMessageW(thread_id, msg, w_param, l_param) }
    }
}

#[allow(non_snake_case)]
pub mod System {
    use windows::Win32::System::Threading::GetCurrentThreadId;

    // use windows::Win32;
    pub fn get_current_thread_id() -> u32 {
        unsafe { GetCurrentThreadId() }
    }
}
