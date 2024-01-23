<a href="https://github.com/Borber/tran"><img width="200px" src="public/icon.png" align="left"/></a>

# Tran

Simple, fast, translate selected words

[![CN_README](https://img.shields.io/badge/-CN_README-yellow?color=%2307baf3&style=for-the-badge&logoColor=white)](./README.md)
[![LICENSE](https://img.shields.io/github/license/borber/tran?color=%2398cbed&logo=rust&style=for-the-badge)](https://github.com/Borber/tran?tab=GPL-3.0-1-ov-file)
[![Downloads](https://img.shields.io/github/downloads/Borber/tran/total.svg?style=for-the-badge&color=82E0AA&logo=github)](https://github.com/Borber/tran/releases)
[![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&style=for-the-badge&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-blue?logo=tauri&color=1B1B1D&style=for-the-badge)](https://tauri.app/)
[![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)
[![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)
[![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&style=for-the-badge&logoColor=white)](https://github.com/Borber/tran/releases)

> **Keep it simple，stupid.**

# Feature

-   No configuration required
-   Free forever
-   Google Translate Mirror

**Shortcut key：** `Alt + X`

|                                      Translate                                       |                                    Pin                                     |
| :----------------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![translate](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/translate.gif) | ![drag](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/drag.gif) |

|                                    Close                                     |                                    Copy                                    |
| :--------------------------------------------------------------------------: | :------------------------------------------------------------------------: |
| ![close](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/close.gif) | ![copy](https://fastly.jsdelivr.net/gh/Borber/PublicPic1/tran/v1/copy.gif) |

# Construct

<div align="center">

| **To \ Form** | **ZH** | **JA** |
| :-----------: | :----: | :----: |
|    **ZH**     |        |   ✅   |
|    **EN**     |   ✅   |        |
|    **JA**     |   ✅   |        |

</div>

> `Form` as first language, non-first language will be translated into first language
>
> `To` as the second language, the first language will be translated into the second language

**Q: Why build them separately instead of one program supporting all languages?**

Because for each additional language type, the built package will increase. If most languages ​​are supported, the program will increase to tens of meters, and this is for people who only need two languages ​​​​~~that’s me~~ not very friendly, and will also cause more time spent in identifying the language type. So `tran` will be built separately

**If you have needs in other languages, please raise an `issue` and I will add it to the build**

# Contribution

## Develop

### Prepare

-   [rust](https://www.rust-lang.org/tools/install)
    -   Basic development environment
-   [pnpm](https://pnpm.io/installation)
    -   Package management
-   [nodejs](https://nodejs.org/)
    -   Recommended to use [fnm](https://github.com/Schniz/fnm) for management
    -   required by `tauri`
-   [just](https://github.com/casey/just) (optional)
    -   convenient commands
-   [deno](https://docs.deno.com/runtime/manual/getting_started/installation) (optional)
    -   script
-   [vscode](https://code.visualstudio.com/) (recommend)
    -   [`deno` plugin](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno)

### Notice

1. If you want to add a new feature, please raise an issue first and discuss it to avoid ineffective work.
2. Improvements to original functionality
3. Cut down useless code or turn off useless `features'
4. Use a lighter `lib` to implement functions
5. Add tests and documentation
6. Update dependencies will also be accepted

## Active use

Theoretically, if more people use `tran`, the translation speed will remain very fast, because `vercel` cold start takes a long time. Frequent requests can keep it running. So you are encouraged to use `tran` frequently . **Similarly, I also implore you to promote `tran`**

## Create mirror

More mirrors can support more people, so you are encouraged to create mirrors.

### [V2G](https://github.com/Borber/v2g)

vercel proxy google translate

[![vercel](https://vercel.com/button)](https://vercel.com/import/project?template=https://github.com/Borber/v2g)

> Because the default domain name of vercel cannot be directly accessed in China, if you do not have a domain name, you can raise an issue and I will provide a domain name for you to bind.

**You can PR to [MIRROR](https://github.com/Borber/tran/blob/master/resource/mirror.json) after deployment to contribute your strength**

# 感谢

-   **[Pot](https://github.com/pot-app/pot-desktop)** : [Selection](https://github.com/pot-app/Selection)
