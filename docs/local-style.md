# 维护 local-style 分支

`local-style` 是在官方 Waku 主干上叠加本地体验改动的构建分支。它的目标是：官方发布或合入新代码后，只同步 `upstream/main`、将本地提交 rebase 到新主干、运行验证，然后通过 GitHub Actions 打出带本地样式的 macOS App。

## 分支与远程约定

| 名称 | 用途 |
| --- | --- |
| `upstream` | 官方仓库 `git@github.com:egoist/waku.git`，只用来获取官方更新。 |
| `origin` | 个人 fork `git@github.com:luo9/waku.git`，保存可构建的分支。 |
| `main` | 与 `upstream/main` 一致的官方基线。不要直接加入本地功能。 |
| `local-style` | 从 `main` 派生，保存本地字体、排版、Mermaid 和构建 workflow。日常构建此分支。 |

首次检查远程与分支跟踪关系：

```bash
rtk git remote -v
rtk git branch -vv
```

期望 `main` 跟踪 `upstream/main`，`local-style` 跟踪 `origin/local-style`。若本地还没有官方远程，可执行：

```bash
rtk git remote add upstream git@github.com:egoist/waku.git
rtk git config branch.main.remote upstream
rtk git config branch.main.merge refs/heads/main
rtk git config remote.pushDefault origin
```

## 同步官方新版

先确保工作区没有未提交内容。下面的流程不会重写 `main`，并会用 fast-forward 限制避免意外合并：

```bash
rtk git switch main
rtk git fetch upstream --tags
rtk git merge --ff-only upstream/main
rtk git push origin main
```

检查本地样式分支相对新基线的提交：

```bash
rtk git log --oneline main..local-style
rtk git diff --stat main...local-style
```

然后将本地改动重新应用到新主干：

```bash
rtk git switch local-style
rtk git rebase main
```

rebase 成功后，提交哈希会改变，因此使用带保护的强推：

```bash
rtk git push --force-with-lease origin local-style
```

不要用普通 `--force`。它可能覆盖其他设备刚推送的 `local-style` 提交。

## 解决 rebase 冲突

官方对 Markdown 渲染的改动常会与本地样式、Mermaid 改动落在同一文件。先查看当前 rebase 正在应用的提交和冲突文件：

```bash
rtk git status
rtk git diff
```

处理后继续：

```bash
rtk git add <已解决的文件>
rtk git rebase --continue
```

若尚未作出任何需要保留的冲突处理，可以放弃这次同步并回到 rebase 前：

```bash
rtk git rebase --abort
```

解决时遵循下面的保留原则：

- 保留官方 `src/md/render.rs` 的新块类型、数学渲染和上下文 API；在其上重新应用本地字体与间距，而不是用旧文件整体覆盖。
- 保留本地等宽字体级联：`SF Mono`，回退到 `PingFang SC`、`Maple Mono NF CN`。
- 保留本地 Markdown 行高：正文比例 `1.625`，代码块 `20px`。这是与 `feature/markdown-line-height-1-8` 一致的排版基线。
- 保留内联代码的 Codex 风格：`4.5px` 左右留白、`4px` 圆角、`1.5px` 上下内缩；以等宽字体和中性灰色胶囊区分，不使用棕色或语法高亮式字色。
- 保留浅色主题中间聊天列的纯白正文底色；它使用独立的 `Theme::transcript`，不要把通用 `surface` 变白，否则右侧面板和设置页会丢失层级。
- 保留 Mermaid 的后台 SVG 缓存。SVG 解析必须停留在 `prepare_mermaid` 的后台任务中；行构建和 `render` 路径只能读取缓存。
- 官方新增 `Block` 枚举变体时，更新 `mermaid_sources` 的穷尽匹配。非代码块应显式跳过；例如 `Block::DisplayMath { .. }`。否则 Rust 会以 `E0004` 阻止构建。

本地功能主要涉及以下位置，适合在冲突时优先审阅：

| 功能 | 文件 |
| --- | --- |
| Markdown 字体、行高、段距、列表和 Mermaid 渲染 | `src/md/render.rs` |
| 消息、推理和工具输出的等宽字体 | `src/app/transcript_view.rs` |
| Mermaid 图片预览入口 | `src/app.rs`、`src/app/right_panel.rs`、`src/app/skills_page.rs` |
| 手动 macOS 打包 | `.github/workflows/build-macos-app.yml` |

## 添加新的本地改动

所有本地功能都应在 `local-style` 上以小提交保存，不要修改 `main`：

```bash
rtk git switch local-style
# 编辑并验证
rtk git add <文件>
rtk git commit -m 'local: 描述改动'
rtk git push origin local-style
```

尽量让每个提交只解决一个主题。这样官方升级冲突时可以逐个 rebase、跳过或重做，而不会混淆字体、Mermaid 与 CI 改动。

## 本地验证

每次 rebase 或修改渲染代码后，至少运行：

```bash
rtk rustfmt --edition 2024 --check src/md/render.rs src/app/transcript_view.rs
rtk git diff --check
rtk cargo test md::render::tests --lib
```

Waku 的 macOS 依赖会调用 Metal 编译器。若最后一条在本机因 `xcrun: error: unable to find utility "metal"` 失败，先确认完整 Xcode 已安装并被选中：

```bash
rtk xcrun --find metal
```

`rustfmt` 和 `git diff --check` 仍应通过；不要把缺少本机 Xcode 误判成 Rust 源码错误。

## 通过 GitHub Actions 打包

本地 workflow 位于 `.github/workflows/build-macos-app.yml`，名称为 **Build local macOS app**。它是手动触发的，因此单纯 `git push` 不会创建构建任务。

1. 打开 GitHub 仓库的 **Actions** 页面。
2. 选择 **Build local macOS app**。
3. 点击 **Run workflow**，分支选择 `local-style`。
4. 完成后下载 artifact `Waku-macos-arm64`，解压得到 `Waku.app`。

该 workflow 使用 `macos-15` 的 arm64 runner。CUA 的 `apple-metal` 依赖需要 macOS 15 SDK；不要降回 `macos-14`，否则会在 Swift 编译阶段缺少 `MTLLogState` 等新 Metal API。构建产物保留 14 天。

## 发布前检查清单

```bash
rtk git switch local-style
rtk git status --short --branch
rtk git log --oneline main..HEAD
rtk git diff --check main...HEAD
```

确认工作区干净、`local-style` 已推送、所有本地提交都能从 `main..HEAD` 看见后，再触发手动构建。若 GitHub 构建报错，以 workflow 日志中第一个编译错误为准；警告不应掩盖真正的失败原因。
