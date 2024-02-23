#[allow(dead_code)]

const MAX_BOOKS: usize = 100;

#[derive(Debug, PartialEq, Clone)]
enum BookStatus {
    Available,
    CheckedOut,
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    status: BookStatus,
}

impl Book {
    fn new() -> Book {
        Book {
            title: "".to_owned(),
            author: "".to_owned(),
            isbn: "".to_owned(),
            status: BookStatus::Available,
        }
    }
}

#[derive(Debug)]
struct Library {
    books: [Book; MAX_BOOKS],
    num_books: usize,
}

impl Library {
    // fn new() -> Library {
    //     Library {
    //         books: [Book::new(); MAX_BOOKS],
    //         num_books: 0,
    //     }
    // }
    fn new() -> Library {
        let default_book = Book::new();
        Library {
            books: [default_book; MAX_BOOKS],
            num_books: 0,
        }
    }
    fn add_book(&mut self, book: &Book) {
        self.books[self.num_books] = book.clone();
        self.num_books += 1;
    }

    fn display_books(&self) {
        for i in 0..self.num_books {
            println!("Book {0}: {1:#?}", i, self.books[i]);
        }
    }

    fn checkout_book(&self, isbn: String) {
        let mut flag: bool = false;
        for i in 0..self.num_books {
            if self.books[i].isbn == isbn {
                flag = true;
                if self.books[i].status == BookStatus::CheckedOut {
                    println!("Book with isbn ({}) is already checked out.", isbn);
                } else {
                    self.books[i].status = BookStatus::CheckedOut;
                    println!("Book with isbn ({}) checked out successfully.", isbn);
                }
                return;
            }
        }
        println!("Can't find book with this isbn ({}) in library.", isbn);
    }
    fn return_book() {}
}

fn main() {}
