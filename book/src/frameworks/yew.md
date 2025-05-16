# Lucide Yew

Implementation of the Lucide icon library for [Yew](https://yew.rs/) applications.

## Installation

Install the icons from your command line.

```shell
# Selective categories (see https://lucide.dev/icons/categories)
cargo add lucide-yew --features accessibility,communication

# All categories
cargo add lucide-yew --features all-icons
```

- [View on crates.io](https://crates.io/crates/lucide-yew)
- [View on docs.rs](https://docs.rs/lucide-yew/latest/lucide_yew/)
- [View source](https://github.com/RustForWeb/lucide/tree/main/packages/yew)

## Usage

```rust,ignore
use lucide_yew::Camera;
use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <Camera color="red" size=48 />
    }
}
```

## Props

| Name                    | Type                | Default          |
| ----------------------- | ------------------- | ---------------- |
| `size`                  | `usize`             | `24`             |
| `color`                 | `AttrValue`         | `"currentColor"` |
| `fill`                  | `AttrValue`         | `"none"`         |
| `stroke_width`          | `usize`             | `2`              |
| `absolute_stroke_width` | `bool`              | `false`          |
| `class`                 | `Classes`           | -                |
| `style`                 | `Option<AttrValue>` | -                |

## Icons

```toml,trunk
package = "lucide-yew-book"
features = ["icons"]
files = ["src/icons.rs"]
```
