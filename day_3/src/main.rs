#![allow(dead_code)]

// There are three types of structs.
// -> Tuple structs
// -> C-like structs
// -> Unit structs

// -> Tuple Structs.
#[derive(Debug)]
struct MyTupleStruct(i32, bool);

// -> C-type Structs.
#[derive(Debug)]
struct MyCTypeStruct {
    field1: i32,
    field2: bool,
}

// -> Unit struct.
#[derive(Debug)]
struct MyUnitStruct; // No fields.

// --------------------------------------------------------------------------------------------------

// Enums

enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

// Type Aliase for Enum.
type WB = WebEvent;

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded."),
        WebEvent::PageUnLoad => println!("Page UnLoaded."),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),
        WebEvent::Click { x, y } => println!("Clicked at {0}x, {1}y", x, y),
    }
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            Self::PageLoad => println!("Page Loaded."),
            Self::PageUnLoad => println!("Page UnLoaded."),
            Self::KeyPress(c) => println!("pressed {}", c),
            Self::Paste(s) => println!("Pasted {}", s),
            Self::Click { x, y } => println!("Clicked at {0}x, {1}y", x, y),
        }
    }
}

fn main() {
    // Structs
    let tuple_struct = MyTupleStruct(4, true);
    let c_type_struct = MyCTypeStruct {
        field1: 4,
        field2: true,
    };
    let unit_struct = MyUnitStruct;

    println!("Tuple Struct: {:?}", tuple_struct);
    println!("C-Type Struct: {:#?}", c_type_struct);
    println!("Unit Struct: {:?}", unit_struct);

    // Enums
    // let load = WebEvent::PageLoad;
    // let pressed = WebEvent::KeyPress('x');
    // let pasted = WebEvent::Paste("My_text".to_owned());
    // let clicked = WebEvent::Click { x: 3, y: 4 };
    // let unload = WebEvent::PageUnLoad;

    // Using Type Aliase.
    let load = WB::PageLoad;
    let pressed = WB::KeyPress('x');
    let pasted = WB::Paste("My_text".to_owned());
    let clicked = WB::Click { x: 3, y: 4 };
    let unload = WB::PageUnLoad;

    inspect(load);
    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(unload);

    // Explicitly 'use' each name so that they are available without manual scoping.
    // use crate::WebEvent::{Load, Paste};
    // To use all names.
    use crate::WebEvent::*;
    inspect(PageLoad);
    inspect(KeyPress('a'));
    inspect(Paste("Paste this".to_owned()));
    inspect(Click { x: 2, y: 3 });
    inspect(PageUnLoad);
}
