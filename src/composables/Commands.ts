import {invoke} from "@tauri-apps/api/core";
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from "@tauri-apps/plugin-fs";

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

export interface EditableAccount {
    id: number,
    name: string,
    secret: string,
    totp_step: number,
    otp_digits: number,
    algorithm: AccountAlgorithm,
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

interface EditableAccountResponse {
    response: ResponseType,
    account: EditableAccount,
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

interface SyncValidationResponse {
    response: ResponseType,
    message: string
}

interface SyncAccount {
    id: number|null,
    username: string,
    password: string,
    url: string,
}

interface SyncAccountResponse {
    response: ResponseType,
    syncAccount: SyncAccount,
}

interface ExistingSyncAccountResponse {
    response: ResponseType,
    syncAccount: SyncAccount|null,
}

export interface SyncLog {
    id: number,
    log: string,
    log_type: SyncLogType,
    timestamp: number,
}

export enum SyncLogType {
    ERROR = "ERROR",
}

interface SyncLogResponse {
    response: ResponseType,
    logs: SyncLog[],
}

export enum THEME_MODES {
    DEFAULT,
    DARK,
    LIGHT,
}

export interface Setting {
    id: number,
    theme: THEME_MODES,
}

interface SettingResponse {
    response: ResponseType,
    settings: Setting,
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

export async function editExistingAccount(id: number, name: string, digits: number, step: number, algorithm: AccountAlgorithm) {
    const response = await invoke("edit_account", {id, name, digits, step, algorithm});

    if (typeof response !== 'string') {
        return {
            response: ResponseType.FAILURE,
            message: 'Unknown Error',
        }
    }

    if (response.includes('Invalid')) {
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

export async function getEditableAccount(accountId: number): Promise<EditableAccountResponse>
{
    const result = JSON.parse(await invoke("get_editable_account", {accountId}));

    if (typeof result !== "object") {
        return {
            response: ResponseType.FAILURE,
            account: {
                id: 0,
                name: '',
                secret: '',
                totp_step: 0,
                otp_digits: 0,
                algorithm: AccountAlgorithm.AUTODETECT,
            },
        }
    }

    return {
        response: ResponseType.SUCCESS,
        account: result,
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
            name: 'WA Export',
            extensions: ['.wa.txt']
        }]
    });

    // Now we can write the file to the disk
    // @ts-expect-error Save returns string but return type isn't correctly typed
    await writeTextFile(filePath, contents);
}

export async function validateSyncAccount(host: string, username: string, password: string): Promise<SyncValidationResponse>
{
    try {
        await invoke("validate_sync_account", {host, username, password});

        return {
            response: ResponseType.SUCCESS,
            message: 'Successfully Validated Account',
        }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (e: any) {
        return {
            response: ResponseType.FAILURE,
            message: e,
        }
    }
}

export async function saveSyncAccount(syncAccount: SyncAccount): Promise<SyncAccountResponse>
{
    const response: SyncAccount = await invoke("save_sync_account", {host: syncAccount.url, username: syncAccount.username, password: syncAccount.password});

    return {
        response: ResponseType.SUCCESS,
        syncAccount: response,
    }
}

export async function getExistingAccount(): Promise<ExistingSyncAccountResponse>
{
    try {
        const response: SyncAccount = await invoke("get_existing_sync_account");

        return {
            response: ResponseType.SUCCESS,
            syncAccount: response,
        }
    } catch (e) {
        console.error(e)
        return {
            response: ResponseType.FAILURE,
            syncAccount: null,
        }
    }
}

export async function getSyncLogs(): Promise<SyncLogResponse>
{
    const result = JSON.parse(await invoke("get_sync_logs"));

    if (typeof result !== "object") {
        return {
            response: ResponseType.FAILURE,
            logs: [],
        }
    }

    return {
        response: ResponseType.SUCCESS,
        logs: result
    }
}

export async function attemptSyncAccounts(): Promise<boolean>
{
    await invoke("attempt_sync_with_remote");

    return true;
}

export async function getSettings(): Promise<SettingResponse>
{
    const result: Setting = await invoke("get_settings");
    result.theme = getThemeFromString(result.theme);

    console.log(result);

    return {
        response: ResponseType.SUCCESS,
        settings: result,
    }
}

export async function saveSettings(theme: THEME_MODES): Promise<SettingResponse>
{
    const result: Setting = await invoke("save_settings", {theme});
    result.theme = getThemeFromString(result.theme);

    return {
        response: ResponseType.SUCCESS,
        settings: result,
    }
}

function getThemeFromString(theme: string): THEME_MODES {
    switch (theme) {
        case "DARK":
            return THEME_MODES.DARK;
        case "LIGHT":
            return THEME_MODES.LIGHT;
        default:
            return THEME_MODES.DEFAULT;
    }
}