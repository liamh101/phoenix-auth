import {generateToken} from "./Commands.ts";
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

export async function copyOtpToClipboard(accountId: number): Promise<void> {
    try {
        await writeText((await generateToken(accountId)).token);
    } catch (e) {
        console.error(e)
    }
}