import "../css/Setting.css"

import { getVersion } from "@tauri-apps/api/app"
import { invoke } from "@tauri-apps/api/core"
import { getCurrent } from "@tauri-apps/api/window"
import { exit } from "@tauri-apps/plugin-process"
import { createSignal, Match, onMount, Show, Switch } from "solid-js"

import Control from "../components/Control"
import TopBar from "../components/TopBar"
import { GithubIcon, UpdateIcon } from "../icon"
import { Resp } from "../model/resp"

interface ConfigProps {
    mode: number
    url: string
}

const Setting = () => {
    const current = getCurrent()
    const [mode, Mode] = createSignal(0)
    const [url, Url] = createSignal("")
    const [update, Update] = createSignal(false)
    const [version, Version] = createSignal("")

    onMount(async () => {
        const appVersion = await getVersion()
        Version(appVersion)

        await fetch(
            "https://fastly.jsdelivr.net/gh/Borber/tran@master/package.json"
        )
            .then((res) => res.json())
            .then((json) => {
                Update(json.version != appVersion)
            })

        const resp = await invoke<Resp<ConfigProps>>("get_config")
        Mode(resp.data.mode)
        Url(resp.data.url)

        // 100ms 后显示界面
        setTimeout(async () => {
            await current.show()
        }, 100)
    })

    return (
        <div data-tauri-drag-region class="container">
            <Control maximize={false} minimize={false} />
            <TopBar />
            <div data-tauri-drag-region class="content">
                <div data-tauri-drag-region class="setting-title">
                    设置
                </div>
                <div class="setting-option">
                    <div
                        class="setting-option-item"
                        classList={{
                            "setting-option-item-selected": mode() == 0,
                        }}
                        onClick={async () => {
                            Mode(0)
                            await invoke("switch_mode", {
                                mode: 0,
                            })
                        }}
                    >
                        镜像
                    </div>
                    <div
                        class="setting-option-item"
                        classList={{
                            "setting-option-item-selected": mode() == 1,
                        }}
                        onClick={async () => {
                            Mode(1)
                            await invoke("switch_mode", {
                                mode: 1,
                            })
                        }}
                    >
                        直连
                    </div>
                    <div
                        class="setting-option-item"
                        classList={{
                            "setting-option-item-selected": mode() == 2,
                        }}
                        onClick={async () => {
                            Mode(2)
                            await invoke("switch_mode", {
                                mode: 2,
                            })
                        }}
                    >
                        代理
                    </div>
                </div>
                <Switch fallback={"Need fix"}>
                    <Match when={mode() < 2}>
                        <div class="setting-enjoy">🎉</div>
                    </Match>
                    <Match when={mode() == 2}>
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
                <div
                    class="exit"
                    onClick={async () => {
                        await current.hide()
                        await exit(0)
                    }}
                >
                    退出
                </div>

                <div class="version">{version()}</div>

                <Show when={update()}>
                    <div
                        class="update"
                        onClick={async () => {
                            await invoke("open", {
                                url: "https://github.com/Borber/tran/releases/latest",
                            })
                        }}
                    >
                        <UpdateIcon size={30} />
                    </div>
                </Show>

                <div
                    class="github"
                    onClick={async () => {
                        await invoke("open", {
                            url: "https://github.com/Borber/tran",
                        })
                    }}
                >
                    <GithubIcon size={30} />
                </div>
            </div>
        </div>
    )
}

export default Setting
