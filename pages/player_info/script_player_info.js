let nickname = document.URL.split('/')
nickname = nickname[nickname.length - 1]
let nickname_label = document.getElementById('nickname')
let email_label = document.getElementById('email')
let rating_label = document.getElementById('rating')
let avatar_image = document.getElementById('avatar')

init()

async function init() {
    const response = await fetch("http://localhost:3030/api/player/" +  nickname, {
        method: "GET",
    });

    if (!response.ok) {
        window.location.replace("/");
        throw new Error(`Response status: ${response.status}`);
    }

    let player_info = await response.json();
    console.log(player_info);

    nickname_label.innerHTML = player_info.nickname
    email_label.innerHTML = player_info.email

    if (player_info.rating == null) {
        rating_label.innerHTML = 'нет рейтинга'
    } else {
        rating_label.innerHTML = player_info.rating
    }

    if (player_info.has_avatar) {
        avatar_image.src = 'http://localhost:3030/images/' + nickname + '.jpg'
    }
}