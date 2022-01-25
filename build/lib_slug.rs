use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub class: &'static str,
    #[prop_or("24")]
    pub size: &'static str,
    #[prop_or("none")]
    pub fill: &'static str,
    #[prop_or("currentColor")]
    pub color: &'static str,
    #[prop_or("2")]
    pub stroke_width: &'static str,
    #[prop_or("round")]
    pub stroke_linecap: &'static str,
    #[prop_or("round")]
    pub stroke_linejoin: &'static str,
}
