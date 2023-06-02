struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        // todo!("Return the length of `self.books`")
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        // todo!("Return `true` if `self.books` is empty")
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        // todo!("Add a new book to `self.books`")
        self.books.push(book);
        self.books.sort_by(|a, b| a.year.cmp(&b.year))
    }

    fn print_books(&self) {
        // todo!("Iterate over `self.books` and each book's title and year")
        for book in &self.books {
            println!("Book title: {0}, Book year: {1}\n", book.title, book.year)
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        // todo!("Return a reference to the oldest book (if any)")
        if !self.is_empty() {
            return None;
        } else {
            return Some(&self.books[0]);
        }
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
pub(crate) fn library_runner() {
    let mut library = Library::new();

    println!("The library is empty: {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: {}", library.is_empty());

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}
