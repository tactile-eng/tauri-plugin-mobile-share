import { invoke } from "@tauri-apps/api/core";

export type ShareMeta = {
    name: string;
    ext: string;
}

/**
 * 
 * ## Example:
 * ```ts
 * import { ShareMeta, shareText } from "tauri-plugin-mobile-share";
 * 
 * const textData = "lorem ipsum dolor sit amet...";
 * const textMetadata: ShareMeta = { name: "My File", ext: "txt" };
 * 
 * shareText(textData, textMetadata);
 * ```
 */
export function shareText(text: string, meta: ShareMeta) {
    return invoke<void>("plugin:mobile-share|share_text", { text, meta });
}

/**
 * 
 * ## Example:
 * ```ts
 * import { shareBinary, ShareMeta } from "tauri-plugin-mobile-share";
 * 
 * const data: ArrayBuffer; // not included: your file data...
 * const binaryData = Buffer.from(data).toString("base64");
 * const binaryMetadata: ShareMeta = { name: "My File", ext: "png" };
 * 
 * shareBinary(binaryData, binaryMetadata);
 * ```
 */
export function shareBinary(data: string, meta: ShareMeta) {
    return invoke<void>("plugin:mobile-share|share_binary", { data, meta });
}
