use std::env;
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

mod filters {
    use warp::Filter;

    use crate::{handlers, models::{Database, Player, PlayerRegisterForm}};

    pub fn balamut(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        player(db.clone())
        // player(db.clone()).or(warp::any()
        //     .and(with_db(db))
        //     .and_then(handlers::test_fn))
    }

    fn player(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
        warp::path("player").and(
            player_register(db.clone())
                .or(player_nickname(db))
        )
    }

    fn player_register(db: Database) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
        warp::path("register")
            .and(warp::path::end())
            .and(warp::post())
            .and(json_body_player_register_form())
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
            .and(json_body_player())
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

    fn json_body_player_register_form() -> impl Filter<Extract = (PlayerRegisterForm,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use hyper::Error;
    use warp::{http::status::StatusCode, reply::Reply};

    use crate::models::{Database, Player, PlayerRegisterForm};

    pub async fn test_fn(db: Database) -> Result<impl warp::Reply, warp::Rejection>  {
        Ok(warp::reply::html("site is good"))
    }

    pub async fn player_update(nickname: String, player: Player, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
        Ok(warp::reply::html(format!("player {} update", player.nickname)))
    }

    pub async fn player_nickname(nickname: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
        Ok(warp::reply::html(format!("player {} nickname", nickname)))
    }

    pub async fn player_register(new_player: PlayerRegisterForm, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
        let db_response = db.lock().await.execute("insert into players (nickname, email, password_hash) values (?1, ?2, ?3);", [&new_player.nickname, &new_player.email, &new_player.password_hash]);
        match db_response {
            Ok(_) => Ok(warp::reply::with_status("CREATED", StatusCode::CREATED)),
            Err(massage) => {
                println!("{massage}");
                Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR))
            },
            // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
        }
    }
    
    pub async fn player_delete(nickname: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
        let db_response = db.lock().await.execute("delete from players where nickname = ?1;", [&nickname]);
        match db_response {
            Ok(_) => Ok(warp::reply::with_status("DELETED", StatusCode::OK)),
            Err(massage) => {
                println!("{massage}");
                Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR))
            },
            // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
        }
    }
}

mod models {
    use serde_derive::{Deserialize, Serialize};
    use rusqlite::Connection;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Database = Arc<Mutex<Connection>>;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Player {
        pub nickname: String,
        pub email: String,
        pub password_hash: String,
        pub avatar_img: String,
        pub raing: u32,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PlayerRegisterForm {
        pub nickname: String,
        pub email: String,
        pub password_hash: String,
    }

    pub fn open_db(name_db: &str) -> Database {
        let db = Connection::open(name_db).unwrap();
        let db = Arc::new(Mutex::new(db));
        db
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use warp::http::StatusCode;
    use warp::test::request;

    use super::{filters, models};

    #[tokio::test]
    async fn test_player_register() {
        let db = models::open_db("data/balamut.sqlitedb");
        let api = filters::balamut(db);

        let names: [&str; 6] = ["daniel", "nikita", "ivan", "denis", "serega", "misha"];
        let mut register_forms = Vec::with_capacity(names.len());

        for i in 0..names.len() {
            let name = names[i];
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            register_forms.push(models::PlayerRegisterForm {
                nickname: String::from(name),
                email: format!("{name}@gmail.com"),
                password_hash: hasher.finish().to_string(),
            });
        }
        for register_form in &register_forms {
            let response = request()
            .method("POST")
            .path("/player/register")
            .json(register_form)
            .reply(&api)
            .await;

            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // for name in &names {
        //     let response = request()
        //     .method("DELETE")
        //     .path(&format!("/player/{name}/delete"))
        //     .reply(&api)
        //     .await;

        //     assert_eq!(response.status(), StatusCode::OK);
        // }
    }
}