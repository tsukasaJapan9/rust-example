use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
use iced_futures::{self, futures};
use std::time::{Duration, Instant};
use std::hash::Hash;
use std::ops::Sub;

const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../fonts/PixelMplus12-Regular.ttf"),
};

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Update,
}

pub enum TickStates {
    Stopped,
    Ticking,
}

pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration }
    }
}

impl <H, E> iced_native::subscription::Recipe<H, E> for Timer where H: std::hash::Hasher,
{
    type Output = Instant;
    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}

struct GUI {
    tick_state: TickStates,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI{
                tick_state: TickStates::Stopped,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickStates::Ticking;
            },
            Message::Stop => {
                self.tick_state = TickStates::Stopped;
            },
            Message::Reset => {}
        }
        Command::none()
    }
    fn view(&mut self) -> Element<Self::Message>{
        let duration_text = "00:00:00.00";
        let start_stop_text = match self.tick_state {
            TickStates::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            TickStates::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT)
        };

        let start_stop_message = match self.tick_state {
            TickStates::Stopped => Message::Start,
            TickStates::Ticking => Message::Stop,
        };

        // init widget
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        ).min_width(80).on_press(Message::Reset);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }
}

fn main() {
    let mut setting = Settings::default();
    setting.window.size = (400u32, 120u32);
    GUI::run(setting);
}
