pub fn longest_string<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

pub struct Simple<'a> {
    str1: &'a str,
    str2: &'a str,
}

impl<'a> Simple<'a> {
    fn new_string(&self) -> String {
        self.str1.to_owned() + self.str2
    }
}

struct DataHolder<'a> {
    data: &'a str,
}

impl<'a> DataHolder<'a> {
    fn get_data(&self) -> &'a str {
        self.data
    }
}
