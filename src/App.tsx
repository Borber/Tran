import "./App.css"

import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { createSignal, For, Match, onMount, Show, Switch } from "solid-js"
import { ThreeDots } from "solid-spinner"

import { UpdateIcon } from "./icon"
import { Resp, TransVO } from "./model/resp"

const App = () => {
    const panel = getCurrentWebviewWindow()
    const [result, Result] = createSignal<TransVO>()
    const [update, Update] = createSignal(false)

    const close = async () => {
        await panel.hide()
        await invoke("unpin")
        Result(undefined)
    }

    const copy = async (content: string) => {
        await invoke("copy", {
            content,
        })

        let pin = await invoke<Resp<boolean>>("pin").then((pos) => {
            return pos.data
        })

        // 未固定则直接关闭
        // Unfixed, close directly
        if (!pin) {
            await close()
        }
    }

    onMount(async () => {
        // 主题
        // Theme
        let theme = await invoke<Resp<string>>("theme").then((pos) => {
            return pos.data
        })
        document.documentElement.setAttribute("data-theme", theme)

        // 监听更改主题事件
        // Listen to change theme events
        await listen<string>("theme", (event) => {
            console.log(event)
            document.documentElement.setAttribute("data-theme", event.payload)
        })

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

        // 调整滚动范围
        // Adjust scroll range
        const RESULT = document.getElementById("result")!

        RESULT.addEventListener("wheel", (e) => {
            // 阻止浏览器默认行为
            // Prevent browser default behavior
            e.preventDefault()

            const height = 60
            const scrollAmount = e.deltaY > 0 ? height : -height
            const newScrollTopWithAmount = RESULT.scrollTop + scrollAmount

            // 限制滚动范围
            const maxScrollTop = RESULT.scrollHeight - RESULT.clientHeight
            const newScrollTopLimited = Math.min(
                Math.max(newScrollTopWithAmount, 0),
                maxScrollTop
            )

            RESULT.scrollTo({ top: newScrollTopLimited, behavior: "smooth" })
        })

        window.addEventListener("keydown", async (e) => {
            if (e.key == "Escape") {
                await close()
            }
        })

        await fetch("https://key.borber.top/TRAN_VERSION").then(
            async (resp) => {
                const version = await resp.text()
                Update(version != "0.2.17")
            }
        )
    })

    return (
        <div class="panel" data-tauri-drag-region>
            <div
                class="result"
                id="result"
                data-tauri-drag-region
                // 因为全局的可拖拽导致双击正好能触发点击事件
                // Because the draggable global causes the double click to trigger the click event
                onClick={async (e) => {
                    let content
                    if (result() == undefined) {
                        return
                    }
                    if (!result()?.word) {
                        content = e.target.innerHTML.replace(/<br>/g, "\r\n")
                        await copy(content)
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
                                <div class="dict" data-tauri-drag-region>
                                    <Show when={dict.pos != ""}>
                                        <div
                                            class="dict-pos"
                                            data-tauri-drag-region
                                        >
                                            {dict.pos}
                                        </div>
                                    </Show>
                                    <For each={dict.terms}>
                                        {(term) => (
                                            <div
                                                class="dict-term"
                                                data-tauri-drag-region
                                                onClick={async () => {
                                                    await copy(term)
                                                }}
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
                                } else {
                                    return <br />
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
