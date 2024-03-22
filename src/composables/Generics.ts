
export async function copyTextToClipboard(value: string): Promise<void> {
    try {
        await navigator.clipboard.writeText(value);
    } catch (e) {
        console.error(e)
    }
}