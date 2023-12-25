import "../css/Setting.css"

import { invoke } from "@tauri-apps/api"
import { exit } from "@tauri-apps/api/process"
import { getCurrent } from "@tauri-apps/api/window"
import { createSignal, Match, onMount, Switch } from "solid-js"

import Control from "../components/Control"
import TopBar from "../components/TopBar"
import { GithubIcon } from "../icon"
import { Resp } from "../model/resp"

interface ConfigProps {
    mode: number
    url: string
}

const Setting = () => {
    const [mode, Mode] = createSignal(0)
    const [url, Url] = createSignal("")

    onMount(async () => {
        const resp = await invoke<Resp<ConfigProps>>("get_config")
        Mode(resp.data.mode)
        Url(resp.data.url)

        // 100ms 后显示界面
        setTimeout(async () => {
            const current = getCurrent()
            await current.show()
        }, 100)
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
                <div class="exit" onClick={async () => await exit(0)}>
                    退出
                </div>
                <div
                    class="github"
                    onClick={async () => {
                        await invoke("open_github")
                    }}
                >
                    <GithubIcon size={30} />
                </div>
            </div>
        </div>
    )
}

export default Setting
