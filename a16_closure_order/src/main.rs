use std::{cmp::Ordering, fmt::Debug, mem::swap, vec};

// 1. Defina o enum OrdemComparacao aqui
#[derive(Debug, PartialEq)]
pub enum OrdemComparacao {
    Menor,
    Igual,
    Maior,
}

// 2. Implemente a função ordenacao_bolha_custom aqui
fn ordenacao_bolha_custom<T, F>(vec_ref: &mut Vec<T>, comparador: F)
where F: Fn(&T, &T) -> OrdemComparacao{
    for i in 0..vec_ref.len() {
        let mut swapped = false;
        // Os ultimos i elementos já estão em ordem
        for j in 0..vec_ref.len()-1-i{
            let result = comparador(&vec_ref[j], &vec_ref[j+1]);
            match result {
               OrdemComparacao::Maior =>{
                    vec_ref.swap(j, j+1);
                    swapped = true;
               }
               _ => {}
            }
        }
        if !swapped{
            break;
        }
    }
}




// Estrutura para o teste com structs
#[derive(Debug, Clone)]
struct Item {
    valor: f32,
    nome: String,
}

fn main() {
    // --- Teste com Inteiros ---
    let mut numeros: Vec<i32> = vec![64, 34, -25, 12, 22, 11, -90, 0, 12];
    println!("Números Originais: {:?}", numeros);
    // A função ordenacao_bolha_custom precisa ser implementada acima
     ordenacao_bolha_custom(&mut numeros, |a: &i32, b: &i32| {
         if a > b {
             OrdemComparacao::Maior
         } else if a < b {
             OrdemComparacao::Menor
         } else {
             OrdemComparacao::Igual
         }
    }); 
    println!("Números Ordenados (após implementação): {:?}", numeros);
    println!("---");

    // --- Teste com Strings ---
    let mut palavras: Vec<String> = vec![
        "Rust".to_string(), "é".to_string(), "poderoso".to_string(), "e".to_string(),
        "seguro".to_string(), "!".to_string(), "é".to_string(), "".to_string(),
    ];
    println!("Palavras Originais: {:?}", palavras);
     ordenacao_bolha_custom(&mut palavras, |a: &String, b: &String| {
        let comparison = a.cmp(b);
        match comparison {
           Ordering::Greater => OrdemComparacao::Maior,
           Ordering::Less => OrdemComparacao::Menor,
           Ordering::Equal => OrdemComparacao::Igual 
        }

     });
    println!("Palavras Ordenadas (após implementação): {:?}", palavras);
    println!("---");

    // --- Teste com Structs (Item) ---
    let mut itens: Vec<Item> = vec![
        Item { valor: 10.5, nome: "Banana".to_string() },
        Item { valor: 5.2, nome: "Maçã".to_string() },
        Item { valor: 5.2, nome: "Uva".to_string() },
        Item { valor: 10.5, nome: "Abacaxi".to_string() },
        Item { valor: 8.0, nome: "Laranja".to_string() },
    ];
    println!("Itens Originais: {:?}", itens);
    ordenacao_bolha_custom(&mut itens, |a: &Item, b: &Item| {
          if a.valor > b.valor {
            OrdemComparacao::Maior
         } else if a.valor < b.valor {
            OrdemComparacao::Menor
         } else {
            let comparison = a.nome.cmp(&b.nome);
            match comparison {
            Ordering::Greater => OrdemComparacao::Maior,
            Ordering::Less => OrdemComparacao::Menor,
            Ordering::Equal => OrdemComparacao::Igual 
        }            
         }       

    });
    println!("Itens Ordenados (após implementação): {:?}", itens);
    println!("---");

}