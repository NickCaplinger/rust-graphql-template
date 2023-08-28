use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Doodad {
    pub id: ID,
    pub name: String,
}
