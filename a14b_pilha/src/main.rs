use std::fmt::Debug; 

#[derive(Debug)] 
struct PilhaLimitada<T> {
    elementos: Vec<T>,
    capacidade: usize,
}

impl<T> PilhaLimitada<T> {
    fn new(capacidade: usize) -> Self {
        PilhaLimitada {
            elementos: Vec::with_capacity(capacidade), // Pré-aloca espaço
            capacidade,
        }
    }

    fn push(&mut self, item: T) -> Result<(), String> {
        if self.is_full() {
            Err("A pilha está cheia".to_string())
        } else {
            self.elementos.push(item);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }

    fn is_full(&self) -> bool {
        self.elementos.len() == self.capacidade
    }

    fn capacity(&self) -> usize {
        self.capacidade
    }

    fn len(&self) -> usize {
        self.elementos.len()
    }
}

fn main() {
    println!("--- Demonstração com PilhaLimitada<i32> ---");
    let mut pilha_int = PilhaLimitada::<i32>::new(3);
    println!("Pilha criada com capacidade: {}", pilha_int.capacity());
    println!("Está vazia? {}", pilha_int.is_empty()); // true
    println!("Está cheia? {}", pilha_int.is_full());   // false

    match pilha_int.push(10) {
        Ok(_) => println!("Push 10: OK. Tamanho: {}", pilha_int.len()),
        Err(e) => println!("Push 10: Falhou! {}", e),
    }
    match pilha_int.push(20) {
        Ok(_) => println!("Push 20: OK. Tamanho: {}", pilha_int.len()),
        Err(e) => println!("Push 20: Falhou! {}", e),
    }
    println!("Está vazia? {}", pilha_int.is_empty()); // false

    if let Some(topo) = pilha_int.peek() {
        println!("Peek: {}", topo); // 20
    } else {
        println!("Peek: Pilha vazia.");
    }

    match pilha_int.push(30) {
        Ok(_) => println!("Push 30: OK. Tamanho: {}", pilha_int.len()),
        Err(e) => println!("Push 30: Falhou! {}", e),
    }
    println!("Está cheia? {}", pilha_int.is_full()); // true

    match pilha_int.push(40) {
        Ok(_) => println!("Push 40: OK. Tamanho: {}", pilha_int.len()),
        Err(e) => println!("Push 40: Falhou! {}", e), // "A pilha está cheia"
    }

    println!("Pilha atual (i32): {:?}", pilha_int);

    while let Some(valor) = pilha_int.pop() {
        println!("Pop: {}. Tamanho restante: {}", valor, pilha_int.len());
    }

    if let Some(valor) = pilha_int.pop() {
        println!("Pop extra: {}", valor);
    } else {
        println!("Pop extra: Pilha já estava vazia."); // Deve imprimir isso
    }
    println!("Está vazia? {}", pilha_int.is_empty()); // true
    println!("Está cheia? {}", pilha_int.is_full());   // false

    println!("\n--- Demonstração com PilhaLimitada<String> ---");
    let mut pilha_str = PilhaLimitada::<String>::new(2);
    println!("Pilha criada com capacidade: {}", pilha_str.capacity());

    pilha_str.push("Rust".to_string()).unwrap();
    println!("Push 'Rust': OK. Tamanho: {}", pilha_str.len());
    pilha_str.push("é".to_string()).unwrap();
    println!("Push 'é': OK. Tamanho: {}", pilha_str.len());

    println!("Está cheia? {}", pilha_str.is_full()); // true

    match pilha_str.push("seguro".to_string()) {
        Ok(_) => println!("Push 'seguro': OK"),
        Err(e) => println!("Push 'seguro': Falhou! {}", e), // "A pilha está cheia"
    }

    if let Some(topo) = pilha_str.peek() {
        println!("Peek: {}", topo); // "é"
    }

    println!("Pilha atual (String): {:?}", pilha_str);

    while let Some(valor) = pilha_str.pop() {
        println!("Pop: \"{}\". Tamanho restante: {}", valor, pilha_str.len());
    }

    println!("Está vazia? {}", pilha_str.is_empty()); // true
}
