# Tran

简洁, 快速, 划词翻译

> **Keep it simple，stupid.**

# 功能

|                                 划词翻译                                 |                              划过固定                               |
| :----------------------------------------------------------------------: | :-----------------------------------------------------------------: |
| <img src="https://i.pstorage.space/i/yo5R9JJ3n/original_translate.gif"/> | <img src="https://i.pstorage.space/i/Ll7YxqmW3/original_drag.gif"/> |

|                              划过关闭                               |                              划过复制                              |
| :-----------------------------------------------------------------: | :----------------------------------------------------------------: |
| <img src="https://i.pstorage.space/i/JwQDALlO/original_close.gif"/> | <img src="https://i.pstorage.space/i/MDd9XKW6/original_copy.gif"/> |

**快捷键：** `Alt + X`

# 构建

<div align="center">

| **To \ Form** | **CN** |
| :-----------: | :----: |
|    **EN**     |   ✅   |
|    **JP**     |   ✅   |

</div>

> `Form` 为第一语言, 非第一语言将翻译为第一语言
>
> `To` 为第二语言, 第一语言将翻译为第二语言

**Q: 为什么要分别构建, 而不是一个程序支持所有语言呢?**

因为每多一种语言类型, 构建的包就会增大, 若支持大多数语言，程序将增大到数十 m,而这对于仅需要两种语言(~~就是我~~)的人不太友好, 并且也会导致识别语言类型时花费更多的时间. 所以`tran`将分别构建

**如果你有其他语言的需要, 请提 `issue` 我将添加构建**

# 贡献

## 创建镜像

### [V2G](https://github.com/Borber/v2g)

vercel proxy google translate

<a href="https://vercel.com/import/project?template=https://github.com/Borber/v2g" target="_blank" rel="noopener noreferrer"><img loading="lazy" src="https://vercel.com/button" alt="Deploy to Vercel" ></a>

> 因 vercel 默认域名无法直接访问, 如果您没有域名,可以提 issue , 我将提供域名供您绑定。

**部署后可 PR 到 [MIRROR](https://github.com/Borber/tran/blob/master/resource/mirror.json) 来贡献你的力量**

# 感谢

- **[Pot](https://github.com/pot-app/pot-desktop)** 划词功能模块 [Selection](https://github.com/pot-app/Selection)
