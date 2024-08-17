use crate::gui::{Button, Dialog}; // import from gui.rs

// 按鈕
pub struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) {
        println!("Drawing a Windows button");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Hello, Windows!");
    }
}

// 對話框
pub struct WindowsDialog;
impl Dialog for WindowsDialog {
    /// Creates a Windows button.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}