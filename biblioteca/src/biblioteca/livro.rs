use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::errors::ErroBiblioteca;
use crate::traits::Identificavel;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum StatusLivro {
    Disponivel,
    Emprestado,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Livro {
    pub id: Uuid,
    pub titulo: String,
    pub autor: String,
    pub ano: u16,
    pub status: StatusLivro,
}

impl Livro {
    pub fn new(titulo: String, autor: String, ano: u16) -> Self {
        Livro {
            id: Uuid::new_v4(),
            titulo,
            autor,
            ano,
            status: StatusLivro::Disponivel,
        }
    }

    pub fn emprestar(&mut self) -> Result<(), ErroBiblioteca> {
        if self.status == StatusLivro::Disponivel {
            self.status = StatusLivro::Emprestado;
            Ok(())
        } else {
            Err(ErroBiblioteca::EstadoInvalido("Livro já emprestado".to_string()))
        }
    }

    pub fn devolver(&mut self) -> Result<(), ErroBiblioteca> {
        if self.status == StatusLivro::Emprestado {
            self.status = StatusLivro::Disponivel;
            Ok(())
        } else {
            Err(ErroBiblioteca::EstadoInvalido("Livro não está emprestado".to_string()))
        }
    }
}

impl Identificavel for Livro {
    fn id(&self) -> Uuid {
        self.id
    }
}