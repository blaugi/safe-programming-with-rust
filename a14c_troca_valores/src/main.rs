#[derive(Copy, Clone, Debug, PartialEq)]
struct Ponto {
    x: i32,
    y: i32,
}

fn trocar_copiavel<T: Copy>(a: &mut T, b: &mut T) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut num1 = 10;
    let mut num2 = 20;
    println!("Antes da troca (i32): num1 = {}, num2 = {}", num1, num2);
    trocar_copiavel(&mut num1, &mut num2);
    println!("Depois da troca (i32): num1 = {}, num2 = {}", num1, num2);
    assert_eq!(num1, 20);
    assert_eq!(num2, 10);
    println!("Troca de i32 verificada com sucesso!");

    println!("---");

    let mut ponto_a = Ponto { x: 1, y: 2 };
    let mut ponto_b = Ponto { x: 3, y: 4 };
    println!("Antes da troca (Ponto): ponto_a = {:?}, ponto_b = {:?}", ponto_a, ponto_b);
    trocar_copiavel(&mut ponto_a, &mut ponto_b);
    println!("Depois da troca (Ponto): ponto_a = {:?}, ponto_b = {:?}", ponto_a, ponto_b);
    assert_eq!(ponto_a, Ponto { x: 3, y: 4 });
    assert_eq!(ponto_b, Ponto { x: 1, y: 2 });
    println!("Troca de Ponto verificada com sucesso!");

    println!("---");

    // let mut s1 = String::from("hello");
    // let mut s2 = String::from("world");
    // trocar_copiavel(&mut s1, &mut s2);
    // println!("Depois da troca (String): s1 = {}, s2 = {}", s1, s2);
    println!("(A tentativa de trocar Strings está comentada para evitar erro de compilação, pois String não é Copy)");
}