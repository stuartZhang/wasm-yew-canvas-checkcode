use ::gloo::utils;
use ::wasm_bindgen::{JsCast, JsValue};
use ::web_sys::{Document, HtmlBodyElement, HtmlDivElement};

pub fn build_anchor_element() -> Result<HtmlDivElement, JsValue> {
    // 网页 DOM 上下文全局对象
    let document = utils::document();
    let head = document.head().ok_or("未运行于浏览器环境内，没有 head DOM 结点")?;
    let body = utils::body().dyn_into::<HtmlBodyElement>()?;
    // 添加样式
    let _style = head.query_selector("style[data-wasm-yew-canvas-checkcode]")?.ok_or("没有现成的样式dom元素").or_else(|_| {
        let style = document.create_element("style")?;
        head.append_child(&style)?;
        style.set_attribute("data-wasm-yew-canvas-checkcode-test", "")?;
        style.set_text_content(Some(include_str!("wasm_yew_canvas_checkcode.css")));
        Ok::<_, JsValue>(style)
    })?;
    // 向网页添加一个 yew 挂载锚点 DOM 元素
    let div_root = create_element::<HtmlDivElement>(&document, "div")?;
    div_root.class_list().add_1("root-element")?;
    body.append_child(&div_root)?;
    Ok(div_root)
}
fn create_element<T: JsCast>(document: &Document, tag_name: &str) -> Result<T, JsValue> {
    Ok(document.create_element(tag_name)?.dyn_into::<T>()?)
}