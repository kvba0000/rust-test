use crate::helper::get_num_from_input;

#[derive(Debug)]

// Simple implementation of Rectangle struct
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

    println!("Rectangle 1: {:?}", &rect1);
    println!("Rectangle 2: {:?}", &rect2);
    println!("Square 1: {:?}", &square1); // <- We created square using square function in Rectangle implementation

    // can_hold will return true if Rectangle 2 can be contained inside of Rectangle 1
    // is_square will return true if Rectangle's both sides are equal
    
    println!("Area of rectangle 1 is {}", &rect1.area());
    println!("Rectangle 1 {} hold Rectangle 2!", if (&rect1).can_hold(&rect2) {"can"} else {"can't"});
    println!("Rectangle 1 {} square!", if (&rect1).is_square() {"is"} else {"is not"});
}