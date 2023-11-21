[private]
default:
    @just --list

# 编译
b:
    pnpm tauri build

# 调试
d:
    pnpm tauri dev

# 更新版本
v:
    lua ./script/version.lua

# 更新 GUI 依赖
up:
    pnpm up --interactive --latest; \
    cd ./src-tauri; \
    cargo update
