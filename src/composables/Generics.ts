import {generateToken} from "./Commands.ts";

export async function copyOtpToClipboard(accountId: number): Promise<void> {
    try {
        await navigator.clipboard.writeText((await generateToken(accountId)).token);
    } catch (e) {
        console.error(e)
    }
}