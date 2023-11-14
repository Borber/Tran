import "../css/Setting.css"

import { getCurrent } from "@tauri-apps/api/window"

import { vibrancy } from "../common"
import Control from "../components/Control"
import TopBar from "../components/TopBar"

const Setting = () => {
    // TODO 代理模式
    // 设置代理链接
    // TODO 镜像模式
    // 设置镜像网址

    // TODO 设置第一语言
    // TODO 设置第二语言

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
