use iced::widget::{button, column, text, Column};
use iced::{Alignment, Font};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    // 根據訊息(Message)修改Counter的value
    fn update(&mut self, message: Message) {
        match message {
            // value 增加(Increment)
            Message::Increment => { 
                self.value += 1;
            }
            // value 減少(Decrement)
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    // 定義視窗及控制項佈局，並定義事件，傳遞訊息(Message)
    fn view(&self) -> Column<Message> {
        const FONT1: Font = Font::with_name("細明體"); // 標楷體
        let button1_text = text("增加").font(FONT1).size(50);
        let button2_text = text("減少").font(FONT1).size(50);

        column![ // 垂直排列
            // 點選【Increment】按鈕時會傳遞 Increment 訊息
            // button("Increment").on_press(Message::Increment),
            button(button1_text).on_press(Message::Increment),
            // 顯示Counter的value
            text(self.value).size(50).font(FONT1),
            // 點選【Decrement】按鈕時會傳遞 Decrement 訊息
            // button("Decrement").on_press(Message::Decrement)
            button(button2_text).on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
    }
}

pub fn main() -> iced::Result {
    iced::run("計數器", Counter::update, Counter::view)
}
