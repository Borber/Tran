import "../css/Control.css"

import { appWindow } from "@tauri-apps/api/window"
import { createSignal, Show } from "solid-js"

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

    // 模拟 hover 效果, 解决 tauri 最小化/关闭 时, hover 效果不消失的问题
    const [minFlag, setMinFlag] = createSignal(false)
    const [maxFlag, setMaxFlag] = createSignal(false)
    const [closeFlag, setCloseFlag] = createSignal(false)

    return (
        <div class="control control-compromise">
            <Show when={setting.minimize}>
                <div
                    class="control-item"
                    classList={{ hover: minFlag() }}
                    title="最小化"
                    onMouseEnter={() => setMinFlag(true)}
                    onMouseLeave={() => setMinFlag(false)}
                    onClick={() => {
                        appWindow.minimize()
                        setMinFlag(false)
                    }}>
                    <Minimize size={10} />
                </div>
            </Show>
            <Show when={setting.maximize}>
                <div
                    class="control-item"
                    classList={{ hover: maxFlag() }}
                    title="最大化"
                    onMouseEnter={() => setMaxFlag(true)}
                    onMouseLeave={() => setMaxFlag(false)}
                    onClick={async () => {
                        if (await appWindow.isMaximized()) {
                            appWindow.unmaximize()
                        } else {
                            appWindow.maximize()
                        }
                        setMaxFlag(false)
                    }}>
                    <Maximize size={10} />
                </div>
            </Show>
            <Show when={setting.close}>
                <div
                    class="control-item control-item-close"
                    classList={{
                        hover: closeFlag(),
                    }}
                    title="关闭"
                    onMouseEnter={() => setCloseFlag(true)}
                    onMouseLeave={() => setCloseFlag(false)}
                    onClick={async () => {
                        await appWindow.hide()
                        setCloseFlag(false)
                    }}>
                    <Close size={10} />
                </div>
            </Show>
        </div>
    )
}

export default Control
