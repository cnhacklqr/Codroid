Languages:[English](/README.md) [中文](/README_zh.md)

# Codroid

## 简介

**未完成** 安卓上的 Rust 集成开发环境。

## 待办列表

- [x] 主页
- [ ] 关于页
- [ ] 设置页
- [x] 应用栏
- [ ] 项目编辑器
- [ ] Rust 二进制/库 项目支持
- [ ] Tauri 项目支持
- [x] 项目创建器（前端）
- [ ] 项目列表（前端）
- [ ] 项目管理器（后端）
- [ ] 可用于 Android
- [ ] 可用于 Linux

## 构建

如果你的网络可以正常以 https 方式访问 Github：

```shell
git clone https://github.com/shadow3aaa/Codroid
cd Codroid
```

如果你必须使用 ssh 方式访问：

```shell
git clone git@github.com:shadow3aaa/Codroid
cd Codroid
```

接下来，构建：

```shell
pnpm install
# Android
pnpm tauri android build -t aarch64
# Linux
pnpm tauri build
```

## 运行开发编译

```shell
pnpm tauri dev
```

## 注意

由于tauri_specta需要运行`cargo test`来生成绑定，您在修改Rust代码之后最好先执行`cargo test`，然后再运行`pnpm tauri build`。
