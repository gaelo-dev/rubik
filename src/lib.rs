use std::fmt::{self, Display};
use nalgebra_glm as glm;
use moves::Move;

#[derive(Debug, Clone)]
struct Sticker {
    pos: glm::Vec3,
    dst: glm::Vec3,
}

impl Sticker {
    fn new(pos: glm::Vec3) -> Self {
        let dst = pos.clone();
        Self { pos, dst }
    }
    
    fn apply(&mut self, m: &Move) {
        if (m.predicate)(&self.pos) {
            let angle = -m.angle / 180. * glm::pi::<f32>();
            let new_pos = glm::rotate_vec3(&self.pos, angle, &m.axis);
            self.pos = glm::round(&new_pos);
        }
    }

    fn get_original_face(&self) -> char {
        get_face(&self.dst)
    }
}

fn get_face(axis: &glm::Vec3) -> char {
    let (x, y, z) = (axis.x, axis.y, axis.z);
    match () {
        _ if x == 3. => 'R',
        _ if x == -3. => 'L',
        _ if y == 3. => 'U',
        _ if y == -3. => 'D',
        _ if z == 3. => 'F',
        _ if z == -3. => 'B',
        _ => 'X',
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    stickers: Vec<Sticker>
}

impl Cube {
    pub fn solved() -> Self {
        let mut stickers = Vec::new();
        for face in [3., -3.] {
            for coord1 in [-2., 0., 2.] {
                for coord2 in [-2., 0., 2.] {
                    stickers.push(Sticker::new(glm::vec3(face, coord1, coord2)));
                    stickers.push(Sticker::new(glm::vec3(coord1, face, coord2)));
                    stickers.push(Sticker::new(glm::vec3(coord1, coord2, face)));
                }
            }
        }

        Self { stickers }
    }

    pub fn apply_move(&mut self, m: &str) {        
        let m = m.into();
        self.stickers.iter_mut().for_each(|sticker| sticker.apply(&m))
    }

    pub fn apply_moves(&mut self, moves: &str) {
        moves.split_whitespace().for_each(|m| self.apply_move(m));
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scube = vec![];
        
        let mut fill_face = |stickers: &mut [Sticker]| {
            stickers.sort_by(|s, t| {
                if s.pos.z != t.pos.z {
                    s.pos.z.partial_cmp(&t.pos.z).unwrap()
                } else {
                    s.pos.x.partial_cmp(&t.pos.x).unwrap()
                }
            });

            for s in stickers {
                scube.push(s.get_original_face());
            }
        };

        let face_rotating_moves = ["", "y x", "x", "x2", "y' x", "y2 x"];    
        for moves in face_rotating_moves.into_iter() {
            let mut cube = self.clone();
            cube.apply_moves(moves);

            let mut stickers = cube.stickers.into_iter().filter(|s| get_face(&s.pos) == 'U').collect::<Vec<_>>();
            fill_face(&mut stickers);
        }

        write!(f, "{}", scube.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::Cube;

    #[test]
    fn it_works() {
        let mut cube = Cube::solved();
        assert_eq!(cube.to_string(), "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB");

        cube.apply_move("U");
        assert_eq!(cube.to_string(), "UUUUUUUUUBBBRRRRRRRRRFFFFFFDDDDDDDDDFFFLLLLLLLLLBBBBBB");
    }
}

mod moves {
    use nalgebra_glm as glm;

    type Predicate = Box<dyn Fn(&glm::Vec3) -> bool>;

    pub(crate) struct Move {
        // pub(crate) name: char,
        pub(crate) axis: glm::Vec3,
        pub(crate) angle: f32,
        pub(crate) predicate: Predicate
    }

    macro_rules! create_moves {
        ($( $name:expr => $axis:expr, |$pos:ident| $cond:expr ; )*) => {
            impl From<&str> for Move {
                fn from(value: &str) -> Self {
                    match value {
                        $(
                            $name => Move { axis: $axis, angle: 90., predicate: Box::new(|$pos| $cond) },
                            concat!($name, "2") => Move { axis: $axis, angle: 180., predicate: Box::new(|$pos| $cond) },
                            concat!($name, "'") => Move { axis: $axis, angle: 270., predicate: Box::new(|$pos| $cond) },
                        )*
                        _ => panic!("invalid notation"),
                    }
                }
            }
        };
    }
    
    create_moves![
        "U" => glm::vec3(0., 1., 0.), |pos| pos.y > 0.;
        "u" => glm::vec3(0., 1., 0.), |pos| pos.y >= 0.;
        "D" => glm::vec3(0., -1., 0.), |pos| pos.y < 0.;
        "d" => glm::vec3(0., -1., 0.), |pos| pos.y <= 0.;
        
        "E" => glm::vec3(0., 1., 0.), |pos| pos.y == 0.;
        "y" => glm::vec3(0., 1., 0.), |_pos| true;
        
        "L" => glm::vec3(-1., 0., 0.), |pos| pos.x < 0.;
        "R" => glm::vec3(1., 0., 0.), |pos| pos.x > 0.;
        "l" => glm::vec3(-1., 0., 0.), |pos| pos.x <= 0.;
        "r" => glm::vec3(1., 0., 0.), |pos| pos.x >= 0.;
        "M" => glm::vec3(-1., 0., 0.), |pos| pos.x == 0.;
        "x" => glm::vec3(1., 0., 0.), |_pos| true;
        
        "F" => glm::vec3(0., 0., 1.), |pos| pos.z > 0.;
        "B" => glm::vec3(0., 0., -1.), |pos| pos.z < 0.;
        "S" => glm::vec3(0., 0., 1.), |pos| pos.z == 0.;
        "z" => glm::vec3(0., 0., 1.), |_pos| true;
    ];
    
    #[cfg(test)]
    mod tests {
        use super::{glm, Move};

        #[test]
        fn it_works() {
            let move1: Move = "U2".into();
            let move2 = Move { axis: glm::vec3(0., 0., 1.), angle: 180., predicate: Box::new(|pos| pos.y > 0.) };

            assert_eq!(move1.angle, move2.angle);
            assert_eq!(move1.axis, move2.axis);
        }
    }
}
