use std::rc::Rc;
use std::cell::RefCell;

// Um funcionário pertence a um departamento.
// A referência ao departamento é forte! Este é o problema.
struct Funcionario {
    nome: String,
    departamento: Rc<Departamento>, 
}

impl Drop for Funcionario {
    fn drop(&mut self) {
        println!("Notificação de desligamento para {}.", self.nome);
    }
}

// Um departamento tem uma lista de seus funcionários (membros).
struct Departamento {
    nome: String,
    membros: RefCell<Vec<Rc<Funcionario>>>,
}

impl Drop for Departamento {
    fn drop(&mut self) {
        println!("Dissolvendo o departamento: {}!", self.nome);
    }
}

fn main() {
    println!("--- Início da Simulação ---");

    let depto_inovacao = Rc::new(Departamento {
        nome: "Inovação".to_string(),
        membros: RefCell::new(vec![]),
    });

    let alice = Rc::new(Funcionario {
        nome: "Alice".to_string(),
        departamento: Rc::clone(&depto_inovacao),
    });

    let bob = Rc::new(Funcionario {
        nome: "Bob".to_string(),
        departamento: Rc::clone(&depto_inovacao),
    });
    
    depto_inovacao.membros.borrow_mut().push(Rc::clone(&alice));
    depto_inovacao.membros.borrow_mut().push(Rc::clone(&bob));

    println!("Contagem de referências para o depto: {}", Rc::strong_count(&depto_inovacao));
    println!("--- Fim da Simulação ---");
    // A variável `depto_inovacao` sai de escopo, mas a memória não é liberada.
}