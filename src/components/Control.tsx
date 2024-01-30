import "../css/Control.css"

import { getCurrent } from "@tauri-apps/api/window"
import { Show } from "solid-js"

import { Close, Maximize, Minimize } from "../icon"

export interface BarProps {
    minimize?: boolean
    maximize?: boolean
    close?: boolean
}

const defaultProps: BarProps = {
    minimize: true,
    maximize: true,
    close: true,
}

const Control = (props: BarProps) => {
    const setting = { ...defaultProps, ...props }
    const current = getCurrent()

    return (
        <div class="control">
            <Show when={setting.minimize}>
                <div
                    class="control-item"
                    title="最小化"
                    onClick={() => {
                        current.minimize()
                    }}
                >
                    <Minimize size={10} />
                </div>
            </Show>
            <Show when={setting.maximize}>
                <div
                    class="control-item"
                    title="最大化"
                    onClick={async () => {
                        if (await current.isMaximized()) {
                            current.unmaximize()
                        } else {
                            current.maximize()
                        }
                    }}
                >
                    <Maximize size={10} />
                </div>
            </Show>
            <Show when={setting.close}>
                <div
                    class="control-item control-item-close"
                    title="关闭"
                    onClick={async () => {
                        await current.hide()
                        await current.close()
                    }}
                >
                    <Close size={10} />
                </div>
            </Show>
        </div>
    )
}

export default Control
