use crate::gui::{Button, Dialog}; // import from gui.rs

// 按鈕
pub struct HtmlButton; // empty struct
impl Button for HtmlButton {
    fn render(&self) {
        println!("<button>Test Button</button>");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Button says - 'Hello World!'");
    }
}

// 對話框
pub struct HtmlDialog; // empty struct
impl Dialog for HtmlDialog {
    /// Creates an HTML button.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}