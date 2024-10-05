use std::env;
use balamut::filters;
use balamut::models;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=balamut=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "balamut=info");
    }
    pretty_env_logger::init();

    let db = models::open_db("data/balamut.sqlitedb");

    let api = filters::balamut(db);

    let routes = api.with(warp::log("balamut"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}