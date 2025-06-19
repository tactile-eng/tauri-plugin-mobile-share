import { invoke } from "@tauri-apps/api/core";

export type ShareMeta = {
    name: string;
    ext: string;
}

/** Text Document data is just a string, so...that's pretty simple */
export function shareText(payload: { text: string; meta: ShareMeta }) {
    return invoke<void>("plugin:mobile-share|share_text", payload);
}

/**
 * binary data must be encoded as a base64 string, so assuming you have some `file` of type `Blob`:
 * ```js
 * const data = Buffer.from(await file.arrayBuffer()).toString("base64")
 * ```
 */
export function shareBinary(payload: { data: string; meta: ShareMeta }) {
    return invoke<void>("plugin:mobile-share|share_binary", payload);
}
