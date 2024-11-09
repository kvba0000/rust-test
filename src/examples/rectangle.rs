use crate::helper::get_num_from_input;

// Simple implementation of Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

// Here we implement additional methods to Rectangle structure
// to keep everything organized.
impl Rectangle {
    // We can also set the name of method for already existing property in struct
    // fn width() -> bool {
    //    true
    // }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width &&
        self.height > rectangle.height
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }

    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

pub fn rectangle_init() {
    let width = get_num_from_input("Width:");
    let height = get_num_from_input("Height:");

    // Rectangle created by user
    let rect1 = Rectangle {
        width,
        height
    };

    // Example Rectangle 20x50
    let rect2 = Rectangle {
        width: 20.0,
        height: 50.0
    };

    let square1 = Rectangle::square(40.0);

    // can_hold will return true if Rectangle 2 can be contained inside of Rectangle 1
    // is_square will return true if Rectangle's both sides are equal

    println!(
        "Rectangle 1: {:?}\nRectangle 2: {:?}\nSquare 1: {:?}",
        &rect1, &rect2, &square1
    );
    
    println!(
        "Area of rectangle 1 is {}\nRectangle 1 {} hold Rectangle 2!\nRectangle 1 {} square!",
        &rect1.area(),
        if (&rect1).can_hold(&rect2) {"can"} else {"can't"},
        if (&rect1).is_square() {"is"} else {"is not"}
    );
}