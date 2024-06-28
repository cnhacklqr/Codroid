# Codroid

**WIP** Rust IDE on android(and linux).

## Todo List

- [x] HomePage
- [ ] AboutPage
- [ ] SettingsPage
- [x] AppBar
- [ ] Project Editor
- [ ] Rust Binary/Library Project Support
- [ ] Tauri Project Support
- [x] Project Creater(fronted)
- [ ] Project List(frontend)
- [ ] Project Manager(backend)
- [ ] Usable On Android
- [ ] Usable On Linux

## Build

```shell
git clone https://github.com/shadow3aaa/Codroid
cd Codroid

pnpm install
# android
pnpm tauri android build -t aarch64
# linux
pnpm tauri build
```

## Run Development Build

```shell
pnpm tauri dev
```

## Notice

Since tauri_specta requires a cargo test to generate bindings, you had better to execute `cargo test` after any modify of rust code before `pnpm tauri build`
