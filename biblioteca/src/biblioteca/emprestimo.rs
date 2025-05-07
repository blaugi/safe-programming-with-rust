use uuid::Uuid;
use chrono::{Duration, NaiveDate, Utc};
pub struct Emprestimo{
    id : Uuid,
    id_livro : Uuid,
    id_usuario : Uuid,
    datas : NaiveDate,
    status : StatusEmprestimo
}

pub enum StatusEmprestimo{
    Ativo,
    Devolvido
}

impl Emprestimo {
    pub fn new(id: Uuid, id_livro: Uuid, id_usuario: Uuid, data_emprestimo: NaiveDate) -> Self {
        Emprestimo {
            id: id,
            id_livro,
            id_usuario,
            datas: data_emprestimo,
            status: StatusEmprestimo::Ativo
        }
    }
}