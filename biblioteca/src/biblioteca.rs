pub mod livro;
pub mod usuario;
pub mod emprestimo;

use crate::errors::ErroBiblioteca;
use crate::biblioteca::livro::{Livro, StatusLivro};
use crate::biblioteca::emprestimo::{Emprestimo, StatusEmprestimo};
use crate::biblioteca::usuario::Usuario;
use crate::util::buscar_item_por_id;

use chrono::NaiveDate;
use std::path::PathBuf;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct DadosPersistencia {
    pub mapa_livros: HashMap<Uuid, Livro>,
    pub mapa_usuarios: HashMap<Uuid, Usuario>,
    pub mapa_emprestimos: HashMap<Uuid, Emprestimo>,
}

pub struct Biblioteca {
    pub dados: DadosPersistencia,
    pub persistence_path: PathBuf,
}

impl Biblioteca {
    pub fn new(persistence_path: PathBuf) -> Self {
        // inicializa dados vazios e fixa o caminho de persistência
        Self {
            dados: DadosPersistencia {
                mapa_livros: HashMap::new(),
                mapa_usuarios: HashMap::new(),
                mapa_emprestimos: HashMap::new(),
            },
            persistence_path,
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
        if let Some(livro) = buscar_item_por_id(&self.dados.mapa_livros, &id_livro) {
            Ok(livro)
        } else {
            Err(ErroBiblioteca::LivroNaoEncontrado(id_livro))
        }
    }

    pub fn buscar_usuario_por_id(&self, id_usuario: Uuid) -> Result<&Usuario, ErroBiblioteca> {
        if let Some(usuario) = buscar_item_por_id(&self.dados.mapa_usuarios, &id_usuario) {
            Ok(usuario)
        } else {
            Err(ErroBiblioteca::UsuarioNaoEncontrado(id_usuario))
        }
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

    pub fn listar_usuarios(&self) -> Vec<&Usuario> {
        self.dados.mapa_usuarios.values().collect()
    }

    pub fn buscar_livro_por_titulo_exato(&self, titulo: &str) -> Option<&Livro> {
        self.dados.mapa_livros.values()
            .find(|livro| livro.titulo.eq_ignore_ascii_case(titulo))
    }

    pub fn buscar_usuario_por_nome_exato(&self, nome: &str) -> Option<&Usuario> {
        self.dados.mapa_usuarios.values()
            .find(|usuario| usuario.nome.eq_ignore_ascii_case(nome))
    }

    pub fn emprestar_livro_por_indices(&mut self, indice_usuario: usize, indice_livro: usize) -> Result<(), ErroBiblioteca> {
        let usuarios = self.listar_usuarios();
        let livros = self.listar_todos_livros();
        
        if indice_usuario >= usuarios.len() {
            return Err(ErroBiblioteca::EstadoInvalido("Índice de usuário inválido".to_string()));
        }
        
        if indice_livro >= livros.len() {
            return Err(ErroBiblioteca::EstadoInvalido("Índice de livro inválido".to_string()));
        }
        
        let id_usuario = usuarios[indice_usuario].id;
        let id_livro = livros[indice_livro].id;
        
        self.emprestar_livro(id_usuario, id_livro)?;
        Ok(())
    }

    pub fn devolver_livro_por_indice(&mut self, indice_livro: usize) -> Result<(), ErroBiblioteca> {
        let livros_emprestados: Vec<&Livro> = self.dados.mapa_livros.values()
            .filter(|l| l.status == StatusLivro::Emprestado)
            .collect();
            
        if indice_livro >= livros_emprestados.len() {
            return Err(ErroBiblioteca::EstadoInvalido("Índice de livro inválido".to_string()));
        }
        
        let id_livro = livros_emprestados[indice_livro].id;
        self.devolver_livro(id_livro)
    }

    pub fn remover_livro_por_indice(&mut self, indice_livro: usize) -> Result<(), ErroBiblioteca> {
        let livros = self.listar_todos_livros();
        
        if indice_livro >= livros.len() {
            return Err(ErroBiblioteca::EstadoInvalido("Índice de livro inválido".to_string()));
        }
        
        let id_livro = livros[indice_livro].id;
        self.remover_livro(id_livro)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biblioteca::livro::StatusLivro;
    use tempfile::tempdir;
    use std::path::PathBuf;

    #[test]
    fn test_new_biblioteca() {
        let path = PathBuf::from("test.json");
        let bib = Biblioteca::new(path.clone());
        assert!(bib.dados.mapa_livros.is_empty());
        assert!(bib.dados.mapa_usuarios.is_empty());
        assert!(bib.dados.mapa_emprestimos.is_empty());
        assert_eq!(bib.persistence_path, path);
    }

    #[test]
    fn test_adicionar_livro() {
        let path = PathBuf::from("test.json");
        let mut bib = Biblioteca::new(path);
        
        let id = bib.adicionar_livro(
            "O Senhor dos Anéis".to_string(),
            "J.R.R. Tolkien".to_string(),
            1954
        ).unwrap();
        
        assert_eq!(bib.dados.mapa_livros.len(), 1);
        
        let livro = bib.buscar_livro_por_id(id).unwrap();
        assert_eq!(livro.titulo, "O Senhor dos Anéis");
        assert_eq!(livro.autor, "J.R.R. Tolkien");
        assert_eq!(livro.ano, 1954);
        assert_eq!(livro.status, StatusLivro::Disponivel);
    }

    #[test]
    fn test_adicionar_usuario() {
        let path = PathBuf::from("test.json");
        let mut bib = Biblioteca::new(path);
        
        let id = bib.adicionar_usuario("João Silva".to_string()).unwrap();
        
        assert_eq!(bib.dados.mapa_usuarios.len(), 1);
        
        let usuario = bib.buscar_usuario_por_id(id).unwrap();
        assert_eq!(usuario.nome, "João Silva");
    }

    #[test]
    fn test_emprestar_e_devolver_livro() {
        let path = PathBuf::from("test.json");
        let mut bib = Biblioteca::new(path);
        
        // Adicionar um usuário e um livro
        let id_usuario = bib.adicionar_usuario("Maria Souza".to_string()).unwrap();
        let id_livro = bib.adicionar_livro(
            "Duna".to_string(), 
            "Frank Herbert".to_string(), 
            1965
        ).unwrap();
        
        // Emprestar o livro
        let id_emprestimo = bib.emprestar_livro(id_usuario, id_livro).unwrap();
        
        // Verificar se o livro está emprestado
        let livro = bib.buscar_livro_por_id(id_livro).unwrap();
        assert_eq!(livro.status, StatusLivro::Emprestado);
        
        // Verificar se o empréstimo foi registrado
        assert_eq!(bib.dados.mapa_emprestimos.len(), 1);
        
        // Devolver o livro
        bib.devolver_livro(id_livro).unwrap();
        
        // Verificar se o livro está disponível novamente
        let livro = bib.buscar_livro_por_id(id_livro).unwrap();
        assert_eq!(livro.status, StatusLivro::Disponivel);
        
        // Verificar que o empréstimo foi finalizado mas ainda existe
        let emprestimo = bib.dados.mapa_emprestimos.get(&id_emprestimo).unwrap();
        assert_eq!(emprestimo.status, StatusEmprestimo::Devolvido);
    }

    #[test]
    fn test_remover_livro() {
        let path = PathBuf::from("test.json");
        let mut bib = Biblioteca::new(path);
        
        // Adicionar um livro
        let id_livro = bib.adicionar_livro(
            "1984".to_string(),
            "George Orwell".to_string(),
            1949
        ).unwrap();
        
        // Remover o livro
        bib.remover_livro(id_livro).unwrap();
        
        // Verificar que o livro foi removido
        assert!(bib.dados.mapa_livros.is_empty());
        
        // Tentar remover novamente deve falhar
        let resultado = bib.remover_livro(id_livro);
        assert!(resultado.is_err());
    }

    #[test]
    fn test_buscar_livros_por_titulo() {
        let path = PathBuf::from("test.json");
        let mut bib = Biblioteca::new(path);
        
        // Adicionar alguns livros
        bib.adicionar_livro(
            "Harry Potter e a Pedra Filosofal".to_string(),
            "J.K. Rowling".to_string(),
            1997
        ).unwrap();
        
        bib.adicionar_livro(
            "Harry Potter e a Câmara Secreta".to_string(),
            "J.K. Rowling".to_string(),
            1998
        ).unwrap();
        
        bib.adicionar_livro(
            "O Hobbit".to_string(),
            "J.R.R. Tolkien".to_string(),
            1937
        ).unwrap();
        
        // Buscar por título
        let resultados = bib.buscar_livros_por_titulo("Harry");
        assert_eq!(resultados.len(), 2);
        
        let resultados = bib.buscar_livros_por_titulo("Hobbit");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].titulo, "O Hobbit");
        
        let resultados = bib.buscar_livros_por_titulo("Não Existe");
        assert_eq!(resultados.len(), 0);
    }

    #[test]
    fn test_persistencia() {
        // Criar um diretório temporário que será removido automaticamente
        let dir = tempdir().expect("Falha ao criar diretório temporário");
        let file_path = dir.path().join("biblioteca_test.json");
        
        // Criar uma biblioteca e adicionar dados
        {
            let mut bib = Biblioteca::new(file_path.clone());
            let id_usuario = bib.adicionar_usuario("Ana Pereira".to_string()).unwrap();
            let id_livro = bib.adicionar_livro(
                "A Metamorfose".to_string(),
                "Franz Kafka".to_string(),
                1915
            ).unwrap();
            
            // Salvar os dados
            bib.salvar().unwrap();
        }
        
        // Carregar os dados em uma nova instância
        {
            let bib = Biblioteca::carregar(&file_path).unwrap();
            
            // Verificar se os dados foram carregados corretamente
            assert_eq!(bib.dados.mapa_usuarios.len(), 1);
            assert_eq!(bib.dados.mapa_livros.len(), 1);
            
            // Verificar o conteúdo
            let usuario = bib.dados.mapa_usuarios.values().next().unwrap();
            assert_eq!(usuario.nome, "Ana Pereira");
            
            let livro = bib.dados.mapa_livros.values().next().unwrap();
            assert_eq!(livro.titulo, "A Metamorfose");
            assert_eq!(livro.autor, "Franz Kafka");
            assert_eq!(livro.ano, 1915);
        }
    }
}