import "../css/Panel.css"

import { createSignal } from "solid-js"

import { CopyIcon, HideIcon, PinIcon } from "../icon"

const Panel = () => {
    const [pin, Pin] = createSignal(false)
    const [copy, Copy] = createSignal(false)

    return (
        <div class="panel">
            <div class="panel-control">
                <div class="panel-control-item" onClick={() => Pin(!pin())}>
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
                <div class="panel-control-item">
                    <HideIcon size={14} />
                </div>
            </div>
            <div class="result">Hello world!</div>
        </div>
    )
}

export default Panel
