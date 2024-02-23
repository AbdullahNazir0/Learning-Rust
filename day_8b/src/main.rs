#![allow(dead_code)]
use std::fmt;

// Generic Types.
fn get_largest_integer(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn get_largest_character(list: &[char]) -> &char {
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

// fn largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for num in list {
//         // if num > largest {
//         //     // Will correct this
//         //     largest = num;
//         // }
//     }
//     largest
// }

// Similarly for structs

struct MyStruct<T> {
    field1: T,
    field2: T,
}
// impl<T> fmt::Display for MyStruct<T> {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{:?}{:?}", self.field1, self.field2)
// }
// }
impl<T> MyStruct<T> {}
// We can also implement methods on concrete instances.
impl MyStruct<f32> {}
// can also use multiple generic types.
struct MyStruct2<T, U> {
    field1: T,
    field2: U,
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Traits
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle with radius {} drawn.", self.radius);
    }
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "Rectangle with length {}, width {} drawn.",
            self.length, self.width
        );
    }
}

// Traits can also be passed as parameters.
fn draw_shape(shape: &impl Drawable) {
    // This function can accept anytime that implements the trait Drawable.
    shape.draw();
}
// Trait Bound.
fn draw_shape2<T: Drawable>(shape: &T) {
    shape.draw();
}
// Can also specify multiple trait bounds with + operator.
fn draw_shape3(shape: &(impl Drawable + Area)) {}
fn draw_shape4<T: Drawable + Area>(shape: &T) {}

// Clear Trait Bounds with where clause.
fn draw_shape5<T: Drawable + fmt::Debug, U: Clone + Area>() {}
// can alse be written as
fn draw_shape6<T, U>(t: &T, u: &U)
where
    T: Drawable + fmt::Debug,
    U: Clone + Area,
{
}

// Traits can also have default implementation.
trait Area {
    fn area(&self) {
        println!("This is the area of the shape.");
    }
}
impl Area for Circle {
    fn area(&self) {
        println!("This is the area of the circle.");
    }
}
struct Square {
    length: f64,
}

// Returning types that implement trait.
fn return_type_with_impl() -> impl Area {
    Circle { radius: 45.2 }
}

// Using Trait Bounds to Conditionally Implement Methods

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
