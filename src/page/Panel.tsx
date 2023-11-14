import "../css/Panel.css"

import { invoke } from "@tauri-apps/api"
import { getCurrent, PhysicalPosition } from "@tauri-apps/api/window"
import { createSignal, For, Match, Switch } from "solid-js"

import { CopyIcon, PinIcon } from "../icon"

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
    let moveFlag = true

    const [pin, Pin] = createSignal(false)
    const [copy, Copy] = createSignal(false)
    const [result, Result] = createSignal<TransVO>()

    // 监听事件， 显示panel
    panel.listen<{ x: number; y: number; context: string }>(
        "show",
        async (pos) => {
            Result(undefined)
            if (!pinFlag) {
                await panel.setPosition(
                    new PhysicalPosition(pos.payload.x - 40, pos.payload.y + 20)
                )

                // 刷新 固定图标状态
                moveFlag = true
                pinFlag = false
                Pin(false)
            }

            Copy(false)
            await panel.show()

            Result(
                await invoke<TransVO>("translate", {
                    context: pos.payload.context,
                })
            )
        }
    )

    return (
        <div
            class="panel"
            onMouseLeave={async () => {
                if (moveFlag) {
                    moveFlag = true
                    pinFlag = false
                    Pin(false)
                    await panel.hide()
                    Result(undefined)
                }
            }}>
            <div class="result">
                <Switch fallback={"翻译中..."}>
                    <Match when={result() == undefined}>翻译中...</Match>
                    <Match when={result()?.word}>
                        {/* 每一个都显示为一行, flex 布局, 可挤压下去 */}
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
                        moveFlag = !moveFlag
                        pinFlag = !pinFlag
                        Pin(pinFlag)
                    }}
                    onMouseEnter={() => {
                        moveFlag = false
                        pinFlag = true
                        Pin(true)
                    }}>
                    <div classList={{ "panel-control-pin": pin() }}>
                        <PinIcon size={14} />
                    </div>
                </div>

                <div
                    class="panel-control-item"
                    classList={{ "panel-control-copy": copy() }}
                    onMouseEnter={() => {
                        // TODO 复制到剪贴板
                        Copy(true)
                    }}>
                    <CopyIcon size={12} />
                </div>
            </div>
        </div>
    )
}

export default Panel
