use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::fs::File;
use std::io;

fn main() {
    let mut animais = load_animals();
    menu(&mut animais);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");
    input.trim().to_string()
}

fn menu(animais: &mut Vec<Animal>) {
    loop {
        println!("");
        println!("1 - Listar Animais");
        println!("2 - Incluir Animal");
        println!("3 - Editar Animal");
        println!("4 - Excluir Animal");
        println!("5 - Sair");
        println!("");

        let number_str = read_input();
        let number = match number_str.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido.");
                continue; // Pular o resto do loop
            }
        };
        if number == 1 {
            listar_animais(animais)
        } else if number == 2 {
            incluir_animal(animais)
        } else if number == 3 {
            editar_animal(animais)
        } else if number == 4 {
            excluir_animal(animais)
        } else if number == 5 {
            println!("Tchau!");
            break;
        } else {
            println!("Escolha inválida!")
        }
    }
}

/// Cria um novo animal com o nome e tipo especificados.
/// 
/// # Parâmetros
/// - `nome`: O nome do animal a ser criado.
/// - `tipo`: O tipo ou espécie do animal.
/// 
/// # Retorno
/// - Uma nova instância da estrutura Animal.
fn cria_animal(nome: String, tipo: String) -> Animal {
    let animal = Animal { tipo, nome };
    animal
}

/// Adiciona um novo animal ao zoológico.
/// 
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais.
/// 
/// # Efeitos
/// - Solicita informações do animal ao usuário.
/// - Adiciona o novo animal ao vetor.
/// - Salva os dados atualizados em JSON.
fn incluir_animal(animais: &mut Vec<Animal>) {
    println!("Qual o tipo do animal?");
    let tipo = read_input();
    println!("Qual o nome do animal?");
    let nome = read_input();

    let animal = cria_animal(nome.clone(), tipo.clone());
    animais.push(animal);
    save_to_json(&animais);
    
    println!("✅ Animal '{}' do tipo '{}' adicionado com sucesso!", nome, tipo);
}

/// Edita as informações de um animal existente no zoológico.
/// 
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais.
/// 
/// # Efeitos
/// - Solicita o ID do animal a ser editado.
/// - Solicita novas informações para o animal.
/// - Atualiza o animal no vetor.
/// - Salva os dados atualizados em JSON.
fn editar_animal(animais: &mut Vec<Animal>) {
    println!("Qual o id do animal?");
    let id_str = read_input();
    let id = match id_str.parse::<usize>() {
        Ok(id) if id < animais.len() => id,
        Ok(_) => {
            println!("❌ ID inválido: animal não encontrado!");
            return;
        },
        Err(_) => {
            println!("❌ Por favor, digite um número válido para o ID.");
            return;
        }
    };
    
    println!("Qual o novo tipo do animal?");
    let tipo = read_input();
    println!("Qual o novo nome do animal?");
    let nome = read_input();

    let animal_antigo = &animais[id];
    animais[id] = cria_animal(nome.clone(), tipo.clone());
    save_to_json(&animais);
    
    println!("✅ Animal atualizado com sucesso!");
    println!("   Antes: Tipo: {}, Nome: {}", animal_antigo.tipo, animal_antigo.nome);
    println!("   Depois: Tipo: {}, Nome: {}", tipo, nome);
}

/// Remove um animal do zoológico pelo ID.
/// 
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais.
/// 
/// # Efeitos
/// - Solicita o ID do animal a ser removido.
/// - Remove o animal do vetor.
/// - Salva os dados atualizados em JSON.
fn excluir_animal(animais: &mut Vec<Animal>) {
    println!("Qual o id do animal?");
    let id_str = read_input();
    let id = match id_str.parse::<usize>() {
        Ok(id) if id < animais.len() => id,
        Ok(_) => {
            println!("❌ ID inválido: animal não encontrado!");
            return;
        },
        Err(_) => {
            println!("❌ Por favor, digite um número válido para o ID.");
            return;
        }
    };

    let animal_removido = animais.remove(id);
    save_to_json(&animais);
    
    println!("✅ Animal removido com sucesso: Tipo: {}, Nome: {}", 
             animal_removido.tipo, animal_removido.nome);
}

fn listar_animais(animais: &Vec<Animal>) {
    for (id, animal) in animais.iter().enumerate() {
        println!("Id: {}, Tipo: {}, Nome: {}", id, animal.tipo, animal.nome);
    }
}
#[derive(Serialize, Deserialize)]
struct Animal {
    tipo: String,
    nome: String,
}

fn save_to_json(animais: &Vec<Animal>) {
    let file = File::create("animais.json").expect("Erro ao criar arquivo");
    to_writer(file, &animais).expect("Erro ao salvar dados em JSON");
}

fn load_from_json(filename: &str) -> Result<Vec<Animal>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let animals = from_reader(file)?;
    Ok(animals)
}

fn load_animals() -> Vec<Animal> {
    match load_from_json("animais.json") {
        Ok(animais) => animais,
        Err(e) => {
            eprintln!("Erro ao carregar arquivo JSON: {}. Criando novo vetor.", e);
            Vec::<Animal>::new()
        }
    }
}
