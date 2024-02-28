import {invoke} from "@tauri-apps/api/tauri";

enum ResponseType {
    SUCCESS,
    FAILURE,
}

interface NewAccountResponse {
    response: ResponseType,
    message: string,
}

const INVALID_ACCOUNT_NAME = "Account already exists";
const INVALID_2FA_SECRET = "Invalid 2FA Secret";

export async function createNewAccount(name: string, secret: string): Promise<NewAccountResponse> {
    const response = await invoke("create_new_account", {name, secret});

    if (typeof response !== 'string') {
        return {
            response: ResponseType.FAILURE,
            message: 'Unknown Error',
        }
    }

    if (response.includes(INVALID_ACCOUNT_NAME) || response === INVALID_2FA_SECRET) {
        return {
            response: ResponseType.FAILURE,
            message: response,
        }
    }

    return {
        response: ResponseType.SUCCESS,
        message: response,
    }

}