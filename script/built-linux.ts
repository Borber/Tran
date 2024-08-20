const lang = Deno.env.get("MATRIX_LANG")
const version = Deno.env.get("TRAN_VERSION")
const platform = Deno.env.get("TRAN_PLATFORM")

const root = "src-tauri/target/release/";

const bundle = root + "bundle/"
const deb = bundle + "deb/Tran_" + version + "_amd64.deb"
const rpm = bundle + "rpm/Tran-" + version + "-1.x86_64.rpm"
const appimage = bundle + "appimage/Tran_" + version + "_amd64.AppImage"

const new_deb = "Tran_" + lang + "_amd64.deb"
const new_rpm = "Tran_" + lang + "_amd64.rpm"
const new_appimage = "Tran_" + lang + "_amd64.AppImage"

await Deno.mkdir("release")
await Deno.copyFile(deb, `release/${new_deb}`)
await Deno.copyFile(rpm, `release/${new_rpm}`)
await Deno.copyFile(appimage, `release/${new_appimage}`)

const portable = root + "tran"
const new_portable = "Tran_" + lang + "_" + platform + "_portable"

await Deno.copyFile(portable, `release/${new_portable}`)