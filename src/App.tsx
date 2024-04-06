import "./App.css"

import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { getCurrent } from "@tauri-apps/api/window"
import { createSignal, For, Match, onMount, Show, Switch } from "solid-js"
import { ThreeDots } from "solid-spinner"

import { UpdateIcon } from "./icon"
import { Resp, TransVO } from "./model/resp"

const App = () => {
    const panel = getCurrent()
    const [result, Result] = createSignal<TransVO>()
    const [update, Update] = createSignal(false)

    const close = async () => {
        await panel.hide()
        await invoke("unpin")
        Result(undefined)
    }

    onMount(async () => {
        // 生产环境, 全局取消右键菜单
        // Production environment, cancel right-click menu
        if (!import.meta.env.DEV) {
            document.oncontextmenu = (event) => {
                event.preventDefault()
            }
        }

        // 监听事件， 显示翻译结果
        // Listen to events and display panel
        await listen<Resp<TransVO>>("show", async (pos) => {
            Result(pos.payload.data)
            // 显示完成后取消临时固定
            // After displaying, cancel temporary pin
            await invoke("untmp")
        })

        // 监听事件，清空翻译结果
        // Listen to events and clear translation results
        await listen("reset", async () => {
            Result(undefined)
        })

        window.addEventListener("keydown", async (e) => {
            if (e.key == "Escape") {
                await close()
            }
        })

        await fetch("https://key.borber.top/TRAN_VERSION").then(
            async (resp) => {
                const version = await resp.text()
                Update(version != "0.2.12")
            }
        )
    })

    return (
        <div class="panel" data-tauri-drag-region>
            <div
                data-tauri-drag-region
                class="result"
                // 因为全局的可拖拽导致双击正好能触发点击事件
                // Because the draggable global causes the double click to trigger the click event
                onClick={async (e) => {
                    let content
                    if (result() == undefined) {
                        return
                    } else if (!result()?.word) {
                        content = e.target.innerHTML
                            .replace(/<br>/g, "\r\n")
                            .replace(/&nbsp;/g, " ")
                    } else {
                        content = result()!.dicts[0].terms[0]
                    }

                    await invoke("copy", {
                        content,
                    })

                    let pin = await invoke<Resp<boolean>>("pin").then((pos) => {
                        return pos.data
                    })

                    // 未固定则直接关闭
                    if (!pin) {
                        await close()
                    }
                }}
            >
                <Switch>
                    <Match when={result() == undefined}>
                        <div data-tauri-drag-region>
                            翻译中{`\u00A0`}
                            <ThreeDots width={20} height={10} />
                        </div>
                    </Match>
                    <Match when={result()?.word}>
                        <For each={result()!.dicts}>
                            {(dict) => (
                                <div data-tauri-drag-region class="dict">
                                    <Show when={dict.pos != ""}>
                                        <div
                                            data-tauri-drag-region
                                            class="dict-pos"
                                        >
                                            {dict.pos}
                                        </div>
                                    </Show>
                                    <For each={dict.terms}>
                                        {(term) => (
                                            <div
                                                data-tauri-drag-region
                                                class="dict-term"
                                            >
                                                {term}
                                            </div>
                                        )}
                                    </For>
                                </div>
                            )}
                        </For>
                    </Match>
                    <Match when={!result()?.word}>
                        <For each={result()!.trans}>
                            {(tran) => {
                                if (tran.typ == 0) {
                                    return tran.data
                                } else if (tran.typ == 1) {
                                    return <br />
                                } else {
                                    return `\u00A0`
                                }
                            }}
                        </For>
                    </Match>
                </Switch>
            </div>
            <Show when={update()}>
                <div
                    class="update"
                    onClick={async () => {
                        await invoke("open", {
                            url: "https://github.com/Borber/tran/releases/latest",
                        })
                    }}
                >
                    <UpdateIcon size={20} />
                </div>
            </Show>
        </div>
    )
}

export default App
