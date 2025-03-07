fn main() {
    let mut contador = 0;
    let mut contador2 = 0;
    'a: loop{
        println!("contador = {}", contador);
        contador += 1;

        'b: loop{
            contador2 += 1;
            println!("contador 2: {}", contador2);

            if contador2 == 10{
                break 'a;
            }

            if contador == 10{
                break contador + 5;
            }
        };
        println!("Contador: {}", contador);
    }
}