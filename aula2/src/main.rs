
fn main() {
    let x: i32 = 5;
    println!("O valor de x = {}", x);
    let mut x: &str = "Interessante..."; // let again because redeclaring variable type
    {
        x = "oi";
        println!("O valor de x = {}", x);
    }    

    println!("O valor de x = {}", x);
}
