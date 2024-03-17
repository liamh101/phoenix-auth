import {invoke} from "@tauri-apps/api/tauri";

export enum ResponseType {
    SUCCESS,
    FAILURE,
}

interface NewAccountResponse {
    response: ResponseType,
    message: string,
}

export interface Account {
    id: number,
    name: string,
}

interface AccountListResponse {
    response: ResponseType,
    accounts: Account[],
}

interface AccountDeleteResponse {
    response: ResponseType,
}

interface TokenResponse {
    response: ResponseType,
    token: string,
}

const INVALID_ACCOUNT_NAME = "Account already exists";
const INVALID_2FA_SECRET = "Invalid 2FA Secret";

export async function createNewAccount(name: string, secret: string): Promise<NewAccountResponse>
{
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

export async function getAllAccounts(filter: string): Promise<AccountListResponse>
{
    const result = JSON.parse(await invoke("get_all_accounts", {filter}));

    if (typeof result !== "object") {
        return {
            response: ResponseType.FAILURE,
            accounts: [],
        }
    }

    return {
        response: ResponseType.SUCCESS,
        accounts: result
    }
}

export async function deleteAccount(accountId: number): Promise<AccountDeleteResponse>
{
    const response = await invoke("delete_account", {accountId});

    if (response !== 'Success') {
        return {
            response: ResponseType.FAILURE,
        }
    }

    return {
        response: ResponseType.SUCCESS,
    }
}

export async function generateToken(accountId: number): Promise<TokenResponse>
{
    const response = await invoke("get_one_time_password_for_account", { account: accountId });

    if (typeof response !== 'string') {
        return {
            response: ResponseType.FAILURE,
            token: '',
        }
    }

    return {
        response: ResponseType.SUCCESS,
        token: response,
    }
}