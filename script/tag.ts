const dirs = ["msi", "nsis", "deb", "appimage", "dmg", "macos",]

const lang = Deno.env.get("MATRIX_LANG")

for (const dir of dirs) {
    if (Deno.statSync(`./${dir}`).isDirectory) {
        for await (const file of Deno.readDir(dir)) {
            if (file.isFile && file.name.startsWith("tran")) {
                let name = file.name
                name = name.replace("tran", "tran" + "_" + lang)
                Deno.renameSync(`./${dir}/${file.name}`, `./${dir}/${name}`)
                console.log(`./${dir}/${name}`)
            }
        }
    }
}