let nickname = document.URL.split('/')
nickname = nickname[nickname.length - 1]
let nickname_label = document.getElementById('nickname-label')
let email_label = document.getElementById('email-label')
let rating_label = document.getElementById('rating-label')
let avatar_image = document.getElementById('avatar-image')
let logout_button = document.getElementById('logout-button')
let update_button = document.getElementById('update-button')
let hidden_div = document.getElementById('hidden-div')

onload = init
logout_button.onclick = logout
update_button.onclick = update

async function init() {
    let response = await fetch("http://localhost:3030/api/player/" +  nickname, {
        method: "GET",
    });

    if (!response.ok) {
        window.location.replace("/");
        throw new Error(`Response status: ${response.status}`);
    }

    let player_info = await response.json();
    console.log(player_info);

    response = await fetch("http://localhost:3030/api/player/whoami", {
        method: "GET",
    });

    if (!response.ok) {
        // window.location.replace("/");
        // throw new Error(`Response status: ${response.status}`);
        return
    }

    my_info = await response.json();

    if (my_info.nickname == player_info.nickname) {
        hidden_div.style.visibility = "visible";
    }

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

async function logout() {
    const response = await fetch("http://localhost:3030/api/player/logout", {
        method: "GET",
    });

    if (!response.ok) {
        // window.location.replace("/");
        // throw new Error(`Response status: ${response.status}`);
        return
    }

    window.location.href = '/'
}

async function update() {
    window.location.href = '/player/' + nickname + '/update'
}