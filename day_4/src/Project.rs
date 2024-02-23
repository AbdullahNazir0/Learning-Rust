#![allow(dead_code)]

#[derive(Debug)]
enum ItemCategory {
    Electronics,
    Clothing,
    Books,
    Toys,
}

#[derive(Debug)]
struct InventoryItem {
    name: String,
    category: ItemCategory,
    price: f64,
    quantity: u32,
}

impl std::fmt::Display for InventoryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Item Name: {0}\nItem Category: {1:#?}\nItem Price: ${2}\nItem Quantity: {3}",
            self.name, self.category, self.price, self.quantity
        )
    }
}

impl InventoryItem {
    fn display(&self) {
        println!("{}", self)
    }
    fn calculate_total_value(&self) {
        println!(
            "The total value of item ({0}) is ${1}",
            self.name,
            self.quantity as f64 * self.price
        )
    }
}

fn main() {
    let item1 = InventoryItem {
        name: String::from("Mobile Phone"),
        category: ItemCategory::Electronics,
        price: 999.0,
        quantity: 10,
    };
    println!("Item 1: {}", item1);
    item1.calculate_total_value();
    item1.display();
}
