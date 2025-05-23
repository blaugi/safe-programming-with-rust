use crate::traits::Identificavel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
}

impl Usuario {
    pub fn new(nome: String) -> Self {
        Usuario {
            id: Uuid::new_v4(),
            nome,
        }
    }
}

impl Identificavel for Usuario {
    fn id(&self) -> Uuid {
        self.id
    }
}
