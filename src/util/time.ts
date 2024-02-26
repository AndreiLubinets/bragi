export function convertLength(length: number): string {
    let minutes = Math.floor(length / 60);
    let seconds = Math.floor(length % 60);
    return minutes + ":" + (seconds < 10 ? "0" + seconds : seconds);
}