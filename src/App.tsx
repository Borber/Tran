import "./App.css"

import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import {
    getCurrent,
    LogicalSize,
    PhysicalPosition,
} from "@tauri-apps/api/window"
import { createSignal, For, Match, onMount, Show, Switch } from "solid-js"
import { ThreeDots } from "solid-spinner"

import { UpdateIcon } from "./icon"
import { Resp } from "./model/resp"

interface TransVO {
    word: boolean
    trans: Tran[]
    dicts: Dict[]
}

interface Dict {
    pos: string
    terms: string[]
}

interface Tran {
    typ: number
    data: string
}

const App = () => {
    const panel = getCurrent()
    const [result, Result] = createSignal<TransVO>()
    const [update, Update] = createSignal(false)

    onMount(async () => {
        // 监听事件， 显示panel
        // Listen to events and display panel
        await listen<{
            x: number
            y: number
            content: string
            pin: boolean
        }>("show", async (pos) => {
            Result(undefined)
            if (!pos.payload.pin) {
                await panel.setPosition(
                    new PhysicalPosition(pos.payload.x, pos.payload.y)
                )

                // 移动位置之后需要保证窗口大小不变
                await panel.setSize(new LogicalSize(256, 100))

                await panel.show()

                // pin when shortcut
                // 在快捷键调用时, 接替 pin 达到不关闭的目的
                await panel.setFocus()
                await invoke("pin", {
                    state: false,
                })
            }

            await invoke<Resp<TransVO>>("translate", {
                content: pos.payload.content,
            }).then((resp) => {
                Result(resp.data)
            })
        })

        // 监听事件， 显示panel
        // Listen to events and display panel
        await listen("clean", async () => {
            Result(undefined)
        })

        // 生产环境, 全局取消右键菜单;
        if (!import.meta.env.DEV) {
            document.oncontextmenu = (event) => {
                event.preventDefault()
            }
        }

        window.addEventListener("keydown", async (e) => {
            if (e.key == "Escape") {
                await invoke("pin", {
                    state: false,
                })
                await panel.hide()
                Result(undefined)
            }
        })

        await fetch("https://key.borber.top/TRAN_VERSION").then(
            async (resp) => {
                const version = await resp.text()
                Update(version != "0.2.9")
            }
        )
    })

    return (
        <div class="panel" data-tauri-drag-region>
            <div
                data-tauri-drag-region
                class="result"
                // 因为全局的可拖拽导致双击正好能触发点击事件
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

                    await panel.hide()
                    Result(undefined)
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
