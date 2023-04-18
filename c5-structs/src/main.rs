#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width >= other.width && self.height >= other.height {
            true
        } else {
            false
        }
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("{}", rect.area());

    let other_rect = Rectangle { width: 1, height:1 };
    let max_rect = rect.max(other_rect);
    println!("{}", max_rect.area());
    println!("{}", rect.area());

}
