const lang = Deno.env.get("MATRIX_LANG")
const version = Deno.env.get("TRAN_VERSION")
const platform = Deno.env.get("TRAN_PLATFORM")


const root = "src-tauri/target/release/";

const bundle = root + "bundle/"
const msi = bundle + "msi/Tran_" + version + "_x64_en-US.msi"
const nsis = bundle + "nsis/Tran_" + version + "_x64-setup.exe"

const new_msi = "Tran_" + lang + "_x64.msi"
const new_nsis = "Tran_" + lang + "_x64-setup.exe"

await Deno.mkdir("release")
await Deno.copyFile(msi, `release/${new_msi}`)
await Deno.copyFile(nsis, `release/${new_nsis}`)

const portable = root + "Tran.exe"
const new_portable = "Tran_" + lang + "_" + platform + "_portable.exe"

await Deno.copyFile(portable, `release/${new_portable}`)
