let content = await Deno.readTextFile("src-tauri/tauri.conf.json");
const last_version = content.match(/"version": "([\d+.]*\d+)"/)?.[1];

console.log("Last version: " + last_version);
const new_version = prompt("New version:")

// 更新 tauri.conf.json 文件中的版本号
content = content.replace(/"version": "([\d+.]*\d+)"/, `"version": "${new_version}"`);
await Deno.writeTextFile("src-tauri/tauri.conf.json", content);

// 更新 Cargo.toml 文件中的版本号
content = await Deno.readTextFile("src-tauri/Cargo.toml");
content = content.replace(/version = "([\d+.]*\d+)"/, `version = "${new_version}"`);
await Deno.writeTextFile("src-tauri/Cargo.toml", content);

// 更新 package.json 文件中的版本号
content = await Deno.readTextFile("package.json");
content = content.replace(/"version": "([\d+.]*\d+)"/, `"version": "${new_version}"`);
await Deno.writeTextFile("package.json", content);

// 更新 App.tsx 文件中的版本号
content = await Deno.readTextFile("src/App.tsx");
content = content.replace(/version != "([\d+.]*\d+)"/, `version != "${new_version}"`);
await Deno.writeTextFile("src/App.tsx", content);

// 更新 App.tsx 文件中的版本号
content = await Deno.readTextFile("src-tauri/src/tray.rs");
content = content.replace(/v([\d+.]*\d+)/, `v${new_version}`);
await Deno.writeTextFile("src-tauri/src/tray.rs", content);