const lang = Deno.env.get("MATRIX_LANG")
const version = Deno.env.get("TRAN_VERSION")

const root = "src-tauri/target/release/bundle/";
const msi = root + "msi/Tran_" + version + "_x64_en-US.msi"
const nsis = root + "nsis/Tran_" + version + "_x64-setup.exe"

const new_msi = "Tran_" + lang + "_x64.msi"
const new_nsis = "Tran_" + lang + "_x64-setup.exe"

await Deno.mkdir("release")
await Deno.copyFile(msi, `release/${new_msi}`)
await Deno.copyFile(nsis, `release/${new_nsis}`)

for await (const file of Deno.readDir("release")) {
    console.log(file.name)
}
