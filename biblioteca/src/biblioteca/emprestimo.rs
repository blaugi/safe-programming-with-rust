use uuid::Uuid;
use chrono::{Duration, NaiveDate, Utc};
pub struct Emprestimo{
    id : Uuid,
    id_livro : Uuid,
    id_usuario : Uuid,
    data_emprestimo : NaiveDate,
    data_devolucao : NaiveDate,
    status : StatusEmprestimo
}

pub enum StatusEmprestimo{
    Ativo,
    Devolvido
}

impl Emprestimo {
    pub fn new(id: Uuid, id_livro: Uuid, id_usuario: Uuid) -> Self {
        let hoje =  Utc::now().date_naive();
        Emprestimo {
            id: id,
            id_livro,
            id_usuario,
            data_emprestimo :  hoje,
            data_devolucao : hoje + Duration::days(14),
            status: StatusEmprestimo::Ativo
        }
    }
}