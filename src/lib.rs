mod core;

#[cfg(debug_assertions)]
use ::gloo::console;
use ::gloo::utils;
use ::rand::{Rng, rngs::OsRng};
use ::wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use ::web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use ::yew::{Callback, Component, Context, html, html::Scope, Html, NodeRef, Properties};
use core::CanvasOpts;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(150.0)]
    pub width: f64,
    #[prop_or(50.0)]
    pub height: f64,
    #[prop_or(7.0)]
    pub star_size: f64,
    #[prop_or(25)]
    pub star_count: u8,
    #[prop_or(22.0)]
    pub font_size: f64,
    #[prop_or(5)]
    pub check_code_len: u8,
    pub on_check_code_change: Callback<CheckCode>,
    #[prop_or((|_| {}).into())]
    pub reversed_hook: Callback<Scope<CanvasCheckCode>>,
}
pub enum Message {
    UpdateCheckCode
}
pub enum CheckCode {
    Initialize(String),
    Update(String)
}
pub struct CanvasCheckCode {
    canvas_ref: NodeRef,
    unique_id: String,
}
macro_rules! draw_canvas {
    (@core $self: ident, $ctx: ident, $canvas: ident, $custom_canvas: block, $timing: ident) => {
        let props = $ctx.props();
        let check_code = gen_random_characters(props.check_code_len); //TODO: 如何将内部生成的检验码，输出到组件外;
        let $canvas = $self.canvas_ref.cast::<HtmlCanvasElement>().ok_or("未能获取 canvas 元素")?;
        $custom_canvas
        let window = utils::window();
        let canvas_opts = CanvasOpts::with_canvas(&window, &$canvas, props)?;
        $canvas.set_attribute("width", &format!("{}px", canvas_opts.width)[..])?;
        $canvas.set_attribute("height", &format!("{}px", canvas_opts.height)[..])?;
        core::redraw(
            $canvas.get_context("2d")?.ok_or("浏览器画布不支持 2D 渲染上下文")?.dyn_into::<CanvasRenderingContext2d>()?,
            canvas_opts,
            &check_code[..]
        )?;
        props.on_check_code_change.emit(CheckCode::$timing(check_code));
    };
    ($self: ident, $ctx: ident, $canvas: ident, $custom_canvas: block) => {
        draw_canvas!(@core $self, $ctx, $canvas, $custom_canvas, Initialize);
    };
    ($self: ident, $ctx: ident) => {
        draw_canvas!(@core $self, $ctx, canvas, {}, Update);
    };
}
impl CanvasCheckCode {
    fn init_canvas(&self, ctx: &Context<Self>) -> Result<(), JsValue> {
        let canvas_data_key = format!("data-{}", &self.unique_id[..]);
        draw_canvas!(self, ctx, canvas, {
            canvas.set_attribute(&canvas_data_key[..], "")?;
            canvas.class_list().add_1("wasm-yew-canvas-checkcode")?;
        });
        let document = utils::document();
        let head = document.head().ok_or("未运行于浏览器环境内，没有 head DOM 结点")?;
        // 添加样式
        let style_data_key = format!("data-wasm-yew-canvas-checkcode-{}", &self.unique_id[..]);
        let _style = head.query_selector(&format!("style[{}]", &style_data_key[..])[..])?.ok_or("没有现成的样式dom元素").or_else(|_| {
            let style = document.create_element("style")?;
            head.append_child(&style)?;
            style.set_attribute(&style_data_key[..], "")?;
            style.set_text_content(Some(&format!(include_str!("./wasm_yew_canvas_checkcode.css"), &canvas_data_key[..])[..]));
            Ok::<_, JsValue>(style)
        })?;
        Ok(())
    }
    fn update_canvas(&self, ctx: &Context<Self>) -> Result<(), JsValue> {
        draw_canvas!(self, ctx);
        Ok(())
    }
}
impl Component for CanvasCheckCode {
    type Message = Message;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let scope = ctx.link();
        props.reversed_hook.emit(scope.clone());
        Self {
            canvas_ref: NodeRef::default(),
            unique_id: gen_random_characters(16)
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::UpdateCheckCode => self.update_canvas(ctx).unwrap_throw()
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let scope = ctx.link();
        html! {
            <canvas ref={self.canvas_ref.clone()} onclick={
                scope.callback(move |event: MouseEvent| {
                    event.prevent_default();
                    event.stop_propagation();
                    event.stop_immediate_propagation();
                    #[cfg(debug_assertions)]
                    console::info!("刷新验证码");
                    Message::UpdateCheckCode
                })
            } />
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        self.init_canvas(ctx).unwrap_throw();
    }
}
fn gen_random_characters(count: u8) -> String {
    const CHARS: [char; 67] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z', '你', '我', '他', '她', '它'
    ];
    let mut characters = "".to_string();
    for _ in 0..count {
        characters.push(CHARS[OsRng.gen_range(0..CHARS.len())]);
    }
    characters
}