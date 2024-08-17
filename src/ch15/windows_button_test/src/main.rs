#![allow(unused)]

use native_windows_gui as nwg;
use nwg::NativeUi;
use native_windows_derive as nwd;
use nwd::NwgUi;

// 視窗及控制項佈局
#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), 
            title: "視窗測試", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "王小明", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "按我")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button
}

// 事件處理實作
impl BasicApp {
    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", 
                &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", 
            &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    // 初始化
    nwg::init().expect("Failed to init Native Windows GUI");
    
    // 設定字體
    nwg::Font::set_global_family("標楷體") // "Segoe UI")
        .expect("Failed to set default font");
        
    // 呼叫內建函數建立視窗
    let _app = BasicApp::build_ui(Default::default())
        .expect("Failed to build UI");
    
    // 監聽並提取與程式有關的訊息
    nwg::dispatch_thread_events();
}