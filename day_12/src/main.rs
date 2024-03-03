// Smart Pointers
// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership. (example of a TV in a shared living room)[only used in single-threaded scenarios]
// RefCell<T> is a type that enforces the borrowing rules at runtime instead of compile time
//      Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
//      RefMut<T> is a mutable reference to data inside a RefCell<T>

// Recap
/*
Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};

// Defining our own smart pointer

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
// drop function will be called automatically when the variable goes out of scope, however, it can be called manually using std::mem::drop. But disabling the automatic drop functionality is not straight forward.
use std::mem::drop;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, *y);

    let my_smart_pointer = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("{:?}", my_smart_pointer);
    drop(my_smart_pointer);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
