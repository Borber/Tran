[private]
default:
    @just --list

# 编译 GUI
gb:
    pnpm tauri build

# 调试 GUI
gd:
    pnpm tauri dev

# 更新 GUI 依赖
up:
    pnpm up --interactive --latest; \
    cd ./src-tauri; \
    cargo update
