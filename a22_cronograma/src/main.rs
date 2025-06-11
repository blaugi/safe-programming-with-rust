use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

use dialoguer::{theme::ColorfulTheme, Select, Input};

const DADOS_PATH: &str = "cronograma_dados.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AtividadeSerializable {
    nome: String,
    descricao: String,
    responsavel: Option<String>,
}

#[derive(Debug, Clone)]
struct Atividade {
    nome: String,
    descricao: String,
    responsavel: Option<Rc<String>>,
}

struct Cronograma {
    atividades: Vec<Atividade>,
    responsaveis_cache: HashMap<String, Rc<String>>,
}

impl Cronograma {
    fn novo() -> Self {
        Self {
            atividades: Vec::new(),
            responsaveis_cache: HashMap::new(),
        }
    }

    fn carregar<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(atividades_serializadas) = serde_json::from_str::<Vec<AtividadeSerializable>>(&data) {
                let mut cronograma = Self::novo();
                for atividade in atividades_serializadas {
                    let responsavel = atividade.responsavel.map(|r| Rc::new(r));
                    cronograma.atividades.push(Atividade {
                        nome: atividade.nome,
                        descricao: atividade.descricao,
                        responsavel,
                    });
                }
                return cronograma;
            }
        }
        Self::novo()
    }

    fn salvar<P: AsRef<Path>>(&self, path: P) {
        let atividades_serializadas: Vec<AtividadeSerializable> = self.atividades.iter().map(|a| {
            AtividadeSerializable {
                nome: a.nome.clone(),
                descricao: a.descricao.clone(),
                responsavel: a.responsavel.as_ref().map(|r| Rc::try_unwrap(r.clone()).unwrap_or_default()),
            }
        }).collect();

        let json = serde_json::to_string(&atividades_serializadas).expect("Erro ao serializar atividades");
        fs::write(path, json).expect("Erro ao salvar cronograma");
    }

    fn adicionar_atividade(&mut self, nome: String, descricao: String, optional_resp: Option<Rc<String>>) {
        let responsavel = optional_resp.map(|resp| self.obter_ou_criar_responsavel(&resp));
        let atividade = Atividade {
            nome,
            descricao,
            responsavel,
        };
        self.atividades.push(atividade);
    }

    fn editar_atividade(&mut self, idx: usize, novo_nome: String, nova_desc: String) {
        if let Some(atividade) = self.atividades.get_mut(idx) {
            atividade.nome = novo_nome;
            atividade.descricao = nova_desc;
        } else {
            println!("Atividade não encontrada.");
        }
    }

    fn excluir_atividade(&mut self, idx: usize) {
        if idx < self.atividades.len() {
            self.atividades.remove(idx);
        } else {
            println!("Atividade não encontrada.");
        }
    }

    fn editar_responsavel(&mut self, idx: usize, novo_resp: Option<Rc<String>>) {
        if let Some(atividade) = self.atividades.get_mut(idx) {
            atividade.responsavel = novo_resp;
        } else {
            println!("Atividade não encontrada.");
        }
    }

    fn listar_responsaveis(&self) -> Vec<Rc<String>> {
        let mut responsaveis = Vec::new();
        for atividade in &self.atividades {
            if let Some(resp) = &atividade.responsavel {
                if !responsaveis.iter().any(|r| Rc::ptr_eq(r, resp)) {
                    responsaveis.push(Rc::clone(resp));
                }
            }
        }
        responsaveis
    }

    fn obter_ou_criar_responsavel(&mut self, nome: &str) -> Rc<String> {
        if let Some(resp) = self.responsaveis_cache.get(nome) {
            return Rc::clone(resp);
        }
        let novo_resp = Rc::new(nome.to_string());
        self.responsaveis_cache.insert(nome.to_string(), Rc::clone(&novo_resp));
        novo_resp
    }
}

