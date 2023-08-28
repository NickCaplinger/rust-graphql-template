pub mod doodad;
pub mod mutation_root;
pub mod query_root;

use self::query_root::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
