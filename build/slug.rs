#[function_component(__ComponentName)]
pub fn __component_func(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            __component_markup
        </svg>
    }
}
