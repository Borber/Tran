import "../css/Setting.css"

import { getCurrent } from "@tauri-apps/api/window"

import { vibrancy } from "../common"
import Control from "../components/Control"
import TopBar from "../components/TopBar"

const Setting = () => {
    const main = getCurrent()

    main.listen("tauri://close-requested", async () => {
        await main.hide()
    })

    return (
        <div
            data-tauri-drag-region
            class="container"
            classList={{ compromise: !vibrancy() }}>
            <Control maximize={false} minimize={false} />
            <TopBar />
            <div data-tauri-drag-region class="content">
                <div data-tauri-drag-region class="setting-title">
                    设置
                </div>
            </div>
        </div>
    )
}

export default Setting
