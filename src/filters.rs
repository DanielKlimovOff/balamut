use warp::Filter;

use crate::{handlers, models::{Database, Player, PlayerUpdateForm}};

pub fn balamut(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    api(db)
        .or(data())
        .or(pages())
    // player(db.clone())
    // player(db.clone()).or(warp::any()
    //     .and(with_db(db))
    //     .and_then(handlers::test_fn))
}

fn data() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    support_files()
        .or(images())
}

fn pages() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    home_page()
        .or(player_info_page())
        .or(login_page())
}

fn images() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("images").and(
        warp::fs::dir("./data/images/")
    )
}

fn home_page() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./pages/home/home.html"))
}

fn login_page() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::fs::file("./pages/login/login.html"))
}

fn player_info_page() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("player")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::get())
        .map(|_| ())
        .untuple_one()
        .and(warp::fs::file("./pages/player_info/player_info.html"))
}

fn support_files() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("pages").and(
        warp::fs::dir("./pages/")
    )
}

fn api(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("api")
        .and(player(db.clone())
            .or(login(db.clone()))
    )

}

fn player(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("player").and(
        player_register(db.clone())
            .or(player_nickname(db))
    )
}

fn login(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::exact("WWW-Authenticate", "Basic"))
        .and(warp::header("Authorization"))
        .and(with_db(db))
        .and_then(handlers::login)
}

fn player_register(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path("register")
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body_player())
        .and(with_db(db))
        .and_then(handlers::player_register)
}

fn player_nickname(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    player_info_by_nickname(db.clone())
        .or(player_update(db.clone()))
        .or(player_delete(db))
}

fn player_info_by_nickname(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path::param::<String>()
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::player_nickname)
}   

fn player_delete(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path::param::<String>()
    .and(warp::delete())
    .and(warp::path("delete"))
    .and(warp::path::end())
    .and(with_db(db.clone()))
    .and_then(handlers::player_delete)
}   

fn player_update(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
    warp::path::param::<String>()
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::patch())
        .and(json_body_player_update_form())
        .and(with_db(db))
        .and_then(handlers::player_update)
}

fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// fn with_nickname(nickname: String) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
//     warp::any().map(move || nickname.clone())
// }

fn json_body_player() -> impl Filter<Extract = (Player,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_player_update_form() -> impl Filter<Extract = (PlayerUpdateForm,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
