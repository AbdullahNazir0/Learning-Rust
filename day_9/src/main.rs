// LifeTimes.
// Annotation: 'a (Name should be small, lower cased starting with apostrphy "'")
// Lifetime annotation dont change the lifetime of reference, but tells the compiler about the lifetime.s

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // First rule
    fn level(&self) -> i32 {
        3
    }

    // Third rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Rules
// 1. Compiler assigns a lifetime to each input parameter at compile time.
// 2. If there is exactly one input lifetime parameter, it is assigned to the output lifetime parameter.
// 3. If there are multiple input lifetime parameters, but one is &self or &mut self, then the lifetime of the self is assigned to the output parameter.

mod tasks;

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // -----------_>
    // Tasks
    let string1 = "This is string 1.";
    let string2 = "This is string 2.(bigger)";
    let result = tasks::longest_string(&string1, &string2);
    println!("{}", result);
}
