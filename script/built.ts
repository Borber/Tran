import { ensureDir } from "https://deno.land/std@0.207.0/fs/mod.ts";

const dirs = ["msi", "nsis", "deb", "appimage", "dmg", "macos",]

let system = Deno.env.get("MATRIX_PLATFORM")
system = system?.split("-")[0]

const lang = Deno.env.get("MATRIX_LANG")
const root = "src-tauri/target/release"
const bundle = root + "/bundle"

await Deno.mkdir("release")

// 上传 bundle产物
for (const dir of dirs) {
    ensureDir(`${bundle}/${dir}`).then(async () => {
        if (Deno.statSync(`${bundle}/${dir}`).isDirectory) {
            for await (const file of Deno.readDir(`${bundle}/${dir}`)) {
                if (file.isFile && (file.name.startsWith("Tran") || file.name.startsWith("tran"))) {
                    let name = file.name
                    console.log("old:" + `${bundle}/${dir}/${file.name}`)
                    name = name.replace("Tran", "Tran" + "_" + lang)
                    name = name.replace("tran", "Tran" + "_" + lang)
                    await Deno.copyFile(`${bundle}/${dir}/${file.name}`, `release/${name}`)
                    console.log("new:" + `release/${name}`)
                }
            }
        }
    })
}

// 上传 便携产物
for await (const file of Deno.readDir(root)) {
    let name = file.name
    if (name == "Tran.exe" || name == "Tran") {
        name = name.replace("Tran", "Tran" + "_" + lang + "_portable_" + system)
        await Deno.copyFile(`${root}/${file.name}`, `release/${name}`)
        console.log(`release/${name}`)
    }
}