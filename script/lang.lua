require("util")

print("Lua script start")

local lang_short = { "zh", "en", "jp", }
local lang_long = { "Chinese", "English", "Japanese", }

local Lang = os.getenv("MATRIX_LANG")
local Langs = Util.split(Lang, "-")

FirstShort = Langs[1]
FirstLong = lang_long[Util.indexOf(lang_short, FirstShort)]
FirstLongLower = string.lower(FirstLong)
SecondShort = Langs[2]
SecondLong = lang_long[Util.indexOf(lang_short, SecondShort)]
SecondLongLower = string.lower(SecondLong)

Root = io.popen("git rev-parse --show-toplevel"):read("*l")

File = Root .. "/src-tauri/Cargo.toml"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'chinese', FirstLongLower)
Context = string.gsub(Context, 'english', SecondLongLower)
io.open(File, "w"):write(Context)

File = Root .. "/src-tauri/src/lang.rs"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'zh', FirstShort)
Context = string.gsub(Context, 'en', SecondShort)
Context = string.gsub(Context, 'Chinese', FirstLong)
Context = string.gsub(Context, 'English', SecondLong)
io.open(File, "w"):write(Context)

print("Lua script end")
