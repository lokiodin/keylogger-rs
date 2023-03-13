/// Indicates key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub target: u16,
    pub action: KeyAction,
}

/// A key input action.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum KeyAction {
    Press,
    Release,
}

/// Indicates whether to pass the generated event to the next program or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeEventOperation {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Dispatch,
}

impl Default for &NativeEventOperation {
    fn default() -> Self {
        &NativeEventOperation::Dispatch
    }
}

impl Default for NativeEventOperation {
    fn default() -> Self {
        *<&NativeEventOperation>::default()
    }
}
