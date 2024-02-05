const lang = Deno.env.get("MATRIX_LANG")
const version = Deno.env.get("TRAN_VERSION")

const root = "src-tauri/target/release/bundle/";
const deb = root + "deb/tran_" + version + "_amd64.deb"
const rpm = root + "rpm/tran-" + version + "-1.x86_64.rpm"
const appimage = root + "appimage/tran_" + version + "_amd64.AppImage"

const new_deb = "Tran_" + lang + "_amd64.deb"
const new_rpm = "Tran_" + lang + "_amd64.rpm"
const new_appimage = "Tran_" + lang + "_amd64.AppImage"

await Deno.mkdir("release")
await Deno.copyFile(deb, `release/${new_deb}`)
await Deno.copyFile(rpm, `release/${new_rpm}`)
await Deno.copyFile(appimage, `release/${new_appimage}`)

for await (const file of Deno.readDir("release")) {
    console.log(file.name)
}