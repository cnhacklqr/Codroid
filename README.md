# Codroid

**WIP** Rust ide on android(and linux).

## Todo List

- [x] HomePage
- [ ] AboutPage
- [ ] SettingsPage
- [x] AppBar
- [ ] Project Editor
- [ ] Rust Binary/Library Project Support
- [ ] Tauri Project Support
- [x] Project Creater(fronted)
- [ ] Project Creater(backend)
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

## Debug

```shell
pnpm tauri dev
```
