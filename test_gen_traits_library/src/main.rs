mod describable;

use describable::Describable;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!(
            "Title: {}\nAuthor: {}\nContent: {}",
            self.title, self.author, self.content
        )
    }
}

#[derive(Debug)]
struct Library<T> {
    items: Vec<T>,
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

// Implement here a method enumerate_items for the struct Library that
// iterates over the stored items and calls the describe method on
// each item. The method should display the description of each
// item in the library.

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

fn main() {
    let library = Library {
        items: vec![
            Book {
                title: String::from("The Fellowship of the Ring"),
                author: String::from("J.R.R. Tolkien"),
                content: String::from("Content of the first book..."),
            },
            Book {
                title: String::from("The Two Towers"),
                author: String::from("J.R.R. Tolkien"),
                content: String::from("Content of the second book..."),
            },
        ],
    };

    println!("{:?}", library);
    //library.enumerate_items();
}
