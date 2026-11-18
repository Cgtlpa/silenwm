// Window manager
use super::window::{Window, WindowId, WindowMode};

pub struct WindowManager {
    pub windows: Vec<Window>,
    pub focused: Option<WindowId>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused: None,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        let id = window.id;

        self.windows.push(window);
        self.focused = Some(id);
    }

    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|window| window.id != id);

        if self.focused == Some(id) {
            self.focused = self.windows.last().map(|w| w.id);
        }
    }

    pub fn tiled_windows(&self) -> Vec<&Window> {
        self.windows
            .iter()
            .filter(|window| window.mode == WindowMode::Tiled)
            .collect()
    }
}
