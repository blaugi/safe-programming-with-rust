pub mod livro;
pub mod usuario;
pub mod emprestimo;

use crate::errors::ErroBiblioteca;
use crate::biblioteca::livro::{Livro, StatusLivro};
use crate::biblioteca::emprestimo::{Emprestimo, StatusEmprestimo};
use crate::biblioteca::usuario::Usuario;

use chrono::{NaiveDate};
use std::path::PathBuf;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
struct DadosPersistencia {
    pub mapa_livros: HashMap<Uuid, Livro>,
    pub mapa_usuarios: HashMap<Uuid, Usuario>,
    pub mapa_emprestimos: HashMap<Uuid, Emprestimo>,
}

pub struct Biblioteca {
    pub dados: DadosPersistencia,
    pub persistence_path: PathBuf,
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

    pub fn carregar(path: &PathBuf) -> Result<Self, ErroBiblioteca> {
        if !path.exists() {
            return Ok(Self {
                dados: DadosPersistencia {
                    mapa_livros: HashMap::new(),
                    mapa_usuarios: HashMap::new(),
                    mapa_emprestimos: HashMap::new(),
                },
                persistence_path: path.clone(),
            });
        }
        let file = File::open(path).map_err(|e| ErroBiblioteca::ErroPersistencia(e.to_string()))?;
        let reader = BufReader::new(file);
        let dados: DadosPersistencia = serde_json::from_reader(reader)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(e.to_string()))?;
        Ok(Self {
            dados,
            persistence_path: path.clone(),
        })
    }

    pub fn salvar(&self) -> Result<(), ErroBiblioteca> {
        let file = File::create(&self.persistence_path)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(e.to_string()))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.dados)
            .map_err(|e| ErroBiblioteca::ErroPersistencia(e.to_string()))
    }

    pub fn adicionar_livro(&mut self, titulo: String, autor: String, ano: u16) -> Result<Uuid, ErroBiblioteca> {
        let livro = Livro::new(titulo, autor, ano);
        let id = livro.id;
        self.dados.mapa_livros.insert(id, livro);
        Ok(id)
    }

    pub fn adicionar_usuario(&mut self, nome: String) -> Result<Uuid, ErroBiblioteca> {
        let user = Usuario::new(nome);
        let id = user.id;
        self.dados.mapa_usuarios.insert(id, user);
        Ok(id)
    }

    pub fn emprestar_livro(&mut self, id_usuario: Uuid, id_livro: Uuid) -> Result<Uuid, ErroBiblioteca> {
        let livro = self.dados.mapa_livros.get_mut(&id_livro)
            .ok_or(ErroBiblioteca::LivroNaoEncontrado(id_livro))?;
        let _usuario = self.dados.mapa_usuarios.get(&id_usuario)
            .ok_or(ErroBiblioteca::UsuarioNaoEncontrado(id_usuario))?;

        livro.emprestar()?;

        let emprestimo = Emprestimo::new(id_livro, id_usuario);
        let id_emprestimo = emprestimo.id;
        self.dados.mapa_emprestimos.insert(id_emprestimo, emprestimo);
        Ok(id_emprestimo)
    }

    pub fn devolver_livro(&mut self, id_livro: Uuid) -> Result<(), ErroBiblioteca> {
        let mut emprestimo_id = None;
        for (id, emprestimo) in self.dados.mapa_emprestimos.iter_mut() {
            if emprestimo.id_livro == id_livro && emprestimo.status == StatusEmprestimo::Ativo {
                emprestimo.finalizar();
                emprestimo_id = Some(*id);
                break;
            }
        }
        if emprestimo_id.is_none() {
            return Err(ErroBiblioteca::EmprestimoNaoEncontrado(id_livro));
        }
        let livro = self.dados.mapa_livros.get_mut(&id_livro)
            .ok_or(ErroBiblioteca::LivroNaoEncontrado(id_livro))?;
        livro.devolver()?;
        Ok(())
    }

    pub fn remover_livro(&mut self, id_livro: Uuid) -> Result<(), ErroBiblioteca> {
        if !self.dados.mapa_livros.contains_key(&id_livro) {
            return Err(ErroBiblioteca::LivroNaoEncontrado(id_livro));
        }
        for emprestimo in self.dados.mapa_emprestimos.values() {
            if emprestimo.id_livro == id_livro && !emprestimo.is_devolvido() {
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
            .filter(|livro| livro.titulo.to_lowercase().contains(&titulo_lower))
            .collect()
    }

    pub fn buscar_livros_por_autor(&self, autor: &str) -> Vec<&Livro> {
        let autor_lower = autor.to_lowercase();
        self.dados.mapa_livros.values()
            .filter(|livro| livro.autor.to_lowercase().contains(&autor_lower))
            .collect()
    }

    pub fn listar_todos_livros(&self) -> Vec<&Livro> {
        self.dados.mapa_livros.values().collect()
    }

    pub fn listar_livros_por_status(&self, status: StatusLivro) -> Vec<&Livro> {
        self.dados.mapa_livros.values()
            .filter(|livro| livro.status == status)
            .collect()
    }

    pub fn listar_emprestimos_ativos(&self) -> Vec<&Emprestimo> {
        self.dados.mapa_emprestimos.values()
            .filter(|e| e.status == StatusEmprestimo::Ativo)
            .collect()
    }

    pub fn listar_emprestimos_ativos_usuario(&self, id_usuario: Uuid) -> Vec<&Emprestimo> {
        self.dados.mapa_emprestimos.values()
            .filter(|e| e.status == StatusEmprestimo::Ativo && e.id_usuario == id_usuario)
            .collect()
    }
}