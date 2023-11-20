Root = io.popen("git rev-parse --show-toplevel"):read("*l")

File = Root .. "/src-tauri/Cargo.toml"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'chinese', 'chinese')
Context = string.gsub(Context, 'english', 'japanese')
io.open(File, "w"):write(Context)

File = Root .. "/src-tauri/src/lang.rs"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'zh', 'zh')
Context = string.gsub(Context, 'en', 'ja')
Context = string.gsub(Context, 'Chinese', 'Chinese')
Context = string.gsub(Context, 'English', 'Japanese')
io.open(File, "w"):write(Context)
