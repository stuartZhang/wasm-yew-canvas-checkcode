mod utils;
use ::gloo::console;
use ::wasm_bindgen::{JsValue, UnwrapThrowExt};
use ::wasm_bindgen_test::*;
use ::wasm_yew_canvas_checkcode::{CanvasCheckCode, Props as CanvasCheckCodeProps, CheckCode};
use ::yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn page_dom() {
    page_dom_inner().unwrap_throw()
}
fn page_dom_inner() -> Result<(), JsValue> {
    let div_root = utils::build_anchor_element()?;
    // 类似于 Vue 的 new Vue({...配置项}).$mount('#app');
    Renderer::<CanvasCheckCode>::with_root_and_props(div_root.into(), yew::props![CanvasCheckCodeProps {
        on_check_code_change: |check_code| {
            let check_code = match check_code {
                CheckCode::Initialize(value) => value,
                CheckCode::Update(value) => value,
            };
            console::info!("从父组件收到的校验码", check_code);
        }
    }]).render();
    Ok(())
}
