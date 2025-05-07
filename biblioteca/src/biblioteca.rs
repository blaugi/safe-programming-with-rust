pub mod livro;
pub mod usuario;
pub mod emprestimo;

use crate::biblioteca::livro::Livro;
use crate::biblioteca::emprestimo::Emprestimo;
use crate::biblioteca::usuario::Usuario;

use chrono::{Duration, NaiveDate, Utc};
use std::path::PathBuf;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Biblioteca {
    dados : DadosPersistencia,
    persistence_path : PathBuf
}

struct DadosPersistencia {
    pub mapa_livros: HashMap<Uuid, Livro>,
    pub mapa_usuarios: HashMap<Uuid, usuario::Usuario>,
    pub mapa_emprestimos: HashMap<Uuid, emprestimo::Emprestimo>,
}

impl Biblioteca {
    pub fn new() -> Self {
        {
          todo!(); 
        }
    }

   pub fn adicionar_livro(&mut self, titulo: String, autor: String, ano : u16) -> Uuid {
    let id = Uuid::new_v4();
    let livro = Livro::new(id, titulo, autor, ano);
    self.dados.mapa_livros.insert(id, livro);
    id
} 
   pub fn adicionar_usuario(&mut self, nome : String) -> Uuid {
    let id = Uuid::new_v4();
    let user = Usuario::new(id, nome);
    self.dados.mapa_usuarios.insert(id, user);
    id
}
   pub fn adicionar_emprestimo(&mut self, id_usuario: Uuid, id_livro : Uuid, data_emprestimo: NaiveDate) -> Uuid {
    let id = Uuid::new_v4();
    let emprestimo= Emprestimo::new(id, id_livro, id_usuario, data_emprestimo);
    self.dados.mapa_emprestimos.insert(id, emprestimo);
    id
}
} 