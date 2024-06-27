import {invoke} from "@tauri-apps/api/tauri";
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from "@tauri-apps/api/fs";

export enum ResponseType {
    SUCCESS,
    FAILURE,
}

export enum AccountAlgorithm {
    AUTODETECT = "",
    SHA1 = "SHA1",
    SHA256 = "SHA256",
    SHA512 = "SHA512",
}

interface NewAccountResponse {
    response: ResponseType,
    message: string,
}

export interface Account {
    id: number,
    name: string,
}

export interface DraftAccount {
    import: boolean,
    name: string,
    secret: string,
    totp_step: number,
    otp_digits: number,
    algorithm: AccountAlgorithm,
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

interface OptUrlResponse {
    response: ResponseType,
    account: DraftAccount,
}

const INVALID_ACCOUNT_NAME = "Account already exists";
const INVALID_2FA_SECRET = "Invalid 2FA Secret";

export async function createNewAccount(name: string, secret: string, digits: number, step: number, algorithm: AccountAlgorithm): Promise<NewAccountResponse>
{
    const response = await invoke("create_new_account", {name, secret, digits, step, algorithm});

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

export async function parseOptUrl(url: string): Promise<OptUrlResponse>
{
    const response = JSON.parse(await invoke("parse_otp_url", {otpUrl: url}));

    if (typeof response !== "object") {
        return {
            response: ResponseType.FAILURE,
            account: {
                import: true,
                name: 'Failure',
                secret: '',
                otp_digits: 0,
                totp_step: 0,
                algorithm: AccountAlgorithm.AUTODETECT,
            }
        }
    }

    response.import = true;
    return {
        response: ResponseType.SUCCESS,
        account: response,
    }
}

export async function exportAccounts() {
    const contents = await invoke("export_accounts_to_wa");
    const filePath = await save({
        filters: [{
            name: 'export',
            extensions: ['wa']
        }]
    });

    // Now we can write the file to the disk
    // @ts-expect-error Save returns string but return type isn't correctly typed
    await writeTextFile(filePath, contents);
}