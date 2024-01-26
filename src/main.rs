use std::fs::{read_to_string, File};
use std::io::Write;
use std::sync::Arc;
use actix_web::{HttpServer,web,App};
use mongodb::{Client,Database};
use mongodb::options::ClientOptions;
use server::r_auth::{register,login};
use server::s_state::State;
use server::s_config::Config;
use server::constants::HTML;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // read config
    let config_str: String = read_to_string("config.toml")
    .expect("err-reading-config");
    let config: Config = toml::from_str(&config_str)
    .expect("err-parsing-config");
    println!("{:?}", config);

    // build index
    let mut f: File = File::create("./public/index.html").expect("err-opening-index");
    f.write_all(
        HTML.replace("[TITLE]", &config.title)
            .replace("[KEYWORDS]", &config.keywords)
            .replace("[DESCRIPTION]", &config.description)
            .as_bytes()
    ).expect("err-writing-index");
    drop(f);

    // init state(db)
    let opt: ClientOptions = ClientOptions::parse(config.mongo_binding)
        .await.expect("err-parsing-mongo-connection-string");
    let db: Database = Client::with_options(opt)
        .expect("err-connecting-mongo")
        .database(&config.mongo_db);
    let state: State = State {db: Arc::new(db)};

    // init logging
    env_logger::init();

    // init server
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(state.clone()))
        // .route("/test", web::get().to(|| async {"/test"}))
        .service(
            web::scope("/rest")
            .service(register)
            .service(login)
        )
    })
    .bind(&config.server_rest)
    .expect("err-binding-server")
    .run().await
}