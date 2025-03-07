fn main() {


    let y = {
        let mut x = 3;
        x +=1; 
        x
    };
    println!("Valor de y: {}", y);
    println!("incrementa(6) = {}", incrementa(6));
    teste_if();
    teste_if_aninhado();
}


fn incrementa(x: i32) -> i32{
    return x+ 1;
}


fn teste_if(){
    let numero = 3;

    if numero > 5{
        println!("Maior que 5");
    }else {
        println!("<= a 5");
    }
}


fn teste_if_aninhado(){
    let number = 6;
    

    if number % 4 == 0{
        println!("number é divisivel por 4");
    } else if number % 3 == 0 {
        println!("number é divisível por 3");
    }
}