let nickname = document.getElementById('nickname-textbox')
let password = document.getElementById('password-textbox')
let enter_button = document.getElementById('enter-button')
enter_button.onclick = login

async function login() {
    let auth = btoa(nickname.value + ":" + password.value)
    console.log(nickname.value + ":" + password.value)
    console.log(auth)
    const response = await fetch("http://localhost:3030/api/player/login", {
        method: "GET",
        headers: {
            "WWW-Authenticate": "Basic",
            "Authorization": auth,
        },
    });

    console.log(response)

    if (!response.ok) {
        // window.location.replace("/");
        return
        // throw new Error(`Response status: ${response.status}`);
    }
    window.location.replace("/");
}
