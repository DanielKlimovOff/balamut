use rusqlite::params;
use warp::{http::status::StatusCode, reply::Reply};

use crate::models::{Database, Player, PlayerUpdateForm};

pub async fn login(auth: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{auth}");
    Ok(warp::reply())
}

pub async fn player_update(nickname: String, player: PlayerUpdateForm, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db_response = db.lock().await.execute(
        "update players set email=?1, password_hash=?2, has_avatar=?3 where nickname=?4",
        params![
            &player.email,
            &player.password_hash,
            &player.has_avatar,
            &nickname,
    ]);
    match db_response {
        Ok(_) => Ok(warp::reply::with_status("UPDATED", StatusCode::OK).into_response()),
        Err(massage) => {
            println!("{massage}");
            Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR).into_response())
        },
        // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
    }
}

pub async fn player_nickname(nickname: String, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
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
    match db_response {
        Ok(player_info) => {
            Ok(warp::reply::json(&player_info).into_response())
        },
        Err(massage) => {
            println!("{massage}");
            Ok(warp::reply::with_status("ERROR_WITH_DB", StatusCode::INTERNAL_SERVER_ERROR).into_response())
        },
        // Error(error_massage) => Err(warp::reject::custom(error_on_db)),
    }
}

pub async fn player_register(new_player: Player, db: Database) -> Result<impl warp::Reply, warp::Rejection> {
    let db_response = db.lock().await.execute("insert into players (nickname, email, password_hash, has_avatar) values (?1, ?2, ?3, ?4);", params![&new_player.nickname, &new_player.email, &new_player.password_hash, &new_player.has_avatar]);
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