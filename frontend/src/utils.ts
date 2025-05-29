import {computed} from "vue";
import * as interceptorWasm from "interceptor-wasm"

export function getCookie(name: string): string | undefined {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);

    if (parts.length === 2)
        return parts.pop()?.split(';').shift();
}

export function logout() {
    document.cookie = 'jwt=; expires=Thu, 01 Jan 1970 00:00:01 GMT;';
    alert('Logged out successfully.')
    location.href = '/'
}

export const isLoggedIn = computed(() => {
    return getCookie("jwt") !== undefined && getCookie("jwt")?.length > 0; // todo
})

export function convertToImageUrl(bytes: Uint8Array | number[] | ArrayBuffer): string | null {
    let imageUrl = null;
    if (bytes && bytes.length > 0) {
        if (!(bytes instanceof Uint8Array)) {
            bytes = new Uint8Array(bytes);
        }
        const blob = new Blob([bytes], { type: 'image/jpeg' });
        imageUrl = URL.createObjectURL(blob);
    }
    return imageUrl
}


let backendConfig = new interceptorWasm.BackendConfig();
backendConfig.base_url = "http://localhost:6191";
backendConfig.login = "/login";
backendConfig.register = "/register";
backendConfig.get_image_path = "/images?id={}";
backendConfig.get_images_path = "/images";
backendConfig.get_poem_path = "/poems?id={}";
backendConfig.get_poems_path = "/poems";
backendConfig.get_profile_path = "/profile";

export const wasmBackend = new interceptorWasm.Backend(backendConfig);


