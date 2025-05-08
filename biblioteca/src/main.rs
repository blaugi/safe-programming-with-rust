mod biblioteca;
mod errors;
mod traits;
mod util;

use crate::biblioteca::Biblioteca;
use crate::biblioteca::livro::StatusLivro;
use crate::errors::ErroBiblioteca;
use std::path::PathBuf;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <arquivo_de_dados.json>", args[0]);
        std::process::exit(1);
    }
    let path = PathBuf::from(&args[1]);
    let mut bib = match Biblioteca::carregar(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Erro ao carregar dados: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        println!("\n=== Biblioteca Menu ===");
        println!("1) Adicionar usuário");
        println!("2) Adicionar livro");
        println!("3) Emprestar livro");
        println!("4) Devolver livro");
        println!("5) Remover livro");
        println!("6) Buscar livro pelo título");
        println!("7) Buscar usuário pelo nome");
        println!("8) Listar todos os livros");
        println!("9) Listar usuários");
        println!("0) Salvar e sair");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let opt = read_input().trim().parse::<u8>().unwrap_or(255);
        match opt {
            1 => {
                print!("Nome do usuário: ");
                io::stdout().flush().unwrap();
                let nome = read_input().trim().to_string();
                match bib.adicionar_usuario(nome) {
                    Ok(_) => println!("Usuário criado com sucesso!"),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            2 => {
                print!("Título: "); io::stdout().flush().unwrap();
                let titulo = read_input().trim().to_string();
                print!("Autor: "); io::stdout().flush().unwrap();
                let autor = read_input().trim().to_string();
                print!("Ano: "); io::stdout().flush().unwrap();
                let ano = read_input().trim().parse::<u16>().unwrap_or(0);
                match bib.adicionar_livro(titulo, autor, ano) {
                    Ok(_) => println!("Livro adicionado com sucesso!"),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            3 => {
                // Listar usuários numerados
                let usuarios = bib.listar_usuarios();
                if usuarios.is_empty() {
                    println!("Não há usuários cadastrados.");
                    continue;
                }
                println!("Selecione o usuário:");
                for (i, usuario) in usuarios.iter().enumerate() {
                    println!("{}) {}", i+1, usuario.nome);
                }
                print!("Escolha um número: "); io::stdout().flush().unwrap();
                let indice_usuario = match read_input().trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= usuarios.len() => num - 1,
                    _ => {
                        println!("Número inválido!");
                        continue;
                    }
                };

                // Listar livros disponíveis numerados
                let livros = bib.listar_todos_livros().into_iter()
                    .filter(|l| l.status == StatusLivro::Disponivel)
                    .collect::<Vec<_>>();
                
                if livros.is_empty() {
                    println!("Não há livros disponíveis para empréstimo.");
                    continue;
                }
                
                println!("Selecione o livro:");
                for (i, livro) in livros.iter().enumerate() {
                    println!("{}) {} - {} ({})", i+1, livro.titulo, livro.autor, livro.ano);
                }
                print!("Escolha um número: "); io::stdout().flush().unwrap();
                let indice_livro = match read_input().trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= livros.len() => num - 1,
                    _ => {
                        println!("Número inválido!");
                        continue;
                    }
                };

                // Realizar empréstimo
                let id_usuario = usuarios[indice_usuario].id;
                let id_livro = livros[indice_livro].id;
                let titulo_livro = livros[indice_livro].titulo.clone();
                let nome_usuario = usuarios[indice_usuario].nome.clone();
                
                match bib.emprestar_livro(id_usuario, id_livro) {
                    Ok(_) => println!("Livro '{}' emprestado com sucesso para '{}'!", 
                              titulo_livro, nome_usuario),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            4 => {
                // Listar livros emprestados
                let emprestados: Vec<&_> = bib.listar_todos_livros().into_iter()
                    .filter(|l| l.status == StatusLivro::Emprestado)
                    .collect();
                
                if emprestados.is_empty() {
                    println!("Não há livros emprestados no momento.");
                    continue;
                }
                
                println!("Selecione o livro a devolver:");
                for (i, livro) in emprestados.iter().enumerate() {
                    println!("{}) {} - {} ({})", i+1, livro.titulo, livro.autor, livro.ano);
                }
                
                print!("Escolha um número: "); io::stdout().flush().unwrap();
                let indice = match read_input().trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= emprestados.len() => num - 1,
                    _ => {
                        println!("Número inválido!");
                        continue;
                    }
                };
                
                let id_livro = emprestados[indice].id;
                let titulo_livro = emprestados[indice].titulo.clone();
                
                match bib.devolver_livro(id_livro) {
                    Ok(_) => println!("Livro '{}' devolvido com sucesso!", titulo_livro),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            5 => {
                // Listar todos os livros
                let livros = bib.listar_todos_livros();
                if livros.is_empty() {                    println!("Não há livros cadastrados.");
                    continue;
                }
                
                println!("Selecione o livro a remover:");
                for (i, livro) in livros.iter().enumerate() {
                    let status = match livro.status {
                        StatusLivro::Disponivel => "Disponível",
                        StatusLivro::Emprestado => "Emprestado",
                    };
                    println!("{}) {} - {} ({}) [{}]", i+1, livro.titulo, livro.autor, livro.ano, status);
                }
                
                print!("Escolha um número: "); io::stdout().flush().unwrap();
                let indice = match read_input().trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= livros.len() => num - 1,
                    _ => {
                        println!("Número inválido!");
                        continue;
                    }
                };
                
                let id_livro = livros[indice].id;
                let titulo_livro = livros[indice].titulo.clone();
                
                match bib.remover_livro(id_livro) {
                    Ok(_) => println!("Livro '{}' removido com sucesso!", titulo_livro),
                    Err(e) => eprintln!("Erro: {}", e),
                }
            }
            6 => {
                print!("Digite o título do livro: "); io::stdout().flush().unwrap();
                let titulo = read_input().trim().to_string();
                let resultados = bib.buscar_livros_por_titulo(&titulo);
                
                if resultados.is_empty() {
                    println!("Nenhum livro encontrado com esse título.");
                } else {
                    println!("Livros encontrados:");
                    for (i, livro) in resultados.iter().enumerate() {
                        let status = match livro.status {
                            StatusLivro::Disponivel => "Disponível",
                            StatusLivro::Emprestado => "Emprestado",
                        };
                        println!("{}) {} - {} ({}) [{}]", i+1, livro.titulo, livro.autor, livro.ano, status);
                    }
                }
            }
            7 => {
                print!("Digite o nome do usuário: "); io::stdout().flush().unwrap();
                let nome = read_input().trim().to_string();
                
                // Vamos usar uma abordagem simples para buscar por parte do nome
                let resultados: Vec<&_> = bib.listar_usuarios().into_iter()
                    .filter(|u| u.nome.to_lowercase().contains(&nome.to_lowercase()))
                    .collect();
                
                if resultados.is_empty() {
                    println!("Nenhum usuário encontrado com esse nome.");
                } else {
                    println!("Usuários encontrados:");
                    for (i, usuario) in resultados.iter().enumerate() {
                        println!("{}) {}", i+1, usuario.nome);
                    }
                }
            }
            8 => {
                let livros = bib.listar_todos_livros();
                if livros.is_empty() {
                    println!("Não há livros cadastrados.");
                } else {
                    println!("Lista de livros:");
                    for (i, livro) in livros.iter().enumerate() {
                        let status = match livro.status {
                            StatusLivro::Disponivel => "Disponível",
                            StatusLivro::Emprestado => "Emprestado",
                        };
                        println!("{}) {} - {} ({}) [{}]", i+1, livro.titulo, livro.autor, livro.ano, status);
                    }
                }
            }
            9 => {
                let usuarios = bib.listar_usuarios();
                if usuarios.is_empty() {
                    println!("Não há usuários cadastrados.");
                } else {
                    println!("Lista de usuários:");
                    for (i, usuario) in usuarios.iter().enumerate() {
                        println!("{}) {}", i+1, usuario.nome);
                    }
                }
            }
            0 => {
                match bib.salvar() {
                    Ok(_) => println!("Dados salvos. Saindo..."),
                    Err(e) => eprintln!("Erro ao salvar: {}", e),
                }
                break;
            }
            _ => println!("Opção inválida"),
        }    }
}

fn read_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}
