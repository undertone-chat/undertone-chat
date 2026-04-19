mod session;
use iced::widget::{Column, button, column, text};
use session::Session;

#[derive(Debug, Clone, Copy)]
enum Message {
    Connect,
    Disconnect,
}
#[derive(Default)]
struct App {
    session: Session,
}

impl App {
    pub fn update(&mut self, message: Message) {
        self.session.update(message);
    }

    fn view(&self) -> Column<'_, Message> {
        let interface = column![
            button("Connect").on_press(Message::Connect),
            button("Disconnect").on_press(Message::Disconnect),
            text("Diconnected"),
        ];

        interface
    }
}
fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
