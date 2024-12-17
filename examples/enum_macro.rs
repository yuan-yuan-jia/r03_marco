use _3macro::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right {a: u32},
}

#[derive(Debug)]
#[allow(unused)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {

    fn new(speed: u32) -> Self {
        Self {
            speed,
        }
    }

}

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::Up(value)
//     }
// }

fn main() {

    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?}, {:?}", up, left);

}

