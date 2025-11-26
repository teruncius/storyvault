export function convertSecondsToISO8601(seconds: number) {
    return `PT${Math.abs(Math.floor(seconds))}S`;
}

export function convertISO8601ToSeconds(isoString: string) {
    const match = isoString.match(/PT(\d+)S/);
    if (!match) {
        return 0;
    }
    return parseInt(match[1], 10);
}
