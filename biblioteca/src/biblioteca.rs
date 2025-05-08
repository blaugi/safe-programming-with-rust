pub mod livro;
pub mod usuario;
pub mod emprestimo;
pub mod errors

use crate::errors::ErroBiblioteca;
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
        Self {
            dados: DadosPersistencia {
                mapa_livros: HashMap::new(),
                mapa_usuarios: HashMap::new(),
                mapa_emprestimos: HashMap::new(),
            },
            persistence_path: PathBuf::new(),
        }
    }

   pub fn adicionar_livro(&mut self, titulo: String, autor: String, ano : u16) -> Result<Uuid, ErroBiblioteca>{
        let id = Uuid::new_v4();
        let livro = Livro::new(id, titulo, autor, ano);
        self.dados.mapa_livros.insert(id, livro);
        Ok(id)
    } 

   pub fn adicionar_usuario(&mut self, nome : String) -> Result<Uuid, ErroBiblioteca> {
        let id = Uuid::new_v4();
        let user = Usuario::new(id, nome);
        self.dados.mapa_usuarios.insert(id, user);
        Ok(id)
    }

   pub fn adicionar_emprestimo(&mut self, id_usuario: Uuid, id_livro : Uuid, data_emprestimo: NaiveDate) -> Result<Uuid, ErroBiblioteca> {
        if !self.dados.mapa_livros.contains_key(&id_livro) {
            return Err(ErroBiblioteca::LivroNaoEncontrado(id_livro));
        }
        
        if !self.dados.mapa_usuarios.contains_key(&id_usuario) {
            return Err(ErroBiblioteca::UsuarioNaoEncontrado(id_usuario));
        }
        
        for emprestimo in self.dados.mapa_emprestimos.values() {
            if emprestimo.get_id_livro() == id_livro && !emprestimo.is_devolvido() {
                return Err(ErroBiblioteca::EstadoInvalido(format!("Livro já emprestado: {}", id_livro)));
            }
        }
        
        let id = Uuid::new_v4();
        let emprestimo = Emprestimo::new(id, id_livro, id_usuario);
        self.dados.mapa_emprestimos.insert(id, emprestimo);
        Ok(id)
    }
    
    pub fn remover_livro(&mut self, id_livro: Uuid) -> Result<(), ErroBiblioteca> {
        if !self.dados.mapa_livros.contains_key(&id_livro) {
            return Err(ErroBiblioteca::LivroNaoEncontrado(id_livro));
        }
        
        for emprestimo in self.dados.mapa_emprestimos.values() {
            if emprestimo.get_id_livro() == id_livro && !emprestimo.is_devolvido() {
                return Err(ErroBiblioteca::EstadoInvalido(String::from("Não é possível remover um livro emprestado")));
            }
        }
        
        self.dados.mapa_livros.remove(&id_livro);
        Ok(())
    }
    
    pub fn buscar_livro_por_id(&self, id_livro: Uuid) -> Result<&Livro, ErroBiblioteca> {
        self.dados.mapa_livros.get(&id_livro)
            .ok_or(ErroBiblioteca::LivroNaoEncontrado(id_livro))
    }
    
    pub fn buscar_livros_por_titulo(&self, titulo: &str) -> Vec<&Livro> {
        let titulo_lower = titulo.to_lowercase();
        self.dados.mapa_livros.values()
            .filter(|livro| livro.get_titulo().to_lowercase().contains(&titulo_lower))
            .collect()
    }
    
    pub fn buscar_livros_por_autor(&self, autor: &str) -> Vec<&Livro> {
        let autor_lower = autor.to_lowercase();
        self.dados.mapa_livros.values()
            .filter(|livro| livro.get_autor().to_lowercase().contains(&autor_lower))
            .collect()
    }
    
    pub fn listar_todos_livros(&self) -> Vec<&Livro> {
        self.dados.mapa_livros.values().collect()
    }
    
    pub fn listar_livros_disponiveis(&self) -> Vec<&Livro> {
        let livros_emprestados: Vec<Uuid> = self.dados.mapa_emprestimos.values()
            .filter(|emp| !emp.is_devolvido())
            .map(|emp| emp.get_id_livro())
            .collect();
            
        self.dados.mapa_livros.values()
            .filter(|livro| !livros_emprestados.contains(&livro.get_id()))
            .collect()
    }
    
    pub fn listar_livros_emprestados(&self) -> Vec<&Livro> {
        let livros_emprestados: Vec<Uuid> = self.dados.mapa_emprestimos.values()
            .filter(|emp| !emp.is_devolvido())
            .map(|emp| emp.get_id_livro())
            .collect();
            
        self.dados.mapa_livros.values()
            .filter(|livro| livros_emprestados.contains(&livro.get_id()))
            .collect()
    }
}