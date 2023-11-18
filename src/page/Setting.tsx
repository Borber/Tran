import "../css/Setting.css"

import { invoke } from "@tauri-apps/api"
import { getCurrent } from "@tauri-apps/api/window"
import { createSignal, Match, onMount, Switch } from "solid-js"

import Control from "../components/Control"
import TopBar from "../components/TopBar"
import { Resp } from "../model/resp"

interface ConfigProps {
    proxy: boolean
    url: string
}

const Setting = () => {
    const main = getCurrent()

    const [proxy, Proxy] = createSignal(false)
    const [url, Url] = createSignal("")

    main.listen("tauri://close-requested", async () => {
        await main.hide()
    })

    onMount(async () => {
        const resp = await invoke<Resp<ConfigProps>>("get_config")
        Proxy(resp.data.proxy)
        Url(resp.data.url)
    })

    return (
        <div data-tauri-drag-region class="container compromise">
            <Control maximize={false} minimize={false} />
            <TopBar />
            <div data-tauri-drag-region class="content">
                <div data-tauri-drag-region class="setting-title">
                    设置
                </div>
                <div class="setting-option">
                    <div
                        class="setting-option-item"
                        classList={{ "setting-option-item-selected": !proxy() }}
                        onClick={async () => {
                            Proxy(false)
                            await invoke("disable_proxy")
                        }}>
                        镜像
                    </div>
                    <div
                        class="setting-option-item"
                        classList={{ "setting-option-item-selected": proxy() }}
                        onClick={async () => {
                            Proxy(true)
                            await invoke("enable_proxy")
                        }}>
                        代理
                    </div>
                </div>
                <Switch fallback={"Need fix"}>
                    <Match when={!proxy()}>
                        <div class="setting-mirror">🎉Enjoy </div>
                    </Match>
                    <Match when={proxy()}>
                        <input
                            class="setting-proxy-url"
                            type="text"
                            placeholder="http://127.0.0.1:8080"
                            value={url()}
                            onInput={async (e) => {
                                Url(e.currentTarget.value)
                                await invoke("set_proxy_url", {
                                    url: e.currentTarget.value,
                                })
                            }}
                        />
                    </Match>
                </Switch>
            </div>
        </div>
    )
}

export default Setting
