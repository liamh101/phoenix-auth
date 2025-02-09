import { getCurrentWindow } from '@tauri-apps/api/window';

const DEFAULT_THEME = 'light';

export async function getOsTheme(): string {
    const theme = await getCurrentWindow().theme();

    if (!theme) {
        return DEFAULT_THEME;
    }

    return theme;
}