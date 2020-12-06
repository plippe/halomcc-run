use hyper::{Body, Request, Response};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use std::sync::Arc;

use crate::graphql::context::Context;
use crate::graphql::query::Query;

#[derive(Clone)]
pub struct Service {
    pub context: Arc<Context>,
    pub root_node:
        Arc<RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>>,
}

impl Service {
    pub async fn graphiql(
        &self,
        _req: Request<Body>,
    ) -> Result<Response<Body>, hyper::error::Error> {
        juniper_hyper::graphiql("/graphql", None).await
    }

    pub async fn graphql(&self, req: Request<Body>) -> Result<Response<Body>, hyper::error::Error> {
        juniper_hyper::graphql(self.root_node.clone(), self.context.clone(), req).await
    }

    pub fn default() -> Self {
        Self {
            context: Arc::new(Context::default()),
            root_node: Arc::new(RootNode::new(
                Query,
                EmptyMutation::<Context>::new(),
                EmptySubscription::<Context>::new(),
            )),
        }
    }
}
