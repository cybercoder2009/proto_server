use std::fs;
use std::sync::Arc;
use actix_web::{HttpServer,web,App};
use mongodb::{Client,Database};
use mongodb::options::ClientOptions;
use server::r_auth::{register,login};
use server::s_state::State;
use server::s_config::Config;

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register));
    cfg.route("/login", web::post().to(login));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config_str: String = fs::read_to_string("config.toml")
    .expect("Error on reading config.toml");
    let config: Config = toml::from_str(&config_str)
    .expect("Error on parsing config.toml");
    println!("{:?}", config);

    let opt: ClientOptions = ClientOptions::parse(config.mongo_binding)
        .await.expect("Error on mongo connection string");
    let db: Database = Client::with_options(opt)
        .expect("Error on connecting mongodb")
        .database(&config.mongo_db);
    let state: State = State {
        db: Arc::new(db)
    };

    env_logger::init();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(state.clone()))
        .service(web::scope("/rest")
        .configure(routes))
    })
    .bind(&config.server_rest)
    .expect("Error on rest binding")
    .run().await
}