import {parseOptUrl} from "./Commands.ts";

export function parseWaFile(fileContents: string) {
    return fileContents
        .split(/\r?\n|\r|\n/g)
        .filter(potentialOpt => isValidOtpUrl(potentialOpt))
        .map(otpUrl => parseOptUrl(otpUrl));
}

function isValidOtpUrl(potentialUrl: string) {
    return potentialUrl.match(/^otpauth:\/\/([ht]otp)\/(?:[a-zA-Z0-9%]+:)?([^?]+)\?(?=.*secret).*/)
}