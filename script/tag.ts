import { ensureDir } from "https://deno.land/std@0.207.0/fs/mod.ts";

const dirs = ["msi", "nsis", "deb", "appimage", "dmg", "macos",]

const lang = Deno.env.get("MATRIX_LANG")
const root = "src-tauri/target/release/bundle"

for (const dir of dirs) {
    ensureDir(`${root}/${dir}`).then(async () => {
        if (Deno.statSync(`${root}/${dir}`).isDirectory) {
            for await (const file of Deno.readDir(`${root}/${dir}`)) {
                if (file.isFile && file.name.startsWith("tran")) {
                    let name = file.name
                    name = name.replace("tran", "tran" + "_" + lang)
                    Deno.renameSync(`${root}/${dir}/${file.name}`, `${root}/${dir}/${name}`)
                    console.log(`${root}/${dir}/${name}`)
                }
            }
        }
    })
}