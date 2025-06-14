<template>
    <div class="login-container">
        <h2>Login</h2>
        <form @submit.prevent="handleLogin">
            <div class="form-group">
                <label for="username">Username:</label>
                <input type="text" id="username" v-model="username" required/>
            </div>
            <div class="form-group">
                <label for="password">Password:</label>
                <input type="password" id="password" v-model="password" required/>
            </div>
            <button type="submit">Login</button>
        </form>
    </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {saveToken, NTorInitApi, LoginApi} from "@/utils.js";
import * as interceptor_wasm from "interceptor-wasm"

const username = ref('');
const password = ref('');
let client: interceptor_wasm.Client;
let options = new interceptor_wasm.HttpRequestOptions()

onMounted(() => {
    client = new interceptor_wasm.Client()
    let init_session_msg = client.initialise_session();
    let init_session_response: interceptor_wasm.InitSessionResponse

    interceptor_wasm.http_post(NTorInitApi, {
        public_key: Array.from(init_session_msg.public_key())
    }).then(response => {
        options.headers = new Map<string, string>([
            ["Content-Type", "application/json"],
            ["nTor_session_id", response.get("session_id")]
        ]);

        init_session_response = new interceptor_wasm.InitSessionResponse(new Uint8Array(response.get("public_key")), new Uint8Array(response.get("t_hash")))
        let nTorCertificate = new interceptor_wasm.Certificate(new Uint8Array(response.get("static_public_key")), response.get("server_id"))

        let flag = client.handle_response_from_server(nTorCertificate, init_session_response)
        console.log("nTor flag:", flag)
    }).catch(err => {
        console.error(err)
    })
})

const handleLogin = () => {
    let body = {
        username: username.value,
        password: password.value,
    }
    const bodyBytes = new TextEncoder().encode(JSON.stringify(body));
    let encrypted = client.encrypt(bodyBytes);
    let encryptedBody = {
        nonce: Array.from(encrypted.nonce),
        encrypted: Array.from(encrypted.encrypted)
    }

    interceptor_wasm.http_post(LoginApi, encryptedBody, options)
        .then(response => {
            let login_res = client.decrypt(
                new Uint8Array(response.get("nonce")),
                new Uint8Array(response.get("encrypted"))
            )
            let deciphered = new TextDecoder().decode(login_res);
            console.log("deciphered:", deciphered)

            let data = JSON.parse(deciphered);
            let token = data.token || data["token"] || data.get("token");
            saveToken(token);
            alert(`Logged in as: ${username.value}`);
            location.href = '/profile';
        }).catch(err => {
        alert(`An error occurred while logging in. ${err}`);
    })
};
</script>

<style scoped>
.login-container {
    max-width: 400px;
    margin: 0 auto;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 8px;
    background-color: #f9f9f9;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

h2 {
    text-align: center;
    margin-bottom: 20px;
    color: #333;
}

.form-group {
    margin-bottom: 15px;
}

label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
    color: #555;
}

input {
    width: 100%;
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
    box-sizing: border-box;
}

button {
    width: 100%;
    padding: 10px;
    background-color: #007bff;
    color: #fff;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 16px;
}

button:hover {
    background-color: #0056b3;
}
</style>
