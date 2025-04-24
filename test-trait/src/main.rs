use std::fmt::format;

pub trait Describable {
    fn describe(&self) -> String;

    fn formatted_string(&self) -> String {
        format!("+++++++++++++\n{}\n+++++++++++", self.describe())
    }
}

impl Describable for String {
    fn describe(&self) -> String {
        format!("Esta é a descrição de {}", self)
    }
}

impl Describable for i32 {
    fn describe(&self) -> String {
        format!("Esta é a descrição do inteiro {}", self)
    }
}

fn main() {
    let s = "Olá mundo".to_string();
    println!("{}", s.formatted_string());
    let i = 32;
    println!("{}", i.formatted_string());
}
