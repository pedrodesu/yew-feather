## yew-feather

[![crates.io version](https://img.shields.io/crates/v/yew-feather.svg?style=flat-square)](https://crates.io/crates/yew-feather)
[![crates.io downloads](https://img.shields.io/crates/d/yew-feather.svg?style=flat-square)](https://crates.io/crates/yew-feather)

#### What is yew-feather?

yew-feather is a collection of simply beautiful open source icons for Yew. Each icon is designed on a 24x24 grid with an emphasis on simplicity, consistency and readability.

#### Based on Feather Icons `v4.28.0`

https://feathericons.com/

### Usage

```rust
use yew::prelude::{html, Component, Context, Html};
use yew_feather::camera::Camera;

struct Model {}

impl Component for Model
{
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self
    {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html
    {
        html! {
            <>
                <span>{"Hello, world!"}</span>
                <Camera />
            </>
        }
    }
}

pub fn main()
{
    yew::start_app::<Model>();
}
```

Icons can be configured with inline props:

```rust
<Camera size=48 />
```
