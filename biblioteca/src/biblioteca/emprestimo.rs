use uuid::Uuid;
use chrono::{Duration, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use crate::traits::Identificavel;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum StatusEmprestimo {
    Ativo,
    Devolvido,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emprestimo {
    pub id: Uuid,
    pub id_livro: Uuid,
    pub id_usuario: Uuid,
    pub data_emprestimo: NaiveDate,
    pub data_devolucao: NaiveDate,
    pub status: StatusEmprestimo,
}

impl Emprestimo {
    pub fn new(id_livro: Uuid, id_usuario: Uuid) -> Self {
        let hoje = Utc::now().date_naive();
        Emprestimo {
            id: Uuid::new_v4(),
            id_livro,
            id_usuario,
            data_emprestimo: hoje,
            data_devolucao: hoje + Duration::days(14),
            status: StatusEmprestimo::Ativo,
        }
    }

    pub fn is_devolvido(&self) -> bool {
        matches!(self.status, StatusEmprestimo::Devolvido)
    }

    pub fn finalizar(&mut self) {
        self.status = StatusEmprestimo::Devolvido;
    }
}

impl Identificavel for Emprestimo {
    fn id(&self) -> Uuid {
        self.id
    }
}