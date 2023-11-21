console.log("Start confirm language");

const lang_short = ["zh", "en", "jp",]
const lang_long = ["Chinese", "English", "Japanese",]

const lang = Deno.env.get("MATRIX_LANG")
const langs = lang!.split("_")

const first_short = langs[0]
const first_long = lang_long[lang_short.indexOf(first_short)]
const first_long_lower = first_long.toLowerCase()

const second_short = langs[1]
const second_long = lang_long[lang_short.indexOf(second_short)]
const second_long_lower = second_long.toLowerCase()

let context = await Deno.readTextFile("src-tauri/Cargo.toml");
context = context.replace("chinese", first_long_lower);
context = context.replace("english", second_long_lower);
await Deno.writeTextFile("src-tauri/Cargo.toml", context);

context = await Deno.readTextFile("src-tauri/src/lang.rs");
context = context.replace("zh", first_short);
context = context.replace("en", second_short);
context = context.replaceAll("Chinese", first_long);
context = context.replaceAll("English", second_long);
await Deno.writeTextFile("src-tauri/src/lang.rs", context);

console.log("End confirm language");

