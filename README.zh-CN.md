# herdr 简体中文维护版

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="#一键安装">一键安装</a> · <a href="#语言切换">语言切换</a> · <a href="#升级">升级</a> · <a href="https://herdr.dev/zh-cn/docs/">上游文档</a> · <a href="README.md">English</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 license" /></a>
  <a href="https://github.com/AoManoh/herdr/releases"><img src="https://img.shields.io/github/downloads/AoManoh/herdr/total?label=downloads&labelColor=333333&color=666666" alt="fork release downloads" /></a>
  <a href="https://github.com/AoManoh/herdr/releases/latest"><img src="https://img.shields.io/github/v/release/AoManoh/herdr?label=中文维护版&labelColor=333333&color=666666" alt="latest Chinese fork release" /></a>
</p>

> 本仓库是由 AoManoh 维护的 herdr 简体中文版本。当前版本基于上游 [`v0.9.1`](https://github.com/herdrdev/herdr/releases/tag/v0.9.1)，保留上游功能并增加可切换的 English / 简体中文 TUI 与 plugin CLI 汉化。它不是 herdr 官方发布渠道。

## 主要差异

- 内置简体中文 TUI，覆盖工作区、窗格、设置、快捷键、通知、对话框和移动布局。
- plugin CLI 的帮助、安装预览、列表、提示和构建诊断会跟随界面语言，JSON/协议字段保持不变。
- 设置面板提供 `语言 / Language` 分区，可即时预览并写回配置。
- 支持本地与已保存 SSH 机器的统一工作区/智能体视图，远程机器状态会跟随连接状态显示。
- 中文显示宽度、按钮区域和鼠标命中使用终端列宽计算。
- fork 使用独立 GitHub Release，不会修改 herdr.dev 或官方更新清单。

## 一键安装

适用于 Linux、WSL 和 macOS：

```bash
curl -fsSL https://github.com/AoManoh/herdr/releases/latest/download/install.sh | sh
```

安装器会：

1. 识别 Linux/macOS 和 x86_64/aarch64。
2. 从 `AoManoh/herdr` 最新 Release 下载对应二进制。
3. 使用 `checksums.txt` 校验 SHA-256。
4. 安装到 `~/.local/bin/herdr`，也可通过 `HERDR_INSTALL_DIR` 更改目录。
5. 首次安装时默认启用简体中文。
6. 检测到默认 session 正在运行时，通过 live-handoff 保留 panes 和 agents。

如果 `~/.local/bin` 不在 `PATH` 中：

```bash
export PATH="$HOME/.local/bin:$PATH"
```

安装指定版本：

```bash
curl -fsSL https://github.com/AoManoh/herdr/releases/latest/download/install.sh \
  | HERDR_VERSION=zh-v0.9.1.1 sh
```

Windows 原生版本暂不通过此 shell 安装器分发；Windows 用户推荐在 WSL 中使用。

### 手动下载与校验

不希望执行远程脚本时，可从 [Releases](https://github.com/AoManoh/herdr/releases) 下载：

| 系统 | 架构 | 资产 |
| --- | --- | --- |
| Linux / WSL | x86_64 | `herdr-linux-x86_64` |
| Linux | aarch64 | `herdr-linux-aarch64` |
| macOS | Intel | `herdr-macos-x86_64` |
| macOS | Apple Silicon | `herdr-macos-aarch64` |

下载同一 Release 中的 `checksums.txt`，然后校验：

```bash
sha256sum -c checksums.txt --ignore-missing
```

macOS 可使用：

```bash
shasum -a 256 herdr-macos-aarch64
```

## 启动与基本使用

在项目目录运行：

```bash
herdr
```

常用操作：

- `Ctrl+B`：进入前缀模式。
- `Ctrl+B`，然后 `S`：打开设置。
- `Ctrl+B`，然后 `Q`：分离客户端，pane 中的任务继续运行。
- 再次执行 `herdr`：重新附加。
- 鼠标可用于切换工作区、选择窗格、拖动边界和打开右键菜单。

## 语言切换

打开设置：

```text
Ctrl+B → S → 语言 / Language
```

可选：

- `English`
- `简体中文`

上下键或鼠标选择后会立即预览；按 Enter 或点击“应用”写入 `config.toml`；按 Esc 或点击“关闭”会撤销尚未应用的预览。

也可以直接编辑：

```toml
[ui]
language = "zh-cn" # 或 "en"
```

然后执行：

```bash
herdr server reload-config
```

## 升级

重新运行一键安装命令即可：

```bash
curl -fsSL https://github.com/AoManoh/herdr/releases/latest/download/install.sh | sh
```

若默认 session 正在运行，安装器会备份旧二进制并执行 live-handoff。升级命令应在 Herdr TUI 外的普通终端中运行；如果在 Herdr pane 内执行，安装器会拒绝升级。客户端可能返回 shell，再执行一次 `herdr` 即可重新附加；pane 中的进程不会因正常 handoff 而重启。

如果存在运行中的命名 session，安装器会拒绝自动覆盖，避免不同 session 之间出现协议不一致。应先逐个 handoff，或在所有命名 session 结束后升级。

不要使用：

```bash
herdr update
```

该命令仍指向 herdr 官方更新渠道，可能将中文维护版覆盖为官方版本。

## 版本发布与升级说明

中文维护版的代码分支和 GitHub Release 是两回事。推送代码或创建 `zh-v...` 标签后，GitHub Actions 还需要完成跨平台构建并上传 Release 资产；构建完成前，安装器不能下载这个版本。

- `releases/latest/download/...` 只指向最近一个已经发布完成的 Release，不指向最新 commit，也不指向刚推送的标签。
- 指定 `HERDR_VERSION` 时，标签必须已经有 `checksums.txt` 和对应平台的二进制文件。返回 404 通常表示 Release 还没有发布完成；检查 [Chinese Fork Release](https://github.com/AoManoh/herdr/actions/workflows/zh-release.yml) 的运行状态。
- 发布完成后，升级最新版本：

  ```bash
  curl -fsSL https://github.com/AoManoh/herdr/releases/latest/download/install.sh | sh
  ```

- 固定安装某个版本：

  ```bash
  curl -fsSL https://github.com/AoManoh/herdr/releases/latest/download/install.sh \
    | HERDR_VERSION=zh-v0.9.1.1 sh
  ```

  `HERDR_VERSION=...` 必须放在执行安装器的 `sh` 前面。不要把安装器的输出再通过管道交给另一个 `sh`，例如不要写成 `curl ... | sh | HERDR_VERSION=... sh`。

## 从源码构建

普通用户推荐使用 Release 二进制。需要开发或审查源码时：

```bash
git clone https://github.com/AoManoh/herdr.git
cd herdr
cargo build --release --locked
```

构建环境由 `rust-toolchain.toml` 固定为 Rust 1.96.1，并需要 Zig 0.16.0。完整验证：

```bash
just check
```

仓库维护方式：

- `upstream` 指向 `herdrdev/herdr`。
- fork `master` 包含中文维护改动并定期同步上游。
- 中文发布标签使用 `zh-v<上游基础版本>.<中文修订号>`，例如 `zh-v0.9.1.1`。

## 上游文档

完整功能文档仍以上游为准：

- [快速开始](https://herdr.dev/zh-cn/docs/quick-start/)
- [核心概念](https://herdr.dev/zh-cn/docs/concepts/)
- [键盘](https://herdr.dev/zh-cn/docs/keyboard/)
- [配置](https://herdr.dev/zh-cn/docs/configuration/)
- [会话状态与恢复](https://herdr.dev/zh-cn/docs/session-state/)
- [连接机器](https://herdr.dev/zh-cn/docs/connecting-machines/)
- [远程访问](https://herdr.dev/zh-cn/docs/persistence-remote/)
- [插件](https://herdr.dev/zh-cn/docs/plugins/)
- [Socket API](https://herdr.dev/zh-cn/docs/socket-api/)

## 贡献与维护

提交代码前请阅读 [`AGENTS.md`](./AGENTS.md) 和 [`CONTRIBUTING.md`](./CONTRIBUTING.md)。中文维护版的问题应提交到 `AoManoh/herdr`；上游通用问题请先确认能否在官方版本复现，再遵循上游贡献规则。

## 许可证与致谢

本 fork 根据 [Apache License 2.0](LICENSE) 分发。感谢 herdr 作者及所有上游贡献者；项目来源和官方发布见 [`herdrdev/herdr`](https://github.com/herdrdev/herdr)。
