mod utils;

use ::deferred_future::LocalDeferredFuture;
use ::gloo::dialogs;
use ::std::{cell::RefCell, rc::Rc};
use ::wasm_bindgen::UnwrapThrowExt;
use ::wasm_bindgen_test::*;
use ::wasm_yew_canvas_checkcode::{CanvasCheckCode, CheckCode, Message as CanvasCheckCodeMessage};
use ::web_sys::{InputEvent, HtmlInputElement, SubmitEvent};
use ::yew::{AttrValue, Callback, Component, Context, html, html::Scope, Html, Properties, Renderer, TargetCast};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(PartialEq, Properties)]
pub struct Props {
    on_submit: Callback<Result<(), ()>>
}
pub enum Message {
    ChangeUserName(AttrValue),
    ChangePassword(AttrValue),
    ChangeCheckCode(AttrValue),
    GenCheckCode(CheckCode),
    SubmitForm
}
#[derive(Default)]
struct App {
    user_name: AttrValue,
    password: AttrValue,
    check_code: AttrValue,
    gen_check_code: String,
    check_code_scope: Rc<RefCell<Option<Scope<CanvasCheckCode>>>>
}
impl Component for App {
    type Message = Message;
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Message::ChangeUserName(value) => self.user_name = value,
            Message::ChangePassword(value) => self.password = value,
            Message::ChangeCheckCode(value) => self.check_code = value,
            Message::GenCheckCode(value) => {
                match value {
                    CheckCode::Initialize(value) => self.gen_check_code = value,
                    CheckCode::Update(value) => {
                        self.check_code = "".into();
                        self.gen_check_code = value;
                    }
                }
                return true;
            },
            Message::SubmitForm => {
                if self.check_code.eq(&self.gen_check_code[..]) {
                    dialogs::alert(&format!(r#"
                        图形验证码输入正确，继续提交表单
                        用户名：{}
                        密码：{}
                    "#, &self.user_name, &self.password));
                    props.on_submit.emit(Ok(()));
                } else {
                    dialogs::alert("图形验证码输入错误");
                    props.on_submit.emit(Err(()));
                }
                self.check_code_scope.borrow().as_ref().map(|scope| {
                    scope.send_message(CanvasCheckCodeMessage::UpdateCheckCode);
                });
            }
        }
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let scope = ctx.link();
        let check_code_scope = Rc::clone(&self.check_code_scope);
        macro_rules! on_input {
            ($variant: ident) => {
                scope.callback(|event: InputEvent| {
                    let input = event.target_unchecked_into::<HtmlInputElement>();
                    Message::$variant(input.value().into())
                })
            };
        }
        html! {
            <form onsubmit={scope.callback(|event: SubmitEvent| {
                event.prevent_default();
                event.stop_propagation();
                Message::SubmitForm
            })}>
                <label>
                    <span>{"用户名："}</span>
                    <input type="text" name="username" placeholder="请输入用户名" value={&self.user_name} oninput={on_input!(ChangeUserName)} />
                </label>
                <label>
                    <span>{"密码："}</span>
                    <input type="password" name="password" placeholder="请输入密码" value={&self.password} oninput={on_input!(ChangePassword)} />
                </label>
                <label>
                    <span>{"验证码："}</span>
                    <input type="text" name="checkcode" placeholder="请输入验证码" value={&self.check_code} oninput={on_input!(ChangeCheckCode)} />
                </label>
                <label><CanvasCheckCode on_check_code_change={scope.callback(Message::GenCheckCode)} reversed_hook={move |scope| {
                    check_code_scope.borrow_mut().replace(scope);
                }} /></label>
                <label><button type="submit">{"登录"}</button></label>
            </form>
        }
    }
}
#[wasm_bindgen_test]
async fn page_dom() {
    let deferred_future = LocalDeferredFuture::default();
    let defer = deferred_future.defer();
    let div_root = utils::build_anchor_element().unwrap_throw();
    // 类似于 Vue 的 new Vue({...配置项}).$mount('#app');
    Renderer::<App>::with_root_and_props(div_root.into(), yew::props![Props {
        on_submit: move |result| {
            defer.borrow_mut().complete(result);
        }
    }]).render();
    assert!(deferred_future.await.is_ok());
}
