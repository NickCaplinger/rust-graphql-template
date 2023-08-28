use crate::{context::MyContext, error::Error};
use async_graphql::{Context, Object, ID};
use tracing::{debug, info};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_doodad(&self, ctx: &Context<'_>) -> Result<ID, Error> {
        let _my_context = ctx.data_unchecked::<MyContext>();

        debug!("Creating a doodad...");

        let id = "definitely_real_id";

        info!("Created a doodad with ID: {id}");

        // We must call `into` on string types to cast the string into an ID.
        // GraphQL IDs can be either strings or integer values.
        Ok(id.into())
    }
}
