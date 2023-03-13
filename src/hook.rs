use std::{
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    thread::{self, JoinHandle},
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::MAPVK_VK_TO_VSC,
        WindowsAndMessaging::{HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_QUIT},
    },
};

use crate::{
    event::{KeyAction, KeyEvent, NativeEventOperation},
    safe_winapi::{System, UI},
};

type HookProc = unsafe extern "system" fn(code: i32, WPARAM, LPARAM) -> LRESULT;

// #[inline]
pub fn keyboard_hook_proc_inner(
    hook_handler: &HookHandler,
    n_code: i32,
    l_param: LPARAM,
) -> NativeEventOperation {
    if n_code < 0 {
        return NativeEventOperation::Dispatch;
    }
    let hook_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let event = match create_keyboard_event(&hook_struct) {
        None => return NativeEventOperation::Dispatch,
        Some(event) => event,
    };

    let native_operation = common_hook_proc_inner(hook_handler, event);
    if event.action == KeyAction::Release {
        return NativeEventOperation::Dispatch;
    }
    native_operation
}

// #[inline]
fn common_hook_proc_inner(hook_handler: &HookHandler, event: KeyEvent) -> NativeEventOperation {
    // if let KeyEvent { target, action } = event {
    //     match action {
    //         KeyAction::Press => target.assume_pressed(),
    //         KeyAction::Release => target.assume_released(),
    //     }
    // }
    hook_handler.send_event(event);
    NativeEventOperation::default()
}

fn create_keyboard_event(hook: &KBDLLHOOKSTRUCT) -> Option<KeyEvent> {
    if hook.dwExtraInfo != 0 {
        return None;
    }

    let action = if hook.flags.0 >> 7 == 0 {
        KeyAction::Press
    } else {
        KeyAction::Release
    };

    let vk_code = hook.vkCode;
    assert!(vk_code < i32::MAX as u32);

    let uscancode = UI::map_virtual_key(vk_code as i32, MAPVK_VK_TO_VSC);

    let mut lpkeystate: [u8; 256] = [0; 256];
    UI::get_keyboard_state(&mut lpkeystate);
    let lpkeystate = Some(&lpkeystate);

    let mut lpchar = 0u16;
    let hkl = UI::get_keyboard_layout(0);
    UI::to_ascii_ex(vk_code, uscancode, lpkeystate, &mut lpchar, 0, hkl);
    UI::get_key_state(UI::VK_SHIFTi32);

    Some(KeyEvent {
        target: lpchar,
        action,
    })
}

// Struct for installing and uninstalling the hook for keyboard event
struct Inner {
    keyboard_hook_handler: HHOOK,
    join_handle: JoinHandle<()>,
    event_sender: Sender<KeyEvent>,
    thread_id: u32,
}

impl Inner {
    fn spawn_thread(keyboard_hook_proc: HookProc, tx: Sender<(HHOOK, u32)>) -> JoinHandle<()> {
        thread::spawn(move || {
            // Set the hook proc for keyboard event
            // * thread_id set to 0 to associate the hook procedure with all existing threads.
            let keyboard_hook_handler =
                UI::set_windows_hook_exw(WH_KEYBOARD_LL, Some(keyboard_hook_proc), HINSTANCE(0), 0)
                    .expect("Failed to install keyboard hook.");

            // Get the currend thread id
            let thread_id = System::get_current_thread_id();

            tx.send((keyboard_hook_handler, thread_id)).unwrap();

            // Retrieve a message from the calling thread's queue
            UI::get_message_w();
        })
    }
    pub fn new(event_sender: Sender<KeyEvent>, keyboard_hook_proc: HookProc) -> Self {
        let (tx, rx) = mpsc::channel();

        let join_handle = Self::spawn_thread(keyboard_hook_proc, tx);

        let (keyboard_hook_handler, thread_id) = rx.recv().unwrap();

        Self {
            keyboard_hook_handler,
            join_handle,
            event_sender,
            thread_id,
        }
    }

    pub fn uninstall(self) {
        // Unhook the keyboard hook handler
        UI::unhook_windows_hook_ex(self.keyboard_hook_handler)
            .expect("Failed to uninstall keyboard hook.");

        // @TODO See why it's recommanded to do this ...
        UI::post_thread_message_w(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0)).unwrap();

        self.join_handle.join().unwrap();
    }
}

#[derive(Default)]
pub struct HookHandler {
    inner: Mutex<Option<Inner>>,
}

impl HookHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn install(&self, tx: Sender<KeyEvent>, keyboard_hook_proc: HookProc) {
        // let mut hook = self.inner.unwrap();
        let mut hook = self.inner.lock().unwrap();
        assert!(hook.is_none(), "No hooks are installed.");

        *hook = Some(Inner::new(tx, keyboard_hook_proc));
    }

    pub fn uninstall(&self) {
        self.inner
            .lock()
            .unwrap()
            .take()
            .expect("Hooks are not installed")
            .uninstall();
    }

    pub fn send_event(&self, event: KeyEvent) {
        self.inner
            .lock()
            .unwrap()
            .as_ref()
            .expect("Hooks are not installed")
            .event_sender
            .send(event)
            .unwrap();
    }
}
