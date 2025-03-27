fn primeira_palavra(s: &str) -> &str {
    // Se a string estiver vazia, retorna uma string vazia
    if s.is_empty() {
        return "";
    }

    // Se a string começar com um espaço, retorna uma string vazia
    if s.starts_with(" ") {
        return "";
    }

    // Converte a string em um array de bytes
    let bytes = s.as_bytes();

    // Itera sobre os bytes da string, procurando por um espaço
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // Se não encontrar espaço, retorna a string inteira
    &s[..]
}

fn main() {
    let s1 = "hello world";
    let s2 = " rust";
    let s3 = "rust";
    let s4 = "";
    let s5 = "   ";

    println!("String original: '{}', tamanho: {}", s1, s1.len());
    println!("Primeira palavra: '{}'", primeira_palavra(s1));

    println!("String original: '{}', tamanho: {}", s2, s2.len());
    println!("Primeira palavra: '{}'", primeira_palavra(s2));

    println!("String original: '{}', tamanho: {}", s3, s3.len());
    println!("Primeira palavra: '{}'", primeira_palavra(s3));

    println!("String original: '{}', tamanho: {}", s4, s4.len());
    println!("Primeira palavra: '{}'", primeira_palavra(s4));

    println!("String original: '{}', tamanho: {}", s5, s5.len());
    println!("Primeira palavra: '{}'", primeira_palavra(s5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(primeira_palavra("hello world"), "hello");
        assert_eq!(primeira_palavra(" rust"), "");
        assert_eq!(primeira_palavra("rust"), "rust");
        assert_eq!(primeira_palavra(""), "");
        assert_eq!(primeira_palavra("   "), "");
    }
}