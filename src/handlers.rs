use rusqlite::params;
use warp::{http::status::StatusCode, reply::Reply};
use std::convert::Infallible;
use std::net::SocketAddr;
use rand::distributions::{Alphanumeric, DistString};
use crate::models::{self, Database, Player, PlayerUpdateForm, WhoAmIJSON};

pub async fn player_login(address: Option<SocketAddr>, agent: String, auth: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db_response = db.lock().await.query_row("select nickname from players where password_hash = ?1;",
        [&auth], |row| Ok(row.get(0)?));
    if let Err(err) = db_response {
        return Err(warp::reject());
    }
    let nickname: String = db_response.unwrap();

    let address = address.unwrap().to_string();
    let session_hash = create_new_session_for_player(db.clone(), &nickname, &address, &agent).await?;

    Ok(warp::reply::with_header(warp::reply(), "set-cookie", format!("session={session_hash}; path=/")))
}

pub async fn player_update(nickname: String, session_hash: String, player: PlayerUpdateForm, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    println!("i in update");
    println!("{session_hash}");
    let db_response = db.lock().await.query_row("select player from sessions where hash = ?1;",
    [&session_hash], |row| Ok(row.get(0)?));
    if let Err(err) = db_response {
        println!("db error 1");
        return Err(warp::reject());
    }
    let caller_nickname: String = db_response.unwrap();
    println!("{caller_nickname} {nickname}");
    
    if caller_nickname != nickname {
        println!("db error 2");
        return  Err(warp::reject());
    }

    let fields = ["email", "password_hash"];

    for field in fields {
        let mut sql_statment = "update players set ".to_string();
        sql_statment += match field {
            "email": player.email,
        }
        let db_response = db.lock().await.execute(
            format!("update players set email=?1 where nickname=?2", ),
            params![
                &player.email,
                &player.password_hash,
                // &player.has_avatar,
                &nickname,
        ]);
        match db_response {
            Ok(_) => 
            Err(massage) => {
                println!("{massage}");
                Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR).into_response())
            },
            // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
        }
    }

    Ok(warp::reply::with_status("UPDATED", StatusCode::OK).into_response())
}

pub async fn player_info(nickname: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    if models::RESERVED_NICKNAMES.contains(&nickname.as_str()) {
        return Err(warp::reject())
    }

    let db_response = db.lock().await.query_row("select nickname, email, has_avatar, rating from players where nickname = ?1;", [&nickname], |row| Ok(Player{
        nickname: row.get::<usize, String>(0)?,
        email: row.get::<usize, String>(1)?,
        password_hash: "---".to_string(),
        has_avatar: row.get::<usize, bool>(2)?,
        rating: match row.get::<usize, u32>(3) {
            Ok(avatar_name) => Some(avatar_name),
            Err(_) => None,
        },
    }));
    if let Err(err) = db_response {
        return Err(warp::reject());
    }

    let player_info = db_response.unwrap();
    Ok(warp::reply::json(&player_info).into_response())
}

pub async fn player_register(address: Option<SocketAddr>, agent: String, new_player: Player, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db_response = db.lock().await.execute("insert into players (nickname, email, password_hash, has_avatar) values (?1, ?2, ?3, ?4);",
        params![&new_player.nickname, &new_player.email, &new_player.password_hash, &new_player.has_avatar]);
    if let Err(err) = db_response {
        return Err(warp::reject());
    }

    let address = address.unwrap().to_string();
    let session_hash = create_new_session_for_player(db.clone(), &new_player.nickname, &address, &agent).await?;
    
    Ok(warp::reply::with_header(
        warp::reply::with_status(warp::reply(),
        StatusCode::CREATED),
        "set-cookie", format!("session={session_hash}; path=/")
    ))
}

pub async fn player_whoami(session_hash: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    println!("whoami hash {session_hash}");
    let db_response = db.lock().await.query_row("select player from sessions where hash = ?1;",
        [&session_hash], |row| Ok(row.get(0)?));
    if let Err(err) = db_response {
        return Err(warp::reject());
    }
    let nickname: String = db_response.unwrap();
    println!("whoami nickname {nickname}");


    let db_response = db.lock().await.query_row("select has_avatar from players where nickname = ?1;",
        [&nickname], |row| Ok(row.get(0)?));
    if let Err(err) = db_response {
        return Err(warp::reject());
    }
    let has_avatar: bool = db_response.unwrap();
    println!("whoami has_avatar {has_avatar}");


    Ok(warp::reply::json(&WhoAmIJSON {
        nickname,
        has_avatar,
    }))
}

pub async fn player_logout(session_hash: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db_response = db.lock().await.execute("update sessions set is_active=?1 where hash = ?2;",
        params![false, &session_hash]);
    if let Err(err) = db_response {
        return Err(warp::reject());
    }

    Ok(warp::reply::with_header(
        warp::reply::with_status(warp::reply(),
        StatusCode::CREATED),
        "set-cookie", format!("session=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT")
    ))
}

// pub async fn player_delete(nickname: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
//     let db_response = db.lock().await.execute("delete from players where nickname = ?1;", [&nickname]);
//     match db_response {
//         Ok(_) => Ok(warp::reply::with_status("DELETED", StatusCode::OK)),
//         Err(massage) => {
//             println!("{massage}");
//             Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR))
//         },
//         // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
//     }
// }

pub async fn recover(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::with_status(format!("some error :( info-{:?}", err), StatusCode::INTERNAL_SERVER_ERROR))
}

pub fn generate_hash() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

pub async fn create_new_session_for_player(db: Database, nickname: &str, address: &str, agent: &str) -> Result<String, warp::reject::Rejection> {
    let session_hash = generate_hash();
    let db_response = db.lock().await.execute("insert into sessions (hash, address, agent, player, is_active) values (?1, ?2, ?3, ?4, ?5);",
        params![&session_hash, address, agent, nickname, true]);
    if let Err(err) = db_response {
       return Err(warp::reject());
    }

    Ok(session_hash)
}