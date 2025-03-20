fn main() {
    let teste = String::from("Atenção: aqui tem uma pegadinha!!!");
    // enconding pra bytes de caracteres especiais é diferete de caracteres normais 
    println!("{}", indice_final_primeira_palavra(&teste));
    let teste = String::from("Atencao: aqui tem uma pegadinha!!!");
    println!("{}", indice_final_primeira_palavra(&teste));
}

fn indice_final_primeira_palavra(string: &str) -> usize {
    let bytes_a = string.as_bytes();
    
    let mut indice_atual: usize = 0;
    let mut indice = 0;
    let mut found_space = false;
    
    for char in bytes_a {
        if char == &b' ' {
            indice = indice_atual;
            found_space = true;
            break;  // We can stop at the first space
        }
        indice_atual += 1;
    }
    
    if found_space {
        indice
    } else {
        indice_atual  // Return the length of the string if no space is found
    }
}
