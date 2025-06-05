import {computed} from "vue";
import * as interceptorWasm from "interceptor-wasm"

function getCookie(name: string): string | undefined {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);

    if (parts.length === 2)
        return parts.pop()?.split(';').shift();
}

export function saveToken(token: string) {
    document.cookie = `jwt=${token}; path=/;`;
}

export function getToken(name: string): string | undefined {
    let cookie = getCookie(name);
    if (cookie) {
        return `Bearer ${cookie}`
    }
    return undefined;
}

export function logout() {
    document.cookie = 'jwt=; expires=Thu, 01 Jan 1970 00:00:01 GMT;';
    alert('Logged out successfully.')
    location.href = '/'
}

export const isLoggedIn = computed(() => {
    return getCookie("jwt") !== undefined && getCookie("jwt")?.length > 0; // todo
})

export function toBlob(filename: string, bytes: Uint8Array | number[] | ArrayBuffer): Blob | null {
    if (bytes && bytes.length > 0) {
        if (!(bytes instanceof Uint8Array)) {
            bytes = new Uint8Array(bytes);
        }
        return new File([bytes], filename, {type: "image/jpeg"});
    }
    return null
}

export function toImageUrl(filename: string, bytes: Uint8Array | number[] | ArrayBuffer): string | null {
    let blob = toBlob(filename, bytes);
    if (blob == null) {
        return null
    }
    return URL.createObjectURL(blob)
}

export function revokeURL(url: string) {
    URL.revokeObjectURL(url);
}


let backendConfig = new interceptorWasm.WGPBackendConfig();
const BackendBaseURL = "http://localhost:6191";
export const NTorInitApi = `${BackendBaseURL}/ntor_init`;
export const LoginApi = `${BackendBaseURL}/login`;
export const RegisterApi = `${BackendBaseURL}/register`;
export const GetImageApi = `${BackendBaseURL}/images?id=`;
export const GetImagesApi = `${BackendBaseURL}/images`;
export const GetPoemApi = `${BackendBaseURL}/poems?id=`;
export const GetPoemsApi = `${BackendBaseURL}/poems`;
export const GetProfileApi = `${BackendBaseURL}/profile`;

export const wasmBackend = new interceptorWasm.WGPBackend(backendConfig);


