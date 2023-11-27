# Tran

简洁, 快速, 划词翻译

> **Keep it simple，stupid.**

# 功能

|                                       划词翻译                                       |                                  划过固定                                  |
| :----------------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![translate](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/translate.gif) | ![drag](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/drag.gif) |

|                                   划过关闭                                   |                                  划过复制                                  |
| :--------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![close](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/close.gif) | ![copy](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/copy.gif) |

**快捷键：** `Alt + X`

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

-   **[Pot](https://github.com/pot-app/pot-desktop)** 划词功能模块 [Selection](https://github.com/pot-app/Selection)
