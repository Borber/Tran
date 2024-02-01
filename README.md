<a href="https://github.com/Borber/tran"><img width="200px" src="public/icon.png" align="left"/></a>

# Tran

简洁, 快速, 划词翻译

[![EN_README](https://img.shields.io/badge/-EN_README-yellow?color=%2307baf3&style=for-the-badge&logoColor=white)](./README_EN.md)
[![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&style=for-the-badge&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-blue?logo=tauri&color=1B1B1D&style=for-the-badge)](https://tauri.app/)
[![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)
[![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)
[![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)
[![LICENSE](https://img.shields.io/github/license/borber/tran?color=%2398cbed&logo=rust&style=for-the-badge)](https://github.com/Borber/tran?tab=GPL-3.0-1-ov-file)
[![Downloads](https://img.shields.io/github/downloads/Borber/tran/total.svg?style=for-the-badge&color=82E0AA&logo=github)](https://github.com/Borber/tran/releases)
[![Cloudflare](https://img.shields.io/badge/-Cloudflare-yellow?style=for-the-badge&color=555555&logo=cloudflare)](https://www.cloudflare.com/)
[![Telegram](https://img.shields.io/badge/-Telegram-yellow?style=for-the-badge&color=555555&logo=telegram)](https://t.me/borber_tran)

> **Keep it simple，stupid.**

# 功能

-   开箱即用
-   永久免费使用
-   谷歌翻译镜像

**快捷键：** `双击 CapsLock`

|                                       划词翻译                                       |                                  划过固定                                  |
| :----------------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![translate](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/translate.gif) | ![drag](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/drag.gif) |

|                                   划过关闭                                   |                                  划过复制                                  |
| :--------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![close](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/close.gif) | ![copy](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/copy.gif) |

**PS**

1. 固定翻译面板后, 划词自动翻译, 无需按快捷键

# 构建

<div align="center">

| **To \ Form** | **ZH** | **JA** |
| :-----------: | :----: | :----: |
|    **ZH**     |        |   ✅   |
|    **EN**     |   ✅   |        |
|    **JA**     |   ✅   |        |

</div>

> `Form` 为第一语言, 非第一语言将翻译为第一语言
>
> `To` 为第二语言, 第一语言将翻译为第二语言

**Q: 为什么要分别构建, 而不是一个程序支持所有语言呢?**

因为每多一种语言类型, 构建的包就会增大, 若支持大多数语言，程序将增大到数十 m,而这对于仅需要两种语言(~~就是我~~)的人不太友好, 并且也会导致识别语言类型时花费更多的时间. 所以`tran`将分别构建

**如果你有其他语言的需要, 请提 `issue` 我将添加构建**

# 贡献

## 参与开发

### 准备环境

-   [rust](https://www.rust-lang.org/tools/install)
    -   最基本的开发环境
-   [pnpm](https://pnpm.io/installation)
    -   前端部分包管理
-   [nodejs](https://nodejs.org/)
    -   推荐使用 [fnm](https://github.com/Schniz/fnm) 进行管理
    -   `tauri` 开发所需
-   [just](https://github.com/casey/just) (可选)
    -   便捷命令
-   [deno](https://docs.deno.com/runtime/manual/getting_started/installation) (可选)
    -   脚本工具
-   [vscode](https://code.visualstudio.com/) (推荐)
    -   [`deno` 插件](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno)

### 提交

1. 如果想新加一个功能, 请先提 `issue`, 讨论一下, 避免无效工作
2. 对原有功能进行改进
3. 削减无用的代码, 关闭无用的 `feature`
4. 使用更轻量的`lib`实现功能
5. 添加测试与文档
6. 升级, 更新依赖的提交也会被接受

## 积极使用

理论上，更多人使用则 `tran` 翻译速度会保持在很快的速度，因为 `vercel` 冷启动耗时很长。频繁的请求能使它一直运行。所以鼓励您频繁使用 `tran`. **同理也拜托您推广 `tran`**

## 创建镜像

更多的镜像能支持更多的人使用，所以鼓励您创建镜像。

### [V2G](https://github.com/Borber/v2g)

vercel proxy google translate

[![vercel](https://vercel.com/button)](https://vercel.com/import/project?template=https://github.com/Borber/v2g)

> 因 vercel 默认域名无法直接访问, 如果您没有域名,可以提 issue , 我将提供域名供您绑定。

**部署后可 PR 到 [MIRROR](https://github.com/Borber/tran/blob/master/resource/mirror.json) 来贡献你的力量**

# 感谢

-   **[Pot](https://github.com/pot-app/pot-desktop)** : [Selection](https://github.com/pot-app/Selection)
