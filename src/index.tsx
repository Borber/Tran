/* @refresh reload */
import "./styles.css"

import { Router } from "@solidjs/router"
import { lazy, onMount } from "solid-js"
import { render } from "solid-js/web"

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
        path: "/panel",
        component: lazy(() => import("./page/Panel")),
    },
]

render(
    () => <Router>{routes}</Router>,
    document.getElementById("root") as HTMLElement
)
