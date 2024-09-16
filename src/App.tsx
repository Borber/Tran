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
        const RESULT = document.getElementById("result") as HTMLElement

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
            // 滚动到顶部
            RESULT.scrollTop = 0
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
        const easeOutCubic = (t: number): number => 1 - Math.pow(1 - t, 3)
        const smoothScroll = (
            element: HTMLElement,
            target: number,
            duration: number
        ): void => {
            const start = element.scrollTop
            const distance = target - start
            const startTime = performance.now()
            const animation = (currentTime: number): void => {
                const progress = Math.min(
                    (currentTime - startTime) / duration,
                    1
                )
                element.scrollTop = start + distance * easeOutCubic(progress)
                if (progress < 1) requestAnimationFrame(animation)
            }
            requestAnimationFrame(animation)
        }

        let lastScrollTime = 0
        let accumulatedDelta = 0
        let scrollAnimationFrame: number
        let scrollTimeoutId: number

        RESULT.addEventListener("wheel", (e: WheelEvent): void => {
            e.preventDefault()
            const currentTime = performance.now()
            if (currentTime - lastScrollTime > 50) accumulatedDelta = 0
            lastScrollTime = currentTime
            accumulatedDelta =
                Math.sign(e.deltaY) *
                Math.min(Math.abs(accumulatedDelta + e.deltaY), 50)
            const newScrollTop = RESULT.scrollTop + accumulatedDelta * 0.5
            const maxScrollTop = RESULT.scrollHeight - RESULT.clientHeight
            const targetScrollTop = Math.max(
                0,
                Math.min(newScrollTop, maxScrollTop)
            )
            cancelAnimationFrame(scrollAnimationFrame)
            scrollAnimationFrame = requestAnimationFrame(() => {
                smoothScroll(RESULT, targetScrollTop, 150)
            })
            clearTimeout(scrollTimeoutId)
            scrollTimeoutId = window.setTimeout(() => {
                accumulatedDelta = 0
            }, 200)
        })

        window.addEventListener("keydown", async (e) => {
            if (e.key == "Escape") {
                await close()
            }
        })

        await fetch("https://key.borber.top/TRAN_VERSION").then(
            async (resp) => {
                const version = await resp.text()
                Update(version != "0.2.18")
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
