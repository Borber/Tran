print("Lua script start")

local lang_short = { "cn", "en", "jp", }
local lang_long = { "Chinese", "English", "Japanese", }


Lang = os.getenv("MATRIX_LANG")

Langs = split(Lang, "-")

FirstShort = Langs[1]
FirstLong = lang_long[indexOf(lang_short, FirstShort)]
FirstLongLower = string.lower(FirstLong)
SecondShort = Langs[2]
SecondLong = lang_long[indexOf(lang_short, SecondShort)]
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
Context = string.gsub(Context, 'English', SecondShort)
io.open(File, "w"):write(Context)

print(Context)

print("Lua script end")


local function split(str, sep)
    local result = {}
    local regex = ("([^%s]+)"):format(sep)
    for each in str:gmatch(regex) do
        table.insert(result, each)
    end
    return result
end

local function indexOf(array, value)
    for i, v in ipairs(array) do
        if v == value then
            return i
        end
    end
    return nil
end
