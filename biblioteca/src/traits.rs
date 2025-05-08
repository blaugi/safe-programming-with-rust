use uuid::Uuid;

pub trait Identificavel {
    fn id(&self) -> Uuid;
}
