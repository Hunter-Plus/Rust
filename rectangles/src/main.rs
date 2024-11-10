//using Debug output format
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// using method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
        self.width > another_rectangle.width && self.height > another_rectangle.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 3;
    let height = 6;
    let rectangle2 = (30, 60);
    let rectangle3 = Rectangle {
        width: 300,
        height: 600,
    };

    println!("The area of the rectangle is {}", area(width, height));
    println!("The area of the second rectangle is {}", area2(rectangle2));
    println!("The area of the struct rectangle is {}", area3(&rectangle3));
    println!("Print the struct with debug format is {rectangle3:?}");
    // dbg macro will take the ownership
    dbg!(&rectangle3);
    println!(
        "The area of the rectangle calculated by the method is {}",
        rectangle3.area()
    );

    let sq1 = Rectangle::square(3);
    println!("Rectangle3 can hold the square? {}", rectangle3.can_hold(&sq1));
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn area2(dimmensions: (u32, u32)) -> u32 {
    dimmensions.0 * dimmensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
