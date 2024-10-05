use balamut::models;
use balamut::filters;
use std::hash::{DefaultHasher, Hash, Hasher};

use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_player_register() {
    // cargo test test_player_register -- --exact
    let db = models::open_db("data/balamut.sqlitedb");
    let api = filters::balamut(db);

    let names: [&str; 6] = ["daniel", "nikita", "ivan", "denis", "serega", "misha"];
    let mut register_forms = Vec::with_capacity(names.len());

    for i in 0..names.len() {
        let name = names[i];
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        register_forms.push(models::Player {
            nickname: String::from(name),
            email: format!("{name}@gmail.com"),
            password_hash: hasher.finish().to_string(),
            has_avatar: false,
            rating: None,
        });
    }
    for register_form in &register_forms {
        let response = request()
        .method("POST")
        .path("/api/player/register")
        .json(register_form)
        .reply(&api)
        .await;

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}

#[tokio::test]
async fn test_clear_players() {
    // cargo test test_clear_players -- --exact
    let db = models::open_db("data/balamut.sqlitedb");
    let api = filters::balamut(db.clone());

    let mut players_nicknames: Vec<String> = Vec::new();
    {
        let db_clone = db.lock().await;
        let mut statesment = db_clone.prepare("select nickname from players").unwrap();
        let rows = statesment.query_map([], |row| row.get(0)).unwrap();
        
        for nickname in rows {
            players_nicknames.push(nickname.unwrap());
        }
    }   

    for nickname in &players_nicknames {
        let response = request()
        .method("DELETE")
        .path(&format!("/api/player/{nickname}/delete"))
        .reply(&api)
        .await;

        assert_eq!(response.status(), StatusCode::OK);
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
