use iced::{
    widget::{
        canvas::{
            self, Cursor, Frame, Geometry, Path, Stroke
        }, 
        text_input, column, row, button
    },
    Sandbox, Element, Vector, Point, Size, Settings, Theme, Color
};
use rubik::Cube as GCube;

fn main() -> iced::Result {
    Rubik::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    InputChange(String),
    Apply,
    Reset,
}

struct Rubik {
    gcube: GCube,
    moves: String
}

impl Sandbox for Rubik {
    type Message = Message;
    
    fn new() -> Self {
        Rubik {
            gcube: GCube::solved(),
            moves: String::new()
        }
    }

    fn title(&self) -> String {
        "Cube".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChange(s) => {
                self.moves = s;
            }
            Message::Apply => {
                if !self.moves.is_empty() {
                    self.gcube.apply_moves(&self.moves);
                }
            }
            Message::Reset => {
                self.gcube = GCube::solved()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            row![
                text_input("moves", &self.moves, Message::InputChange),
                button("Apply")
                    .on_press(Message::Apply),
                button("Reset")
                    .on_press(Message::Reset)
            ],
            cube(15., &self.gcube.to_string())
        ].into()
    }
}

#[derive(Debug)]
enum Sticker {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green
}

impl Sticker {
    fn into_color(self) -> Color {
        self.into()
    }
}

impl From<char> for Sticker {
    fn from(value: char) -> Self {
        match value {
            'U' => Sticker::White,
            'R' => Sticker::Red,
            'F' => Sticker::Green,
            'D' => Sticker::Yellow,
            'L' => Sticker::Orange,
            'B' => Sticker::Blue,
            _ => panic!("sticker color cannot be obtained")
        }
    }
}

impl Into<Color> for Sticker {
    fn into(self) -> Color {
        match self {
            Sticker::White => Color::WHITE,
            Sticker::Red => Color::from_rgb8(255, 0, 0),
            Sticker::Green => Color::from_rgb8(0, 255, 0),
            Sticker::Yellow => Color::from_rgb8(255, 255, 0),
            Sticker::Orange => Color::from_rgb8(255, 102, 0),
            Sticker::Blue => Color::from_rgb8(0, 0, 255)
        }
    }
}

fn cube<Msg>(sc: f32, state: &str) -> canvas::Canvas<Msg, Theme, FCube> {
    canvas::Canvas::new(FCube::new(sc, state))
}

struct FCube {
    sc: f32,
    state: String,
}

impl FCube {
    fn new(sc: f32, state: &str) -> Self {
        Self { sc, state: state.to_string() }
    }

    fn draw_face(&self, frame: &mut Frame, id: usize, r: u8, c: u8) {
        println!("id: {id}, r: {r}, c: {c}");
        let mut id = id;
        for i in r..r+3 {
            for j in c..c+3 {
                let sticker: Sticker = self.state.char_indices().find_map(|(i, c)| {
                    if i == id {
                        Some(c.into())
                    } else {
                        None
                    }
                }).unwrap();
                id += 1;
                println!("{sticker:?}");

                let rect = Path::rectangle(
                    Point::new(self.sc * j as f32, self.sc * i as f32), 
                    Size::new(self.sc, self.sc)
                );

                frame.fill(&rect, sticker.into_color());
                frame.stroke(&rect, Stroke::default().with_width(2.));

                println!("i: {i}, j: {j}\n");
            }
        }
    }
}

impl<Msg> canvas::Program<Msg> for FCube {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &iced_native::Theme,
        _bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(Size::new(13. * self.sc, 10. * self.sc));
        frame.translate(Vector::new(self.sc / 2.0, self.sc / 2.0));
        
        self.draw_face(&mut frame, 0, 0, 3); // U
        self.draw_face(&mut frame, 9, 3, 6); // R
        self.draw_face(&mut frame, 18, 3, 3); // F 
        self.draw_face(&mut frame, 27, 6, 3); // D
        self.draw_face(&mut frame, 36, 3, 0); // L
        self.draw_face(&mut frame, 45, 3, 9); // B

        vec![frame.into_geometry()] 
    }
}
