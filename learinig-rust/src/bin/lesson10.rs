// borrow
struct Book {
    pages:i32,
    rating:i32
}
fn display_pages(book:&Book){
    println!("pages :{:?}", book.pages);
}
fn display_rating(book:&Book){
    println!("rating :{:?}", book.rating);
}
fn main(){
let book = Book{
    pages: 5,
    rating: 3
};

display_pages(&book);
display_rating(&book);

}