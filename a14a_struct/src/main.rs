
fn main() {
    // ParOrdenado com i32
    let par_i32 = ParOrdenado::new(10, 20);
    println!("par_i32: {:?}", par_i32);
    println!("x: {}, y: {}", par_i32.x(), par_i32.y());
    assert_eq!(par_i32.x(), &10);
    assert_eq!(par_i32.y(), &20);

    // ParOrdenado com &str
    let par_str = ParOrdenado::new("olá", "mundo");
    println!("par_str: {:?}", par_str);
    println!("x: {}, y: {}", par_str.x(), par_str.y());
    assert_eq!(par_str.x(), &"olá");
    assert_eq!(par_str.y(), &"mundo");

    // ParOrdenado com f64
    let par_f64 = ParOrdenado::new(3.14, 2.71);
    println!("par_f64: {:?}", par_f64);
    println!("x: {}, y: {}", par_f64.x(), par_f64.y());
    assert_eq!(par_f64.x(), &3.14);
    assert_eq!(par_f64.y(), &2.71);

    // Exemplo comentado: tipos diferentes (gera erro de compilação)
    // let par_misto = ParOrdenado::new(10, "texto");
}

#[derive(Debug, PartialEq)]
struct ParOrdenado<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Debug> ParOrdenado<T> {
    pub fn new(x: T, y: T) -> Self {
        ParOrdenado { x, y }
    }

    pub fn x(&self) -> &T {
        println!("x: {:?}", self.x);
        &self.x
    }

    pub fn y(&self) -> &T {
        println!("y: {:?}", self.y);
        &self.y
    }
}