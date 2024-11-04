// http://rust.w8.pl/book/ch04-01-what-is-ownership.html

pub fn ownership_init() {
    // Presentation of moving literal value to another variable
    {
        let x = "meow";
        let mut y = x;

        println!("BEFORE y MODIFICATION x: {}", x);
        println!("BEFORE y MODIFICATION y: {}", y);

        y = "woof";

        println!("AFTER y MODIFICATION x: {}", x);
        println!("AFTER y MODIFICATION y: {}", y);

        // Nothing bad happens here because we are not moving the ownership of the value.
        // We are moving literal value to another variable.
    }

    println!("--------------------------------");

    // Presentation of cloning value to another variable
    {
        // \/ Not a literal, but a value that is allocated on the heap
        let x = String::from("meow");
        let mut y = x.clone(); // <- This is the point where the value of x is cloned

        println!("BEFORE y MODIFICATION x: {}", x);
        println!("BEFORE y MODIFICATION y: {}", y);

        y.push_str("woof");

        println!("AFTER y MODIFICATION x: {}", x);
        println!("AFTER y MODIFICATION y: {}", y);

        // Nothing bad happens here because we are not moving the ownership of the value.
        // We are cloning the value to another variable.
        
        // We can modify both values without any problems because both have different owners
        // Meaning they have different memory addresses
    }

    println!("--------------------------------");

    // Presentation of moving ownership to another variable - making older variable invalid
    {
        let mut x = String::from("meow");
        let y = &mut x; // <- This is the point where we move the ownership of x to y

        println!("BEFORE y MODIFICATION y: {}", y);

        y.push_str("woof"); // <- This is the point where the value of original x (now y) is modified

        println!("AFTER y MODIFICATION x: {}", x);

        // We cannot use x after this point because it no longer exists for Rust
        // From now on, y owns the value of x and variable "x" is no longer valid
    }

    println!("Check the code for more details!");
}
