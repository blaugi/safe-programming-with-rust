// 1. Run this program as it is and observe the output.
// 2. Implement the Describable trait for both the Video and Book structs.
// 3. After implementing the Describable trait, use it to call the describe method on instances of Video and Book to display custom descriptions.
mod describable;
use std::fmt::format;

use describable::Describable;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Describable for Book{
    fn describe(&self) -> String {
        format!("Title: {}\nAuthor: {}\nContent:{}", self.title, self.author, self.content)
    }
}

#[derive(Debug)]
pub struct Video {
    pub title: String,
    pub duration: u32, // duração em minutos
    pub content: String,
}

impl Describable for Video{
    fn describe(&self) -> String {
        format!("Title: {}\nDuration: {}\nContent: {}", self.title, self.duration, self.content)
    }
}

fn main() {
    let book = Book {
        title: String::from("O Senhor dos Anéis"),
        author: String::from("J.R.R. Tolkien"),
        content: String::from("Conteúdo do livro..."),
    };

    let video = Video {
        title: String::from("Rust Programming Tutorial"),
        duration: 120,
        content: String::from("Conteúdo do vídeo..."),
    };

    println!("Novo item: {:#?}", book);
    println!("{}", book.formatted_string());
    println!("Novo item: {:#?}", video);
    println!("{}", video.formatted_string());
}
