use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::{fs::read, io};

fn main() {
    let mut animais = load_from_json();
    menu();
    save_to_json(&animais);
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");
    input.trim().to_string()
}

fn menu() {
    loop{
        println!("1 - Listar Animais");
        println!("2 - Incluir Animal");
        println!("3 - Editar Animal");
        println!("4 - Excluir Animal");
        println!("5 - Sair");

        number = read_input();
        if number == 1 {
            listar_animais(&animais)
        } else if number == 2 {
            incluir_animal(&mut animais)
        } else if number == 3 {
            editar_animal(&mut animais)
        } else if number == 4 {
            excluir_animal(&mut animais)
        } else if number == 5 {
            println!("Tchau!");
            break;
        } else {
            println!("Escolha inválida!")
        }
    }
}


fn cria_animal(nome: String, tipo: String) -> Animal {
    animal = Animal(tipo, nome);
    animal
}

fn incluir_animal(animais : &animais){
    println!("Qual o tipo do animal?");
    tipo = read_input();
    println!("Qual o nome do animal?");
    nome = read_input();

    animal = cria_animal(nome, tipo);
    animais.push(animal)
}


fn editar_animal(animais : &animais){
    println!("Qual o id do animal?");
    let id:u32 = read_input();
    println!("Qual o novo tipo do animal?");
    tipo = read_input();
    println!("Qual o novo nome do animal?");
    nome = read_input();

    animais[id] = cria_animal(nome, tipo);
}

fn excluir_animal(animais : &animais){
    println!("Qual o id do animal?");
    let id:u32 = read_input();
    
    animais.remove(id)
}

#[derive(Serialize, Deserialize)]
struct Animal {
    tipo: String,
    nome: String,
}

fn save_to_json(animais: &Vec<Animais>) {
    let file = File::create("animais.json").expect("Erro ao criar arquivo");
    to_writer(file, &animais).expect("Erro ao salvar dados em JSON");
}

fn load_from_json() -> Vec<Animal> {
    let file = File::open("animais.json").expect("Erro ao abrir arquivo");
    from_reader(file).expect("Erro ao carregar dados do JSON")
}
