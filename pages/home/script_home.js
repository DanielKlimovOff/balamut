let login_button = document.getElementById('login-button')
let find_player_button = document.getElementById('find-player-button')
let register_button = document.getElementById('register-button')
let player_nickname_textbox = document.getElementById('player-nickname-textbox')
let player_nickname_label = document.getElementById('nickname-label')
let player_avatar_image = document.getElementById('avatar-image')
let player_info_div = document.getElementById('player-info-div')
login_button.onclick = login;
register_button.onclick = register;
find_player_button.onclick = find_player
player_info_div.onclick = player_info
let nickname = null
onload = init

async function init() {
    const response = await fetch("http://localhost:3030/api/player/whoami", {
        method: "GET",
    });

    if (!response.ok) {
        // window.location.replace("/");
        // throw new Error(`Response status: ${response.status}`);
        return
    }

    let player_info = await response.json();
    console.log(player_info);

    nickname = player_info.nickname
    player_nickname_label.innerHTML = nickname

    if (player_info.has_avatar) {
        player_avatar_image.src = 'http://localhost:3030/images/' + nickname + '.jpg'
        player_avatar_image.style.visibility = "visible";
    }
}

function find_player() {
    window.location.href = "/player/" + player_nickname_textbox.value;
}

function login() {
    window.location.href = "/player/login";
}

function register() {
    window.location.href = "/player/register";
}

function player_info() {
    if (nickname) {
        window.location.href = "/player/" + nickname;
    } else {
        window.location.href = "/player/login";
    }
}
