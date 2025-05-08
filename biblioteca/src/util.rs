use crate::traits::Identificavel;
use std::collections::HashMap;
use uuid::Uuid;

pub fn buscar_item_por_id<'a, 'b, T>(
    colecao: &'a HashMap<Uuid, T>,
    id: &'b Uuid
) -> Option<&'a T>
where
    T: Identificavel,
{
    colecao.get(id)
}
