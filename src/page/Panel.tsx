import "../css/Panel.css"

import { invoke } from "@tauri-apps/api"
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
    panel.listen<{ x: number; y: number; context: string }>(
        "show",
        async (pos) => {
            Result(undefined)
            if (!pinFlag) {
                await panel.setPosition(
                    new PhysicalPosition(pos.payload.x, pos.payload.y)
                )

                // 刷新 固定图标状态
                // Refresh pin icon state
                pinFlag = false
                Pin(false)
            }
            Copy(false)
            await panel.setSize(new LogicalSize(256, 100))
            await panel.show()
            // TODO 错误处理
            const resp = await invoke<Resp<TransVO>>("translate", {
                context: pos.payload.context,
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
            <div class="panel-control">
                <div
                    data-tauri-drag-region
                    class="panel-control-item panel-control-pin-conainer"
                    onMouseUp={() => {
                        pinFlag = !pinFlag
                        Pin(pinFlag)
                    }}
                    onMouseEnter={() => {
                        pinFlag = true
                        Pin(true)
                    }}
                >
                    <div classList={{ "panel-control-pin": pin() }}>
                        <PinIcon size={14} />
                    </div>
                </div>

                <div
                    class="panel-control-item"
                    classList={{ "panel-control-copy": copy() }}
                    onMouseEnter={() => {
                        let context
                        if (result() == undefined) {
                            return
                        } else if (!result()?.word) {
                            context = result()!.trans
                        } else {
                            context = result()!.dicts[0].terms[0]
                        }
                        invoke("copy", {
                            context,
                        })
                        Copy(true)
                    }}
                >
                    <CopyIcon size={12} />
                </div>
            </div>
        </div>
    )
}

export default Panel
