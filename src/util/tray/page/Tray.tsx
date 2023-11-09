import "../css/Tray.css"

import { listen } from "@tauri-apps/api/event"
import { exit } from "@tauri-apps/api/process"
import {
    getCurrent,
    PhysicalPosition,
    WebviewWindow,
} from "@tauri-apps/api/window"
import { createSignal } from "solid-js"

const Tray = () => {
    const tray = getCurrent()

    // 模拟 hover 效果, 解决 tauri 最小化/关闭 时, hover 效果不消失的问题
    const [settingFlag, SettingFlag] = createSignal(false)
    const [exitFlag, ExitFlag] = createSignal(false)

    listen<{ x: number; y: number }>("tray", async (pos) => {
        await tray.setPosition(
            new PhysicalPosition(pos.payload.x, pos.payload.y)
        )
        await tray.show()
        await tray.setFocus()
    })

    return (
        <div
            class="tray compromise"
            onMouseLeave={async () => {
                await tray.hide()
            }}>
            <div
                class="tray-item"
                classList={{ "tray-item-hover": settingFlag() }}
                onMouseEnter={() => {
                    SettingFlag(true)
                }}
                onMouseLeave={() => {
                    SettingFlag(false)
                }}
                onClick={async () => {
                    await tray.hide()
                    SettingFlag(false)
                    const main = WebviewWindow.getByLabel("main")
                    if (await main?.isMinimized()) await main?.unminimize()
                    if (!(await main?.isVisible())) await main?.show()
                    await main?.setFocus()
                }}>
                设置
            </div>
            <div
                class="tray-item"
                classList={{ "tray-item-hover": exitFlag() }}
                onMouseEnter={() => {
                    ExitFlag(true)
                }}
                onMouseLeave={() => {
                    ExitFlag(false)
                }}
                onClick={async () => {
                    ExitFlag(false)
                    await exit(0)
                }}>
                退出
            </div>
        </div>
    )
}

export default Tray
