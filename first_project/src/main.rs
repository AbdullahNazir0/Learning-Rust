fn main() {
    println!("Hello, world!");

    // Formatters in Rust, included in std::fmt
    println!("{} is my Name", "Abdullah");
    println!("{name} is my Name", name = "Abdullah");

    // Or you can also start series of indexes for arguments like,
    println!(
        "{0} {1} is my name. {0} is my first name and {1} is my last name",
        "Abdullah", "Nazir"
    );

    // Can also print different bases by using : followed by character,
    println!("Base 2:                {:b}", 256);
    println!("Base 8(octal):         {:o}", 256);
    println!("Base 10:               {}", 256);
    println!("Base 16(hexadecimal):   {:x}", 256);
    println!("Base 16(hexadecimal):   {:X}", 256);

    // To justify, use :>width
    println!("Name:{name:16}", name = "Abdullah"); // Left justify
    println!("Name:{name:>16}", name = "Abdullah"); // Right justify
    println!("Name:{name:^16}", name = "Abdullah"); // Center

    // To pad zeroes or any numbers, use
    println!("number: {:0>5}", 1);
    // And to flip the number to start, inverse the inequality
    println!("number: {:0<5}", 1);
    // can also use named arguments with $, like
    println!("number: {num:0>width$}", num = 1, width = 7);

    // Rust can also check if valid number of arguments are passed.
    // println!(
    //     "Lets use more than 1 arguments but pass only 1, {0}{1}",
    //     "This is one"
    // );

    // Note that only types inplemented by fmt::Display can be formatted with '{}', user defined types can not be formatted.
    // For example.
    // struct myStruct(i32);
    // println!("This is my struct: {}", myStruct(3));

    let pi = 3.141592;
    println!("The value of pi is {pi:.3}");
}
