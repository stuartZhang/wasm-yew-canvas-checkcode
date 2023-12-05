
/// js 版小样，见：https://codepen.io/stuartZhang/pen/oNQzygO?editors=0010
#[cfg(debug_assertions)]
use ::gloo::console;
mod canvas_options;
use ::rand::{Rng, rngs::OsRng};
use ::wasm_bindgen::JsValue;
use ::web_sys::CanvasRenderingContext2d;
pub use canvas_options::CanvasOpts;
struct Point(f64, f64);
pub fn redraw(context: CanvasRenderingContext2d, canvas_opts: CanvasOpts, text: &str) -> Result<(), JsValue> {
    context.clear_rect(0_f64, 0_f64, canvas_opts.width, canvas_opts.height);
    let background_color = draw_background_color(&context, &canvas_opts);
    #[cfg(debug_assertions)]
    console::info!("canvas_opts=", canvas_opts.to_string(), "background_color=", background_color, "text=", text);
    draw_stars(&context, &canvas_opts, background_color)?;
    draw_text(&context, &canvas_opts, text)?;
    Ok(())
}
fn draw_background_color(context: &CanvasRenderingContext2d, canvas_opts: &CanvasOpts) -> &'static str {
    context.save();
    let background_color = calc_color();
    context.set_fill_style(&background_color.into());
    context.fill_rect(0_f64, 0_f64, canvas_opts.width, canvas_opts.height);
    context.restore();
    return background_color;
    fn calc_color() -> &'static str {
        const COLORS: [&str; 3] = [
            "rgba(232, 232, 232, 1)",
            "rgba(242, 242, 242, 1)",
            "rgba(252, 252, 252, 1)"
        ];
        let index = OsRng.gen_range(0..COLORS.len());
        COLORS[index]
    }
}
fn draw_stars(context: &CanvasRenderingContext2d, canvas_opts: &CanvasOpts, background_color: &str) -> Result<(), JsValue> {
    let mut points: Vec<Point> = vec![];
    let mut degrees: Vec<f64> = vec![];
    let mut rgbas = vec![background_color.to_string()];
    let space_threshold = canvas_opts.star_size * 2_f64;
    for _ in 0..canvas_opts.star_count {
        let point = match calc_point_rel_to(&points, space_threshold, canvas_opts) {
            Some(point) => point,
            None => break
        };
        let degree = calc_rotation_rel_to(&degrees);
        let rgba = calc_color_rel_to(&rgbas);
        let corner_count = calc_corner_count();
        draw_star(context, canvas_opts.star_size, &point, degree, &rgba[..], corner_count)?;
        points.push(point);
        degrees.push(degree);
        rgbas.push(rgba);
    }
    return Ok(());
    fn calc_point_rel_to(points: &Vec<Point>, space_threshold: f64, canvas_opts: &CanvasOpts) -> Option<Point> {
        let mut index = 0_u16;
        return loop {
            if index > 600_u16 {
                break None;
            }
            index += 1;
            let point = Point(
                OsRng.gen_range(canvas_opts.star_size..canvas_opts.viewport_width() + 0.1),
                OsRng.gen_range(canvas_opts.star_size..canvas_opts.viewport_height() + 0.1)
            );
            if points.len() == 0 {
                break Some(point);
            }
            let min_space = match points.iter().map(calc_distance_builder(&point)).min_by(|a, b| a.total_cmp(b)) {
                Some(min_space) => min_space,
                None => break Some(point)
            };
            if min_space >= space_threshold {
                break Some(point);
            }
        };
        fn calc_distance_builder<'a>(point1: &'a Point) -> impl Fn(&'a Point) -> f64 {
            move |point2| {
                ((point1.0 - point2.0).powf(2_f64) + (point1.1 - point2.1).powf(2_f64)).sqrt()
            }
        }
    }
    fn calc_rotation_rel_to(degrees: &Vec<f64>) -> f64 {
        loop {
            let degree = OsRng.gen_range(0_f64..360_f64);
            if degrees.len() == 0 {
                break degree;
            }
            if !degrees.contains(&degree) {
                break degree;
            }
        }
    }
    fn calc_color_rel_to(colors: &Vec<String>) -> String {
        let opacity = format!("{:.2}", 0.4 + OsRng.gen_range(0_f64..0.31_f64));
        return loop {
            let red = OsRng.gen_range(0_u16..256_u16).to_string();
            let green = OsRng.gen_range(0_u16..256_u16).to_string();
            let blue = OsRng.gen_range(0_u16..256_u16).to_string();
            let rgba = format!("rgba({red}, {green}, {blue}, {opacity})");
            if !colors.iter().any(equals_color_builder(&rgba[..])) {
                break rgba;
            }
        };
        fn equals_color_builder<'a>(rgba1: &'a str) -> impl FnMut(&'a String) -> bool {
            move |rgba2| {
                let index1 = match rgba1.rfind(',') {
                    Some(index) => index,
                    None => rgba1.len()
                };
                let index2 = match rgba2.rfind(',') {
                    Some(index) => index,
                    None => rgba2.len()
                };
                rgba1[0..index1] == rgba2[0..index2]
            }
        }
    }
    fn calc_corner_count() -> u8 {
        3_u8 + OsRng.gen_range(0_u8..5_u8)
    }
    fn draw_star(context: &CanvasRenderingContext2d, long_radius: f64, point: &Point, degree: f64, rgba: &str, corner_count: u8) -> Result<(), JsValue> {
        let short_radius = long_radius / 2.5;
        let c1 = 360_f64 / corner_count as f64;
        let c2 = 90_f64 / corner_count as f64;
        let c3 = 270_f64 / corner_count as f64;
        context.save();
        context.translate(point.0, point.1)?;
        context.rotate(degree.to_radians())?;
        context.set_fill_style(&rgba.into());
        context.begin_path();
        for i in 0..corner_count {
            let i = i as f64;
            context.line_to(
                (c2 + i * c1).to_radians().cos() * long_radius,
                (c2 + i * c1).to_radians().sin() * long_radius
            );
            context.line_to(
                (c3 + i * c1).to_radians().cos() * short_radius,
                (c3 + i * c1).to_radians().sin() * short_radius
            );
        }
        context.close_path();
        context.fill();
        context.restore();
        Ok(())
    }
}
fn draw_text(context: &CanvasRenderingContext2d, canvas_opts: &CanvasOpts, text: &str) -> Result<(), JsValue> {
    let mut acc_width = canvas_opts.star_size;
    let width_unit = canvas_opts.viewport_width() / text.len() as f64;
    let half_width_unit = width_unit / 2_f64;
    let middle_y = canvas_opts.height / 2_f64;
    context.save();
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_font(&format!("{} normal bolder {}px Arial icon", calc_font_style(), canvas_opts.font_size)[..]);
    for char in text.chars() {
        let m = context.measure_text(&char.to_string()[..])?;
        context.save();
        context.translate(
            acc_width + half_width_unit + OsRng.gen_range(0_f64..m.width() / 5_f64 + 0.001_f64).copysign(OsRng.gen_range(-0.1..0.1)),
            middle_y + OsRng.gen_range(0_f64..canvas_opts.font_size / 2_f64 + 0.001_f64).copysign(OsRng.gen_range(-0.1..0.1))
        )?;
        context.rotate(OsRng.gen_range(0_f64..20.001_f64).to_radians().copysign(OsRng.gen_range(-0.3..0.1)))?;
        context.set_fill_style(&calc_color().into());
        context.fill_text(&char.to_string()[..], 0_f64, 0_f64)?;
        context.restore();
        acc_width += width_unit;
    }
    context.restore();
    return Ok(());
    fn calc_font_style() -> &'static str {
        const STYLES: [&str; 3] = [
            "normal",
            "italic",
            "oblique"
        ];
        STYLES[OsRng.gen_range(0..STYLES.len())]
    }
    fn calc_color() -> &'static str {
        const COLORS: [&str; 4] = [
            "rgba(255, 0, 0, 1)",
            "rgba(0, 100, 0, 1)",
            "rgba(0, 0, 255, 1)",
            "rgba(0, 0, 0, 1)"
        ];
        COLORS[OsRng.gen_range(0..COLORS.len())]
    }
}
