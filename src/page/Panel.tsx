import "../css/Panel.css"

import { invoke } from "@tauri-apps/api"
import { getCurrent, PhysicalPosition } from "@tauri-apps/api/window"
import { createSignal } from "solid-js"

import { CopyIcon, PinIcon } from "../icon"

const Panel = () => {
    const panel = getCurrent()

    let pinFlag = false
    let moveFlag = true

    const [pin, Pin] = createSignal(false)
    const [copy, Copy] = createSignal(false)
    const [result, Result] = createSignal("")

    // 监听事件， 显示panel
    panel.listen<{ x: number; y: number }>("show", async (pos) => {
        if (!pinFlag) {
            await panel.setPosition(
                new PhysicalPosition(pos.payload.x - 40, pos.payload.y + 20)
            )
        }

        Copy(false)
        Result(await invoke("translate"))

        await panel.show()
        await panel.setFocus()
    })

    return (
        <div
            class="panel"
            onMouseLeave={async () => {
                if (moveFlag) {
                    await panel.hide()
                }
            }}>
            <div class="result">{result()}</div>
            <div class="panel-control">
                <div
                    class="panel-control-item"
                    onClick={() => {
                        moveFlag = !moveFlag
                        pinFlag = !pinFlag
                        Pin(pinFlag)
                    }}>
                    <span classList={{ "panel-control-pin": pin() }}>
                        <PinIcon size={14} />
                    </span>
                </div>
                <div data-tauri-drag-region class="panel-control-drag" />
                <div
                    class="panel-control-item"
                    classList={{ "panel-control-copy": copy() }}
                    onClick={() => Copy(!copy())}>
                    <CopyIcon size={12} />
                </div>
            </div>
        </div>
    )
}

export default Panel
