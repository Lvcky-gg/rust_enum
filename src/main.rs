enum Shape {
    SQUARE,
    PENTAGON,
    OCTAGON,
}
impl Shape{
    fn corners(&self) -> i32 {
        match self {
            Shape::SQUARE => 4,
            Shape::PENTAGON => 5,
            Shape::OCTAGON => 8,
        }
    }
}

fn main() {
    let square: Shape = Shape::SQUARE; 
    println!("{:?}", square.corners());
}
