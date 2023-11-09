import "./App.css"

import { useRoutes } from "@solidjs/router"
import { lazy, onMount } from "solid-js"

const App = () => {
    onMount(async () => {
        // 生产环境, 全局取消右键菜单;
        if (!import.meta.env.DEV) {
            document.oncontextmenu = (event) => {
                event.preventDefault()
            }
        }
    })

    const routes = [
        { path: "/", component: lazy(() => import("./page/Setting")) },
        {
            path: "/tray",
            component: lazy(() => import("./util/tray/page/Tray")),
        },
        {
            path: "/panel",
            component: lazy(() => import("./page/Panel")),
        },
    ]

    const Routes = useRoutes(routes)

    return <Routes />
}

export default App
