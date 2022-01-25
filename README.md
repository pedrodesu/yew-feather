## yew-feather

[![crates.io version](https://img.shields.io/crates/v/yew-feather.svg?style=flat-square)](https://crates.io/crates/yew-feather)
[![crates.io downloads](https://img.shields.io/crates/d/yew-feather.svg?style=flat-square)](https://crates.io/crates/yew-feather)

#### What is yew-feather?

yew-feather is a collection of simply beautiful open source icons for Yew. Each icon is designed on a 24x24 grid with an emphasis on simplicity, consistency and readability.

#### Based on Feather Icons `v4.28.0`

https://feathericons.com/

### Usage

```rust
use yew::{function_component, html};
use yew_feather::camera::Camera;

#[function_component(App)]
fn app() -> Html {
    html! { <Camera /> }
}

fn main() {
    yew::start_app::<App>();
}
```

Icons can be configured with inline props:

```rust
<Camera color="red" size=48 />
```