use _3macro::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right {a: u32},
}

#[derive(Debug)]
#[allow(unused)]
struct DirectionUp<T> {
    speed: T,
}

impl <T> DirectionUp<T> {

    fn new(speed: T) -> Self {
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

    let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<i32> = 10.into();
    println!("{:?}, {:?}", up, left);

}

