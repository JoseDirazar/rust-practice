#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    println!(
        "This is the area of the rectangle, {}",
        rectangle.area()
    );
        println!("{:#?}", rectangle);
}
