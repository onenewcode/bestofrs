# 关于 Best of RS

## 为什么做 Best of RS
> 在 2025 这个 Best of RS 的诞生之年，Rust 已无处不在。

作为一名 Rust 开发者，我希望打造一个地方，收集所有优秀的 Rust 项目及它们在开源世界中的状态。

虽然像 `Lib.rs` 这样的平台已经存在，但它们有时会显得偏高冷。我想象的是一个更有活力、更多彩、更亲和的空间，也许还会有一只可爱的 Ferris？

受到 [`Best of JS`](https://bestofjs.org) 的启发，我决定做一个 Rust 版本，而且全部使用 Rust 构建。

## 它能提供什么
Best of RS 通过追踪 Github 上发布的那些优秀 Rust 项目，重点关注：

1. `元数据` - Stars、Forks、Contributors 和 Tags。
2. `社区健康度` - Issues 数量、创建时间与更新频率。

我们每天追踪这些指标并可视化趋势，服务于开源用户与维护者。
 - `面向用户`：浏览庞大 Rust 生态的动态，发现哪些类型的仓库正在升温。
 - `面向维护者`：把你的 Rust 项目添加到这里，观察它的成长。
通过图表和增量值的可视化，你可以按日、周、月、年感受到 Rust 生态的**脉搏**。这就是我们的愿景。

## 它如何工作
为了追踪这些状态，我们的数据工作流如下：

1. `项目列表`：手动维护一份名为 `Projects` 的精选列表，作为目标追踪仓库。
2. `追踪 Worker`：基于 Github API，追踪 worker 会每天UTC时间对 `Projects` 列表中的项目生成 `Snapshots`，并收集增量（变化）。
3. `趋势 UI`：这些快照与增量共同构成 `Trend` 界面。

## 社区
没有开源，就不会有 Best of RS；而它的诞生也是为了给 Rust 生态添砖加瓦。
由于最初的追踪列表受限于我的个人视野，难免会遗漏一些优秀项目！
欢迎通过 [Recommend One](https://github.com/zhiyanzhaijie/bestofrs/issues/new?template=recommend_repo.yaml) 推荐你心中的好项目。

## 致谢
Best of RS 受到了以下项目的深度启发与支持：

1. [Best of JS](https://bestofjs.org)（追踪工作流）
2. [zed.dev](https://zed.dev/)（UI 设计）
3. [Dioxus](https://dioxuslabs.com/)（Rust UI 驱动）
4. [iA-Fonts](https://github.com/iaolo/iA-Fonts)（iA Writer 字体，OFL-1.1）

特别感谢本项目使用到的所有依赖，当然也包括 [Ferris](https://rustacean.net/)！
