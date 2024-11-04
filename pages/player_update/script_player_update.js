let nickname = document.URL.split('/')
nickname = nickname[nickname.length - 2]
let nickname_label = document.getElementById('nickname-label')
let email_label = document.getElementById('email-label')
let avatar_image = document.getElementById('avatar-image')
let new_avatar_file = document.getElementById('new-avatar-file')
let email_textbox = document.getElementById('email-textbox')
let password_textbox = document.getElementById('password-textbox')
let enter_button = document.getElementById('enter-button')
let cancel_button = document.getElementById('cancel-button')
onload = init
enter_button.onclick = enter
cancel_button.onclick = cancel

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

    if (my_info.nickname != player_info.nickname) {
        window.location.href = '/'
        return
    }

    nickname_label.innerHTML = player_info.nickname
    email_label.innerHTML = player_info.email

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

async function enter() {
    let updated_player_info = {
        email: null,
        password_hash: null,
    }

    let need_to_update = false;

    let password = password_textbox.value
    if (password != '') {
        console.log('check')
        need_to_update = true
        let auth = btoa(nickname + ":" + password)
        updated_player_info.password_hash = auth
    }

    let email = email_textbox.value
    if (email != '') {
        console.log('check')
        need_to_update = true
        updated_player_info.email = email
    }

    console.log(password, email)
    if (need_to_update) {
        const response = await fetch("http://localhost:3030/api/player/" + nickname + '/update', {
            method: "PATCH",
            headers: {
                'Content-Type': 'application/json; charset=utf-8'
            },
            body: JSON.stringify(updated_player_info)
        });
    
        console.log(response)
    
        if (!response.ok) {
            // throw new Error(`Response status: ${response.status}`);
            return
        }
    }

    // let files = new_avatar_file.files
    // if (files.length > 0) {
    //     let avatar = files[0]
    //     // file.size
    // }

    // let formData = new FormData();
    // formData.append("avatar", photo);
    // const response = await fetch('http://localhost:3030/api/player/' + nickname + 'upload_image', {method: "POST", body: formData});
}

async function cancel() {
    window.location.href = '/'
}