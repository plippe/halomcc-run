use std::sync::Arc;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Response, Server, StatusCode,
};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};
use serde::de::DeserializeOwned;
use serde::Deserialize;

fn csv<A: DeserializeOwned>(path: &str) -> Vec<A> {
    csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)
        .map(|mut it| it.deserialize::<A>().flatten().collect())
        .unwrap_or_else(|err| panic!("Failed parsing CSV file: {}, {}", path, err))
}

#[derive(Deserialize, GraphQLObject)]
struct Level {
    game_id: String,
    id: i32,
    name: String,
    par_time: i32,
}

struct LevelsDao;
impl LevelsDao {
    fn all() -> Vec<Level> {
        csv::<Level>("resources/levels.csv")
    }
}

#[derive(Deserialize, GraphQLObject)]
struct Game {
    id: i32,
    name: String,
}

struct GamesDao;
impl GamesDao {
    fn all() -> Vec<Game> {
        csv::<Game>("resources/games.csv")
    }
}

struct Context {
    games: Vec<Game>,
    levels: Vec<Level>,
}
impl juniper::Context for Context {}

struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn games(context: &Context) -> FieldResult<Vec<Game>> {
        Ok(context.games.clone())
    }

    fn game(context: &Context, id: i32) -> FieldResult<Option<Game>> {
        Ok(context.games.clone().into_iter().find(|it| it.id == id))
    }

    fn levels(context: &Context) -> FieldResult<Vec<Level>> {
        Ok(context.levels.clone())
    }
}

#[tokio::main]
async fn main() {
    let ip = [0, 0, 0, 0];
    let port = std::env::var("PORT")
        .map(|it| it.parse().unwrap())
        .unwrap_or(3000);
    let addr = (ip, port).into();

    let ctx = Arc::new(Context {
        games: csv::<Game>("resources/games.csv"),
        levels: csv::<Level>("resources/levels.csv"),
    });

    let root_node = Arc::new(RootNode::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    ));

    let new_service = make_service_fn(move |_| {
        let root_node = root_node.clone();
        let ctx = ctx.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let root_node = root_node.clone();
                let ctx = ctx.clone();
                async {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, ctx, req).await
                        }
                        _ => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
