# About Best of RS

## Why Best of RS
> For 2025 as the birthday of Best of RS, Rust is everywhere.

As a Rust developer, I hope to create a place gathering all awesome rust projects and their state of Open Source.

While platforms like `Lib.rs` exist, they can feel a bit technical-heavy and cold. I envisioned a more vibrant, colorful, and warmer space- perhaps with a cute Ferris?

Inspired by [`Best of JS`](https://bestofjs.org), I decided to build a Rust version, which is all in Rust.

## What it offers
Best of RS focuses on those awesome rust projects published on Github by tracing:

1. `Metadata` - Stars, Forks，Contributors, and Tags.
2. `Community Health` - Issues counts, creation date, and update frequency.

We track these metrics daily to visualize trends, serving both users and maintainers.
 - `For User`:  browsing the dynamic of a large Rust ecosystem, finding what kinds of repositories are hot. 
 - `For Maintainer`: Add your Rust project here to get its growth.
By visualizing charts and delta values, you can feel the **pulse** of Rust ecosystem-daily, weekly, monthly and yearly. That's the vision.

## How it works
To track these states, our data pipeline works as follows:

1. `Project List`: Manually maintain a curated list named `Projects`, aiming target repositories to track.
2. `Tracking Worker`: Basing on the Github API, a tracing worker takes daily `Snapshots` within the `Projects` list and collect the delta(changes).
3. `Trends UI`: These snapshots and deltas bring the `Trend` interface

## Community
Without Open source, Best of RS won't be there, and it was born to help the Rust ecosystem thrive.
Since the initial track list was curated from my personal viewsight, some great projects are bound to be missing!
Please recommend awesome projects by [Recommend One](https://github.com/zhiyanzhaijie/bestofrs/issues/new?template=recommend_repo.yaml).

## Grateful
Best of RS was heavily inspired and supported by:

1. [Best of JS](https://bestofjs.org) (Tracking workflow)
2. [zed.dev](https://zed.dev/) (UI design)
3. [Dioxus](https://dioxuslabs.com/) (Rust UI Powered)
4. [iA-Fonts](https://github.com/iaolo/iA-Fonts) (iA Writer fonts, OFL-1.1)

Special thanks to all the dependencies used in this project, and of course, [Ferris](https://rustacean.net/)!
