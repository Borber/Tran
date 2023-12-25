import "../css/Control.css"

import { appWindow } from "@tauri-apps/api/window"
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

    return (
        <div class="control control-compromise">
            <Show when={setting.minimize}>
                <div
                    class="control-item"
                    title="最小化"
                    onClick={() => {
                        appWindow.minimize()
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
                        if (await appWindow.isMaximized()) {
                            appWindow.unmaximize()
                        } else {
                            appWindow.maximize()
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
                        await appWindow.close()
                    }}
                >
                    <Close size={10} />
                </div>
            </Show>
        </div>
    )
}

export default Control
