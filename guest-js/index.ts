import { invoke } from "@tauri-apps/api/core";

type Meta = {
    name: string; ext: string
}

export function shareText(payload: { text: string; meta: Meta }) {
    return invoke<void>("plugin:mobile-share|share_text", payload);
}

export function shareBinary(payload: { data: string; meta: Meta }) {
    return invoke<void>("plugin:mobile-share|share_binary", payload);
}
