import { invoke } from "@tauri-apps/api"
import { createResource } from "solid-js"

const fetchVibrancy = async () => {
    const response = await invoke<boolean>("vibrancy")
    return response
}

const [vibrancy] = createResource(fetchVibrancy)

export { vibrancy }
