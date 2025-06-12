struct Book {
    isbn: String,
    title: String,
    pages: i32,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping Book {} {} with {} pages. {:p}", self.isbn, self.title, self.pages, self);
    }
}

fn buy_books(b1: Book, b2: Book) {
    let shelf : [Book;2] = [b1,b2];
    println!("{} - {}, pages: {}", shelf[0].isbn, shelf[0].title, shelf[0].pages);
}

fn main() {
  let b1 = Book {isbn: String::from("978-1718503106"),
                 title : String::from("The Rust Programming Language"),
                 pages: 560};
  let b2 = Book {isbn: String::from("978-0131101630"),
                 title : String::from("The C Programming Language"),
                 pages: 228};
  buy_books(b1,b2);
  println!("{}", b1.title);
}
