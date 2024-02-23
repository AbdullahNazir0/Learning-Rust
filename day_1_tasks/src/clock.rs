use std::fmt;

#[derive(Debug)]
struct Clock {
    hour: i8,
    minute: i8,
    second: i8,
    period: String,
    day: String,
    day_numerical: i8,
    month: String,
    year: i16,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{w_day}, {d} {mon} {yr} {hr:0>2}:{min:0>2}:{sec:0>2} {per} ",
            w_day = self.day,
            d = self.day_numerical,
            mon = self.month,
            yr = self.year,
            hr = self.hour,
            min = self.minute,
            sec = self.second,
            per = self.period,
        )
    }
}

fn main() {
    let current_clock = Clock {
        hour: 11,
        minute: 11,
        second: 58,
        period: String::from("PM"),
        day: String::from("Wednesday"),
        day_numerical: 14,
        month: String::from("February"),
        year: 2024,
    };

    println!("{}", current_clock);
    println!("{:?}", current_clock);
    println!("{:#?}", current_clock);
}
