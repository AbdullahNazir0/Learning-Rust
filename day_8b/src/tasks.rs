pub trait Printable {
    fn print(&self) {
        println!("Custom type printed.");
    }
}

struct Circle {
    radius: f64,
}

impl Printable for Circle {
    fn print(&self) {
        println!("The radius of circle is {}.", self.radius);
    }
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!(
            "The length of rectangle is {} and width is {}.",
            self.length, self.width
        );
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!(
            "The name of Person is {} and age is {}.",
            self.name, self.age
        );
    }
}

struct Book {
    pages: u32,
}

impl Printable for Book {}

fn generic_print<T: Printable>(one: &T, two: &T) {
    one.print();
    two.print();
}

trait Drawable {
    fn draw(&self);
}

struct Mutual {
    mutual_field: String,
}

impl Drawable for Mutual {
    fn Draw(&self) {
        println!("Mutual Drawn,");
    }
}

impl Printable for Mutual {} // Default Implementation.

trait Transfer {
    fn transform<T>(&self, item: &T) -> &T;
}

fn apply_transformation<T, U>(item: &T, transformer: &U) -> T
where
    U: Trasnformer<t>,
{
    transformer.transform(item);
}

fn main() {
    let circle = Circle { radius: 3.4 };
    let circle2 = Circle { radius: 5.1 };
    let rectangle = Rectangle {
        length: 4.2,
        width: 3.2,
    };
    let person = Person {
        name: "Abdullah".to_owned(),
        age: 19,
    };
    let book = Book { pages: 234 };
    circle.print();
    rectangle.print();
    person.print();
    book.print();
    generic_print(&circle, &circle2);
}
