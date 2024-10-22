let nickname = document.getElementById('nickname-textbox')
let password = document.getElementById('password-textbox')
let email = document.getElementById('email-textbox')
let enter_button = document.getElementById('enter-button')
enter_button.onclick = register

async function register() {
    let auth = btoa(nickname.value + ":" + password.value)
    console.log(nickname.value + ":" + password.value)
    console.log(auth)
    let new_player = {
        nickname: nickname.value,
        email: email.value ,
        password_hash: auth,
        has_avatar: false,
        rating: null,
    }
    const response = await fetch("http://localhost:3030/api/player/register", {
        method: "POST",
        headers: {
            'Content-Type': 'application/json; charset=utf-8'
        },
        body: JSON.stringify(new_player)

    });

    console.log(response)

    if (!response.ok) {
        // throw new Error(`Response status: ${response.status}`);
        return
    }

    window.location.replace("/");
}
