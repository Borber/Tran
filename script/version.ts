let context = await Deno.readTextFile("src-tauri/tauri.conf.json");
const last_version = context.match(/"version": "([\d+.]*\d+)"/)?.[1];

console.log("Last version: " + last_version);
const new_version = prompt("New version:")

// 更新 tauri.conf.json 文件中的版本号
context = context.replace(/"version": "([\d+.]*\d+)"/, `"version": "${new_version}"`);
await Deno.writeTextFile("src-tauri/tauri.conf.json", context);

// 更新 Cargo.toml 文件中的版本号
context = await Deno.readTextFile("src-tauri/Cargo.toml");
context = context.replace(/version = "([\d+.]*\d+)"/, `version = "${new_version}"`);
await Deno.writeTextFile("src-tauri/Cargo.toml", context);

// 更新 package.json 文件中的版本号
context = await Deno.readTextFile("package.json");
context = context.replace(/"version": "([\d+.]*\d+)"/, `"version": "${new_version}"`);
await Deno.writeTextFile("package.json", context);
