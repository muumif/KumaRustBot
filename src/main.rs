extern crate env_logger;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serenity::{
    model::{prelude::{UserId}},
    http::Http
};

#[macro_use]
extern crate dotenv_codegen;

const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
const WEBSERVER_PORT: &str = dotenv!("WEBSERVER_PORT");

const USER_ID: UserId = UserId(438081715576242176); // Get this from .env

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let webserver_port = WEBSERVER_PORT.parse::<u16>().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(default_route)
            .service(kuma_route)
    })
    .bind(("127.0.0.1", webserver_port))?
    .run()
    .await.unwrap();

    Ok(())
}

#[get("/")]
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}


#[post("/kuma")]
async fn kuma_route(req_body: String) -> impl Responder {
    let parsed = json::parse(&req_body).unwrap();

    let http = Http::new(DISCORD_TOKEN);

    let dm_channel = USER_ID.create_dm_channel(&http).await.unwrap();
    if let Err(why) = dm_channel.say(&http, &parsed["msg"]).await {
        println!("Error sending message: {:?}", why);
    }

    HttpResponse::Ok().body("Ok")
}
