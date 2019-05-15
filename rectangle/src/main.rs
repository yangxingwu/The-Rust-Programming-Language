#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // this is method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // this is associated function
    fn square(length: u32) -> Rect {
        Rect {
            width: length,
            height: length,
        }
    }
}

fn main() {
    let width1 = 40;
    let height1 = 50;
    let rect1 = (40, 60);
    let rect2 = Rect {
        width: 70,
        height: 80,
    };
    let square1 = Rect::square(10);

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle 1 is {} square pixels",
        area_2(rect1)
    );

    println!(
        "The area of the rectangle 2 is {} square pixels",
        area_3(&rect2)
    );

    println!("The area of the rectangle {:?} is {}", rect2, rect2.area());
    println!("The area of the square {:?} is {}", square1, square1.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rect: &Rect) -> u32 {
    rect.width * rect.height
}
