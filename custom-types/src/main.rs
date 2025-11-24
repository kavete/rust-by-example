#![allow(unused)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

//tuple struct
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Enums

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        }
    }
}

// use

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // Field init shorthand
    //

    use crate::Role::*;
    use crate::Stage::{Advanced, Beginner};

    let name = String::from("Kavete");
    let age = 27;

    let kavete = Person { name, age };

    let point = Point { x: 5.2, y: 0.4 };

    println!("Point coordinates {} {}", point.x, point.y);

    let pressed = WebEvent::KeyPress('K');

    inspect(pressed);

    let stage = Beginner;

    let role = Teacher;
}
