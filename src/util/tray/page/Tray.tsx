import "../css/Tray.css"

import { exit } from "@tauri-apps/api/process"
import { getCurrent, WebviewWindow } from "@tauri-apps/api/window"

const Tray = () => {
    const tray = getCurrent()

    return (
        <div
            class="tray compromise"
            onMouseLeave={async () => {
                await tray.close()
            }}
        >
            <div
                class="tray-item"
                onClick={async () => {
                    const main = WebviewWindow.getByLabel("main")
                    if (await main?.isMinimized()) await main?.unminimize()
                    if (!(await main?.isVisible())) await main?.show()
                    await main?.setFocus()
                    await tray.close()
                }}
            >
                设置
            </div>
            <div
                class="tray-item"
                onClick={async () => {
                    await exit(0)
                }}
            >
                退出
            </div>
        </div>
    )
}

export default Tray
