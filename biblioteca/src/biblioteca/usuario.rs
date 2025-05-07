use uuid::Uuid;

pub struct Usuario{
    id : Uuid,
    nome : String
}

impl Usuario{
    pub fn new(id : Uuid, nome : String) -> Self{
        let new_user =  Usuario(id, nome);
        new_user
    }
}