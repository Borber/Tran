console.log("Start confirm language");

const lang_short = ["zh", "en", "ja",]
const lang_long = ["Chinese", "English", "Japanese",]

const lang = Deno.env.get("MATRIX_LANG")
const langs = lang!.split("_")

const first_short = langs[0]
const first_long = lang_long[lang_short.indexOf(first_short)]
const first_long_lower = first_long.toLowerCase()

const second_short = langs[1]
const second_long = lang_long[lang_short.indexOf(second_short)]
const second_long_lower = second_long.toLowerCase()

let content = await Deno.readTextFile("src-tauri/Cargo.toml");
content = content.replace("chinese", first_long_lower);
content = content.replace("english", second_long_lower);
await Deno.writeTextFile("src-tauri/Cargo.toml", content);

content = await Deno.readTextFile("src-tauri/src/lang.rs");
content = content.replace("zh", first_short);
content = content.replace("en", second_short);
content = content.replaceAll("Chinese", first_long);
content = content.replaceAll("English", second_long);
await Deno.writeTextFile("src-tauri/src/lang.rs", content);

console.log("End confirm language");

