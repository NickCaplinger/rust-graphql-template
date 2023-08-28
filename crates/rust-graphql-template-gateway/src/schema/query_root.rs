use super::doodad::Doodad;
use crate::{
    context::{AuthContext, MyContext},
    error::Error,
};
use async_graphql::{Context, Object, ID};
use chrono::{DateTime, Utc};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the timestamp that the gateway has been up since.
    /// This can be sometimes be helpful when debugging.
    async fn up_since<'ctx>(&self, ctx: &Context<'ctx>) -> DateTime<Utc> {
        let my_context = ctx.data_unchecked::<MyContext>();

        my_context.up_since
    }

    async fn doodad<'ctx>(&self, ctx: &Context<'ctx>, id: ID) -> Result<Option<Doodad>, Error> {
        let _my_context = ctx.data_unchecked::<MyContext>();

        // This is where you'd try to fetch an object from your database and return it if it exists.
        // You'd store the reference to your database in the context.
        // Wrapping the return type in `Option` tells us that it can be `None` or not.
        // Wrapping the return type in `Result` tells us that this operation can fail for any reason.

        Ok(Some(Doodad {
            id,
            name: "Doodad Name #1".to_owned(),
        }))
    }

    async fn doodads<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Doodad>, Error> {
        let _my_context = ctx.data_unchecked::<MyContext>();

        // Again, this is where you'd access your database and return all of your doodads.
        // You'd want to implement pagination and filtering to this resolver as well,
        // but we'll cover that in a future commit.

        Err(Error::Internal)
    }

    async fn my_doodads<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Doodad>, Error> {
        let _my_context = ctx.data_unchecked::<MyContext>();
        let auth_context = ctx
            .data::<AuthContext>()
            .map_err(|_| Error::AuthenticationFailure)?;

        // This is where you'd inspect the authentication context for the user's bearer token.
        // After inspection, we could proceed to lookup the user's data if they've been properly authenticated
        // and authorized to access the data.

        if let Some(_bearer_token) = auth_context.bearer_token.as_deref() {
            // TODO Look up user and return real data
            Ok(vec![])
        } else {
            Err(Error::AuthenticationFailure)
        }
    }
}
