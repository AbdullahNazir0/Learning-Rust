use crate::item;
use std::fmt;

#[derive(Debug)]
pub struct Order {
    pub order_number: usize,
    pub customer_name: String,
    pub total_items: usize,
    pub items: Vec<item::Item>,
    pub net_total: f64,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "----------------------Lunarsi Resturant----------------------\n\n"
        )?;
        writeln!(
            f,
            "                       Reciept of Sales                       \n\n"
        )?;
        writeln!(
        f,
        "Customer Name: {}\nOrder Number: {}\n-------------------------------------------------------------\n",
        self.customer_name, self.order_number
    )?;
        writeln!(
            f,
            "{:<3}\t{:<20}\t{:<}\n-------------------------------------------------------------\n",
            "QTY", "ITEM", "AMT"
        )?;
        for i in 0..self.total_items {
            writeln!(
                f,
                "{:<3}\t{:<20}\t${:<}",
                i + 1,
                self.items[i].name,
                self.items[i].price
            )?;
        }
        writeln!(
            f,
            "\n-------------------------------------------------------------\n"
        )?;
        writeln!(f, "{:<3}\t{:<20}\t${}", "", "Total", self.net_total)?;
        writeln!(
            f,
            "\n-------------------------------------------------------------\n"
        )?;
        writeln!(
            f,
            "                     Thanks for Visiting                     \n"
        )?;
        writeln!(
            f,
            "-------------------------------------------------------------\n"
        )?;

        Ok(())
    }
}
