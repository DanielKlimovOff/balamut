let login_button = document.getElementById('login-button')
let find_player_button = document.getElementById('find-player-button')
let register_button = document.getElementById('register-button')
let player_nickname_textbox = document.getElementById('player-nickname-textbox')
login_button.onclick = login;
register_button.onclick = register;
find_player_button.onclick = find_player

function find_player() {
    window.location.href = "/player/" + player_nickname_textbox.value;
}

function login() {
    alert('login')
}

function register() {
    alert('register suka')
}