use actix::prelude::*;
use actix_web::{dev::Server, web, HttpServer};
use sqlx::PgPool;

#[derive(Default)]
struct ExampleActor;

impl Actor for ExampleActor {
    type Context = Context<Self>;
}

fn init_app_without_actor(pool: &PgPool) -> anyhow::Result<Server> {
    let pool = pool.clone();

    let server = HttpServer::new(move || actix_web::App::new().app_data(pool.clone()))
        .bind(("0.0.0.0", 0))?
        .run();

    Ok(server)
}

fn init_app_with_actor(pool: &PgPool) -> anyhow::Result<Server> {
    let actor = ExampleActor::start_default();
    let pool = pool.clone();

    let server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(actor.clone()))
            .app_data(pool.clone())
    })
    .bind(("0.0.0.0", 0))?
    .run();

    Ok(server)
}

fn main() {
    println!("Run cargo test to see test failure");
}

#[sqlx::test]
async fn test_explodes(pool: PgPool) -> anyhow::Result<()> {
    init_app_with_actor(&pool)?;

    Ok(())
}

#[sqlx::test]
async fn test_does_not_explode(pool: PgPool) -> anyhow::Result<()> {
    init_app_without_actor(&pool)?;
    Ok(())
}