// Funções de interface CLI
fn menu_principal(cronograma: &mut Cronograma) {
    loop {
        println!("\n=== Cronograma ===");
        if cronograma.atividades.is_empty() {
            println!("Nenhuma atividade cadastrada.");
        } else {
            for (i, atividade) in cronograma.atividades.iter().enumerate() {
                let resp = atividade.responsavel.as_ref().map(|r| r.as_str()).unwrap_or("-");
                println!("{}. {} [{}]", i + 1, atividade.nome, resp);
            }
        }

        let opcoes = vec![
            "Adicionar atividade",
            "Ver/Editar atividade",
            "Sair"
        ];
        let escolha = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Escolha uma opção")
            .items(&opcoes)
            .default(0)
            .interact()
            .unwrap();

        match escolha {
            0 => {
                let nome: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Nome da atividade")
                    .interact_text()
                    .unwrap();
                let descricao: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Descrição")
                    .interact_text()
                    .unwrap();

                // Responsável opcional
                let responsaveis = cronograma.listar_responsaveis();
                let mut resp_opcoes: Vec<String> = responsaveis.iter().map(|r| r.to_string()).collect();
                resp_opcoes.push("Novo responsável".to_string());
                resp_opcoes.push("Sem responsável".to_string());

                let resp_idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Escolha um responsável")
                    .items(&resp_opcoes)
                    .default(resp_opcoes.len() - 1)
                    .interact()
                    .unwrap();

                let optional_resp = match resp_idx.cmp(&responsaveis.len()) {
                    std::cmp::Ordering::Less => Some(Rc::clone(&responsaveis[resp_idx])),
                    std::cmp::Ordering::Equal => {
                        // Novo responsável
                        let novo_resp: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Nome do novo responsável")
                            .interact_text()
                            .unwrap();
                        Some(cronograma.obter_ou_criar_responsavel(&novo_resp))
                    }
                    std::cmp::Ordering::Greater => None,
                };

                cronograma.adicionar_atividade(nome, descricao, optional_resp);
                println!("Atividade adicionada!");
            }
            1 => {
                if cronograma.atividades.is_empty() {
                    println!("Nenhuma atividade para editar.");
                    continue;
                }
                let nomes: Vec<String> = cronograma.atividades.iter()
                    .map(|a| a.nome.clone())
                    .collect();
                let idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Selecione uma atividade")
                    .items(&nomes)
                    .default(0)
                    .interact()
                    .unwrap();
                menu_atividade(cronograma, idx);
            }
            2 => {
                break;
            }
            _ => {}
        }
    }
}

fn menu_atividade(cronograma: &mut Cronograma, idx: usize) {
    loop {
        let atividade = &cronograma.atividades[idx];
        println!("\n--- Atividade ---");
        println!("Nome: {}", atividade.nome);
        println!("Descrição: {}", atividade.descricao);
        let resp = atividade.responsavel.as_ref().map(|r| r.as_str()).unwrap_or("-");
        println!("Responsável: {}", resp);

        let opcoes = vec![
            "Editar nome/descrição",
            "Editar responsável",
            "Excluir atividade",
            "Voltar"
        ];
        let escolha = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("O que deseja fazer?")
            .items(&opcoes)
            .default(0)
            .interact()
            .unwrap();

        match escolha {
            0 => {
                let novo_nome: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Novo nome")
                    .with_initial_text(&atividade.nome)
                    .interact_text()
                    .unwrap();
                let nova_desc: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Nova descrição")
                    .with_initial_text(&atividade.descricao)
                    .interact_text()
                    .unwrap();
                cronograma.editar_atividade(idx, novo_nome, nova_desc);
                println!("Atividade editada!");
            }
            1 => {
                menu_editar_responsavel(cronograma, idx);
            }
            2 => {
                let confirm = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Tem certeza que deseja excluir?")
                    .items(&["Não", "Sim"])
                    .default(0)
                    .interact()
                    .unwrap();
                if confirm == 1 {
                    cronograma.excluir_atividade(idx);
                    println!("Atividade excluída!");
                    break;
                }
            }
            3 => break,
            _ => {}
        }
    }
}

fn menu_editar_responsavel(cronograma: &mut Cronograma, idx: usize) {
    let responsaveis = cronograma.listar_responsaveis();
    let mut opcoes: Vec<String> = responsaveis.iter().map(|r| r.to_string()).collect();
    opcoes.push("Novo responsável".to_string());
    opcoes.push("Limpar responsável".to_string());
    opcoes.push("Voltar".to_string());

    let escolha = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha um responsável")
        .items(&opcoes)
        .default(opcoes.len() - 1)
        .interact()
        .unwrap();

    if escolha < responsaveis.len() {
        cronograma.editar_responsavel(idx, Some(Rc::clone(&responsaveis[escolha])));
        println!("Responsável atualizado!");
    } else if escolha == responsaveis.len() {
        // Novo responsável
        let novo_resp: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Nome do novo responsável")
            .interact_text()
            .unwrap();
        let rc_resp = cronograma.obter_ou_criar_responsavel(&novo_resp);
        cronograma.editar_responsavel(idx, Some(rc_resp));
        println!("Responsável atualizado!");
    } else if escolha == responsaveis.len() + 1 {
        // Limpar responsável
        cronograma.editar_responsavel(idx, None);
        println!("Responsável removido!");
    }
    // Se for "Voltar", não faz nada
}


fn main() {
    // Carrega cronograma do arquivo
    let mut cronograma = Cronograma::carregar(DADOS_PATH);

    // Loop principal do menu
    menu_principal(&mut cronograma);

    // Salva ao sair
    cronograma.salvar(DADOS_PATH);
}
