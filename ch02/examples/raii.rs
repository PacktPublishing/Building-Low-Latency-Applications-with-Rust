#[derive(Debug)]
pub struct MyString {
    inner: String,
}

impl MyString {
    pub fn new(str: &str) -> Self {
        MyString {
            inner: String::from(str),
        }
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("Dropping MyString {}", self.inner);
    }
}

#[derive(Debug)]
pub struct Book {
    isbn: MyString,
    title: MyString,
    pages: i32,
}

impl Book {
    pub fn new(isbn: &str, title: &str, pages: i32) -> Self {
        Book {
            isbn : MyString::new(isbn),
            title: MyString::new(title),
            pages
        }
    }
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("*********************************");
        println!("Dropping Book {} {} with {} pages. {:p}", self.isbn.inner, self.title.inner, self.pages, self);
    }
}

#[derive(Debug)]
pub struct Shelf {
    books : [Book; 3],
}

impl Shelf {
    pub fn new(b1: Book, b2: Book, b3: Book) -> Self {
        Shelf {
            books : [b1, b2, b3],
        }
    }
}

impl Drop for Shelf {
    fn drop(&mut self) {
        println!("Dropping Shelf");
     }
}

fn buy_books(b1: Book, b2: Book, b3: Book) {
    let shelf = Shelf::new(b1,b2,b3);
    println!("All books are in the shelf");
    println!("{:?}", shelf.books);
}

fn main() {
    println!("Start buying books for my shelf");
    let b1 = Book::new("978-1718503106", "The Rust Programming Language", 560);
    let b2 = Book::new("978-0131101630", "The C Programming Language", 228);
    let b3 = Book::new("978-0321958327 ", "The C++ Programming Language", 1376);
    buy_books(b1,b2,b3);
    println!("Shelf and all books deallocated!");
}
