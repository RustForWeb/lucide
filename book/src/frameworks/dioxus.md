# Lucide Dioxus

Implementation of the Lucide icon library for [Dioxus](https://dioxuslabs.com/) applications.

## Installation

```shell
cargo add lucide-dioxus
```

-   [View on crates.io](https://crates.io/crates/lucide-dioxus)
-   [View on docs.rs](https://docs.rs/lucide-dioxus/latest/lucide_dioxus/)
-   [View source](https://github.com/RustForWeb/lucide/tree/main/packages/dioxus)

## Usage

```rust,ignore
use dioxus::prelude::*;
use lucide_dioxus::Camera;

#[component]
fn App() -> Element {
    rsx! {
        Camera {
            color: "red",
            size: 48,
        }
    }
}
```

## Props

| Name                    | Type     | Default          |
| ----------------------- | -------- | ---------------- |
| `size`                  | `usize`  | `24`             |
| `color`                 | `String` | `"currentColor"` |
| `fill`                  | `String` | `"none"`         |
| `stroke_width`          | `usize`  | `2`              |
| `absolute_stroke_width` | `bool`   | `false`          |

## Icons

```toml,trunk
package = "lucide-dioxus-book"
features = ["icons"]
files = ["src/icons.rs"]
```
