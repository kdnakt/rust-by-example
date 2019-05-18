#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition",
        book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition",
        book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    new_edition(&mut mutabook);
    //new_edition(&mut immutabook);
    // error: cannot borrow immutable local variable as mutable
}
