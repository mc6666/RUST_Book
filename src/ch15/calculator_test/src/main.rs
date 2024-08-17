#![allow(unused)]

use native_windows_gui as nwg;
use nwg::NativeUi;
use native_windows_derive as nwd;
use nwd::NwgUi;

// 0~9、+、-、*、/ 按鈕
#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Mult,
    Div
}

// 視窗及控制項佈局
#[derive(Default, NwgUi)]
pub struct Calculator {

    #[nwg_control(size: (300, 150), position: (300, 300), title: "Calculator")]
    #[nwg_events( OnWindowClose: [Calculator::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2, min_size: [150, 140])]
    grid: nwg::GridLayout,

    #[nwg_control(text: "", align: nwg::HTextAlign::Right, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 5)]
    input: nwg::TextInput,

    #[nwg_control(text: "1", focus: true)] 
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn1: nwg::Button,

    #[nwg_control(text: "2")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn2: nwg::Button,

    #[nwg_control(text: "3")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn3: nwg::Button,

    #[nwg_control(text: "4")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn4: nwg::Button,

    #[nwg_control(text: "5")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn5: nwg::Button,

    #[nwg_control(text: "6")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn6: nwg::Button,

    #[nwg_control(text: "7")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn7: nwg::Button,

    #[nwg_control(text: "8")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn8: nwg::Button,

    #[nwg_control(text: "9")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn9: nwg::Button,

    #[nwg_control(text: "0")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 4, col_span: 3)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn0: nwg::Button,

    #[nwg_control(text: "+")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn_plus: nwg::Button,

    #[nwg_control(text: "-")] 
    #[nwg_layout_item(layout: grid, col: 4, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn_minus: nwg::Button,

    #[nwg_control(text: "*")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn_mult: nwg::Button,

    #[nwg_control(text: "/")] 
    #[nwg_layout_item(layout: grid, col: 4, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::number(SELF, CTRL)] )]
    btn_divide: nwg::Button,

    #[nwg_control(text: "Clear")]
    #[nwg_layout_item(layout: grid, col: 3, row: 3, col_span: 2)]
    #[nwg_events( OnButtonClick: [Calculator::clear] )]
    btn_clear: nwg::Button,

    #[nwg_control(text: "=")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 4, col_span: 2)]
    #[nwg_events( OnButtonClick: [Calculator::compute] )]
    btn_process: nwg::Button,
}

impl Calculator {
    // 輸入0~9時的算術式處理
    fn number(&self, button: &nwg::Button) {
        let text = self.input.text();
        self.input.set_text(&format!("{}{}", text, button.text()));
    }

    // 清除算術式
    fn clear(&self) {
        self.input.set_text("");
    }

    // 算術式運算
    fn compute(&self) {
        use Token::*;
        static SYMBOLS: &'static [char] = &['+', '-', '*', '/'];

        let eq = self.input.text();
        if eq.len() == 0 {
            return;
        }

        let mut tokens: Vec<Token> = Vec::with_capacity(5);
        let mut last = 0;

        for (i, chr) in eq.char_indices() { // 逐個字元解析算術式
            if SYMBOLS.iter().any(|&s| s == chr) {
                let left = &eq[last..i]; // 取數字
                match left.parse::<i32>() { // 數字推入 tokens 陣列
                    Ok(i) => tokens.push(Token::Number(i)),
                    _ => {
                        nwg::error_message("Error", "Invalid equation!");
                        self.input.set_text("");
                        return
                    }
                }

                let tk = match chr {
                    '+' => Plus,
                    '-' => Minus,
                    '*' => Mult,
                    '/' => Div,
                    _ => unreachable!() // 若是其他符號，顯示錯誤訊息
                };

                tokens.push(tk); // 運算子推入 tokens 陣列

                last = i+1;
            }
        }

        // 運算子右方字串解析
        let right = &eq[last..];
        match right.parse::<i32>() {
            Ok(i) => tokens.push(Token::Number(i)),
            _ =>  {
                nwg::error_message("Error", "Invalid equation!");
                self.input.set_text("");
                return
            }
        }

        // 運算
        let mut i = 1;
        // 取第1個數字
        let mut result = match &tokens[0] { Token::Number(n) => *n, _ => unreachable!() };
        // 一次取2個數字
        while i < tokens.len() {
            match [&tokens[i], &tokens[i+1]] {
                [Plus, Number(n)] => { result += n; },
                [Minus, Number(n)] => { result -= n;},
                [Mult, Number(n)] => { result *= n; },
                [Div, Number(n)] => { result /= n; },
                _ => unreachable!()
            }
            i += 2;
        }

        // 運算結果顯示
        self.input.set_text(&result.to_string());
    }

    // 結束程式
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    // 初始化
    nwg::init().expect("Failed to init Native Windows GUI");
    
    // 設定字體
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    // 呼叫內建函數建立視窗
    let _calc = Calculator::build_ui(Default::default()).expect("Failed to build UI");

    // 監聽並提取與程式有關的訊息
    nwg::dispatch_thread_events();
}
