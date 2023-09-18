pub mod doodad;
pub mod mutation_root;
pub mod query_root;

use self::{mutation_root::MutationRoot, query_root::QueryRoot};
use async_graphql::{EmptySubscription, Schema};

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
