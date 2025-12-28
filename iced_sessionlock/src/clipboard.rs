use iced_core::Clipboard;
use iced_core::clipboard::Kind;
use sessionlockev::WindowWrapper;

pub struct SessionLockClipboard {
    #[cfg(feature = "clipboard")]
    state: State,
}

#[cfg(feature = "clipboard")]
enum State {
    Connected(window_clipboard::Clipboard),
    Unavailable,
}

impl SessionLockClipboard {
    /// Creates a new [`Clipboard`] for the given window.
    #[cfg(feature = "clipboard")]
    pub fn connect(window: &WindowWrapper) -> Self {
        #[allow(unsafe_code)]
        let state = unsafe { window_clipboard::Clipboard::connect(window) }
            .ok()
            .map(State::Connected)
            .unwrap_or(State::Unavailable);

        Self { state }
    }

    #[cfg(not(feature = "clipboard"))]
    pub fn connect(_window: &WindowWrapper) -> Self {
        Self {}
    }

    /// Creates a new [`Clipboard`] that isn't associated with a window.
    /// This clipboard will never contain a copied value.
    #[allow(unused)]
    pub fn unconnected() -> Self {
        #[cfg(feature = "clipboard")]
        return Self {
            state: State::Unavailable,
        };
        #[cfg(not(feature = "clipboard"))]
        return Self {};
    }

    /// Reads the current content of the [`Clipboard`] as text.
    #[cfg(feature = "clipboard")]
    pub fn read(&self, kind: Kind) -> Option<String> {
        match &self.state {
            State::Connected(clipboard) => match kind {
                Kind::Standard => clipboard.read().ok(),
                Kind::Primary => clipboard.read_primary().and_then(Result::ok),
            },
            State::Unavailable => None,
        }
    }

    #[cfg(not(feature = "clipboard"))]
    pub fn read(&self, _kind: Kind) -> Option<String> {
        None
    }

    /// Writes the given text contents to the [`Clipboard`].
    #[cfg(feature = "clipboard")]
    pub fn write(&mut self, kind: Kind, contents: String) {
        match &mut self.state {
            State::Connected(clipboard) => {
                let result = match kind {
                    Kind::Standard => clipboard.write(contents),
                    Kind::Primary => clipboard.write_primary(contents).unwrap_or(Ok(())),
                };

                match result {
                    Ok(()) => {}
                    Err(error) => {
                        log::warn!("error writing to clipboard: {error}");
                    }
                }
            }
            State::Unavailable => {}
        }
    }

    #[cfg(not(feature = "clipboard"))]
    pub fn write(&mut self, _kind: Kind, _contents: String) {}
}

impl Clipboard for SessionLockClipboard {
    fn read(&self, kind: Kind) -> Option<String> {
        self.read(kind)
    }

    fn write(&mut self, kind: Kind, contents: String) {
        self.write(kind, contents);
    }
}
