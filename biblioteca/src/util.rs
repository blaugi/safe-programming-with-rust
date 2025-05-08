use crate::traits::Identificavel;
use std::collections::HashMap;
use uuid::Uuid;

pub fn buscar_item_por_id<T>(
    colecao: &HashMap<Uuid, T>,
    id: &Uuid
) -> Option<&T>
where
    T: Identificavel,
{
    colecao.get(id)
}
