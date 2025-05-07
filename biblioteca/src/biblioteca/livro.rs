use uuid::Uuid;

enum StatusLivro{
    Disponivel,
    Emprestado
}
pub struct Livro{
    id : Uuid,
    titulo : String,
    autor : String,
    ano : u16,
    status : StatusLivro
}


impl Livro{
    pub fn new(id: Uuid,  titulo: String, autor: String, ano: u16 ) -> Self {
        let new_livro = Livro {
            id: id,
            titulo: titulo,
            autor: autor,
            ano: ano,
            status: StatusLivro::Disponivel,
        };
        new_livro
    }
}