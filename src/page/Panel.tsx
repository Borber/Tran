import "../css/Panel.css"

import { invoke } from "@tauri-apps/api/core"
import {
    getCurrent,
    LogicalSize,
    PhysicalPosition,
} from "@tauri-apps/api/window"
import { createSignal, For, Match, Switch } from "solid-js"

import { CopyIcon, PinIcon } from "../icon"
import { Resp } from "../model/resp"

interface TransVO {
    word: boolean
    trans: string
    dicts: Dict[]
}

interface Dict {
    pos: string
    terms: string[]
}

const Panel = () => {
    const panel = getCurrent()

    let pinFlag = false

    const [pin, Pin] = createSignal(false)
    const [copy, Copy] = createSignal(false)
    const [result, Result] = createSignal<TransVO>()

    // 监听事件， 显示panel
    // Listen to events and display panel
    panel.listen<{ x: number; y: number; content: string }>(
        "show",
        async (pos) => {
            Result(undefined)
            if (!pinFlag) {
                await panel.setPosition(
                    new PhysicalPosition(pos.payload.x, pos.payload.y)
                )
                // 移动位置之后需要保证窗口大小不变
                await panel.setSize(new LogicalSize(256, 100))
            }
            Copy(false)
            await panel.show()

            const resp = await invoke<Resp<TransVO>>("translate", {
                content: pos.payload.content,
            })
            Result(resp.data)
        }
    )

    const hide = async () => {
        pinFlag = false
        Pin(false)
        Copy(false)
        await panel.hide()
        Result(undefined)
    }

    return (
        <div
            class="panel"
            onMouseLeave={async () => {
                if (!pinFlag) {
                    await hide()
                }
            }}
        >
            <div class="panel-control">
                <div
                    class="panel-control-item"
                    classList={{ "panel-control-copy": copy() }}
                    onMouseEnter={() => {
                        let content
                        if (result() == undefined) {
                            return
                        } else if (!result()?.word) {
                            content = result()!.trans
                        } else {
                            content = result()!.dicts[0].terms[0]
                        }
                        invoke("copy", {
                            content,
                        })
                        Copy(true)
                    }}
                >
                    <CopyIcon size={12} />
                </div>

                <div
                    data-tauri-drag-region
                    class="panel-control-item panel-control-pin-conainer"
                    onMouseUp={() => {
                        pinFlag = !pinFlag
                        Pin(pinFlag)
                        invoke("pin", { state: pinFlag })
                    }}
                    onMouseEnter={() => {
                        pinFlag = true
                        Pin(true)
                        invoke("pin", { state: true })
                    }}
                >
                    <div classList={{ "panel-control-pin": pin() }}>
                        <PinIcon size={14} />
                    </div>
                </div>
            </div>

            <div class="result">
                <Switch fallback={"翻译中..."}>
                    <Match when={result() == undefined}>翻译中...</Match>
                    <Match when={result()?.word}>
                        <For each={result()!.dicts}>
                            {(dict) => (
                                <div class="dict">
                                    <div class="dict-pos">{dict.pos}</div>
                                    <For each={dict.terms}>
                                        {(term) => (
                                            <div class="dict-term">{term}</div>
                                        )}
                                    </For>
                                </div>
                            )}
                        </For>
                    </Match>
                    <Match when={!result()?.word}>{result()?.trans}</Match>
                </Switch>
            </div>
        </div>
    )
}

export default Panel
