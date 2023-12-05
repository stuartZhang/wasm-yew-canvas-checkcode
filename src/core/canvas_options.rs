use ::std::fmt::Display;
use ::wasm_bindgen::JsValue;
use ::web_sys::{HtmlCanvasElement, Window};
use crate::Props;
#[derive(Debug)]
pub struct CanvasOpts {
    pub width: f64,
    pub height: f64,
    pub star_size: f64,
    pub star_count: u8,
    pub font_size: f64
}
impl CanvasOpts {
    pub fn with_canvas(window: &Window, canvas: &HtmlCanvasElement, props: &Props) -> Result<Self, JsValue> {
        let parse_int_builder = |default: f64| move |str: String| -> f64 {
            if str.is_empty() {
                return default;
            }
            let str = str.find(char::is_alphabetic).map_or(&str[..], |index| &str[0..index]);
            str.parse::<f64>().map_or(default, |v| v)
        };
        let styles = window.get_computed_style(canvas)?.ok_or("浏览器不支持 CssStyleDeclaration")?;
        let canvas_width = styles.get_property_value("width").map(parse_int_builder(props.width))?;
        let canvas_height = styles.get_property_value("height").map(parse_int_builder(props.height))?;
        Ok(CanvasOpts {
            width: canvas_width,
            height: canvas_height,
            star_size: props.star_size,
            star_count: props.star_count,
            font_size: props.font_size,
        })
    }
    pub fn viewport_width(&self) -> f64 {
        self.width - self.star_size * 2_f64
    }
    pub fn viewport_height(&self) -> f64 {
        self.height - self.star_size * 2_f64
    }
}
impl Display for CanvasOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}