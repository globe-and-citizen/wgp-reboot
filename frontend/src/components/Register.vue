<template>
    <div class="register-container">
        <h2>Register</h2>
        <form @submit.prevent="handleRegister">
            <div class="form-group">
                <label for="username">Username:</label>
                <input type="text" id="username" v-model="username" required/>
            </div>
            <div class="form-group">
                <label for="password">Password:</label>
                <input type="password" id="password" v-model="password" required/>
            </div>
            <button type="submit">Register</button>
        </form>
    </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {getToken, NTorInitApi, RegisterApi} from "@/utils.js";
import * as interceptor_wasm from "interceptor-wasm";

const username = ref('');
const password = ref('');
let client: interceptor_wasm.Client;
let requestOptions = new interceptor_wasm.HttpRequestOptions()

onMounted(() => {
    client = new interceptor_wasm.Client();
    let init_session_msg = client.initialise_session();
    let init_session_response: interceptor_wasm.InitSessionResponse

    interceptor_wasm.http_post(NTorInitApi, {
        public_key: Array.from(init_session_msg.public_key())
    }).then(response => {
        let token = getToken('jwt') || "";
        requestOptions.headers = new Map<string, string>([
            ["Content-Type", "application/json"],
            ["nTor_session_id", response.get("session_id")],
            ["Authorization", token]
        ]);

        init_session_response = new interceptor_wasm.InitSessionResponse(new Uint8Array(response.get("public_key")), new Uint8Array(response.get("t_hash")))
        let nTorCertificate = new interceptor_wasm.Certificate(new Uint8Array(response.get("static_public_key")), response.get("server_id"))

        let flag = client.handle_response_from_server(nTorCertificate, init_session_response)
        console.log("ntor flag:", flag)
    }).catch(err => {
        console.error(err)
    })
})

const handleRegister = () => {
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

    interceptor_wasm.http_post(RegisterApi, encryptedBody, requestOptions)
        .then(response => {
            alert("Registration successful! You can now log in.");
            location.href = '/';
        }).catch(() => {
        alert('An error occurred while registering.');
    })
};
</script>

<style scoped>
.register-container {
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
    background-color: #28a745;
    color: #fff;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 16px;
}

button:hover {
    background-color: #218838;
}
</style>
