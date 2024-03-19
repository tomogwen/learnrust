// struct for rectangle, area function

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}


fn main() {
    let rect = Rectangle {
        width: 10,
        height: 12
    };
    println!("Area = {}, width = {}", rect.area(), rect.width);
    println!("{:#?}", rect);

    let rect2 = Rectangle {
        width: 12,
        height: 15
    };
    println!("Can hold: {}", rect.can_hold(rect2));

    let square = Rectangle::square(12);
    println!("width of square is {}", square.width);
}
