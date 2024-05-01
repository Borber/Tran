<a href="https://github.com/Borber/tran"><img width="200px" src="https://cdn.jsdelivr.net/gh/Borber/tran/public/icon.png" align="left"/></a>

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
[![Telegram](https://img.shields.io/badge/-Telegram-yellow?style=for-the-badge&color=25a3e1&logo=telegram)](https://t.me/tran_rust)
[![Cloudflare](https://img.shields.io/badge/-Cloudflare-yellow?style=for-the-badge&color=555555&logo=cloudflare)](https://www.cloudflare.com/)

> **Keep it simple，stupid.**

# 你好

<div align="center">
    <a href="https://github.com/Borber/tran" target="_blank" alt="Tran">
        <img src="https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/tran.png">
    </a>
</div>

-   开箱即用
-   永久免费使用
-   谷歌翻译镜像

# 如何选择?

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

如果你是中文用户, 但需要进行英语阅读, 则对应的选择 `zh_en` 的安装包即可. 同理选择适合你的安装包, **如果你没有安装 `webview`, 或不知道 `webview` 是什么。 请不要选择 `portable` 版本.**

**如果你有其他语言的需要, 请提 `issue` 我将添加构建**

# 使用

-   单次翻译
    -   选中文本之后, 快速按下快捷键 `Shift` 键 **两次**. 你可以通过点击其他地方使 `Tran` 失去焦点, 自动关闭
        -   双击 `Shift`. 我知道这可能会一些非常出色的工具产生不可调和的快捷键冲突, 在此提前抱歉.
-   多次翻译
    -   在单次翻译之后, 拖动翻译面板, 使 `Tran` 固定
    -   固定面板后, 双击单词或选中长文本 `Tran` 将自动翻译
    -   选中面板， 按 `Esc` 键关闭
-   复制译文
    -   双击翻译结果即可将译文到剪切板

## 推荐

推荐安装以下字体:

-   [FiraCode Nerd Font Mono](https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip)
-   [LXGW WenKai](https://github.com/lxgw/LxgwWenKai)

## 自动启动

-   [Windows](https://support.microsoft.com/zh-cn/windows/%E5%9C%A8-windows-10-%E4%B8%AD%E6%B7%BB%E5%8A%A0%E5%9C%A8%E5%90%AF%E5%8A%A8%E6%97%B6%E8%87%AA%E5%8A%A8%E8%BF%90%E8%A1%8C%E7%9A%84%E5%BA%94%E7%94%A8-150da165-dcd9-7230-517b-cf3c295d89dd)
-   [MacOS](https://support.apple.com/zh-cn/guide/mac-help/mh15189/mac)
-   Linux wiki 是你最好的教程

# 贡献

## 参与开发

### 准备环境

| 工具                                            | 备注       |
| ----------------------------------------------- | ---------- |
| [rust](https://www.rust-lang.org/tools/install) | Rust 开发  |
| [pnpm](https://pnpm.io/installation)            | 前端包管理 |
| [nodejs](https://nodejs.org/)                   | 前端开发   |

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

-   消耗 Edge Requests

[![vercel](https://vercel.com/button)](https://vercel.com/import/project?template=https://github.com/Borber/v2g)

> 因 vercel 默认域名无法直接访问, 如果您没有域名,可以提 issue , 我将提供域名供您绑定。

**部署后可 PR 到 [MIRROR](https://github.com/Borber/tran/blob/master/resource/mirror.json) 来贡献你的力量**

# 感谢

-   **[Pot](https://github.com/pot-app/pot-desktop)** : [Selection](https://github.com/pot-app/Selection)
