Root = io.popen("git rev-parse --show-toplevel"):read("*l")
File = Root .. "/src-tauri/tauri.conf.json"
Context = io.open(File, "r"):read("*a")
LastVersion = string.match(Context, '"version": "([%d+.]*%d+)"')
io.write("Last version: " .. LastVersion .. "\n" .. "NewVersion: ")
NewVersion = io.read()

--- 更新 tauri.conf.json 文件中的版本号
Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

--- 更新 Cargo.toml 文件中的版本号
File = Root .. "/src-tauri/Cargo.toml"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'version = "([%d+.]*%d+)"', 'version = "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

--- 更新 package.json 文件中的版本号
File = Root .. "/package.json"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)
