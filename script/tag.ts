const dirs = ["msi", "nsis", "deb", "appimage", "dmg", "macos",]

const lang = Deno.env.get("MATRIX_LANG")
const root = "src-tauri/target/release/bundle"

for (const dir of dirs) {
    if (Deno.statSync(`${root}/${dir}`).isDirectory) {
        for await (const file of Deno.readDir(dir)) {
            if (file.isFile && file.name.startsWith("tran")) {
                let name = file.name
                name = name.replace("tran", "tran" + "_" + lang)
                Deno.renameSync(`${root}/${file.name}`, `${root}/${name}`)
                console.log(`${root}/${name}`)
            }
        }
    }
}