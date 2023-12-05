# wasm-yew-canvas-checkcode

纯前端`yew.rs`图形验证码控件。不同于从后端拉取图形验证码图片，该控件以更“轻量级”的技术手段抑制`UI`用户重复地连续提交表单。

## 工作原理

1. 借助`rand crate`，本地生成随机字符串。
2. 经由`web_sys crate`，操作`Canvas 2D`平面渲染引擎，生成图形验证码图片。在图片中的
   1. 背景色
   2. 背景星型图案
   3. 每个星型图案的角数（3 ~ 8）、位置、旋转、颜色、密度
   4. 每位验证字符的位置、旋转、颜色，间距

   都是被即时演算出来的。

3. 通过被传入控件的【回调函数】`on_check_code_change(CheckCode)`，将被生成的随机字符串验证码返回给父控件。

## `crate`导出项清单

1. `::wasm_yew_canvas_checkcode::CanvasCheckCode`图形验证码控件自身。在渲染函数中，其被记为`<CanvasCheckCode>`。
2. `::wasm_yew_canvas_checkcode::Message`控件的枚举类内部状态集
3. `::wasm_yew_canvas_checkcode::Props`控件的输入参数属性集
4. `::wasm_yew_canvas_checkcode::CheckCode`包装了图形验证码字符串的枚举类
     * `CheckCode::Initialize(String)`代表控件初始化过程生成的图形验证码
     * `CheckCode::Update(String)`代表由
       * `UI`鼠标点击事件或
       * 父控件程序触发

       生成的图形验证码

## 控件输入参数列表

1. `width: f64`
   * 可选参数
   * 单位：像素
   * 图形验证码`canvas`画布的宽度
   * 默认值优先次序
     1. `css`样式表设置的宽度
     2. 缺省值`150`
2. `height: f64`
   * 可选参数
   * 单位：像素
   * 图形验证码`canvas`画布的高度
   * 默认值优先次序
     1. `css`样式表设置的宽度
     2. 缺省值`50`
3. `star_size: f64`
   * 可选参数
   * 单位：像素
   * 背景随机星型图案的大小尺寸。因为星型图案的`BBox`是正方形，所以这里仅只需要设置一个值。
   * 默认值`7`
4. `star_count: u8`
   1. 可选参数
   2. 单位：个
   3. 背景随机星型图案的最多个数。
   4. 默认值`25`。在图形渲染过程中，虽然做了星型图形的`BBox`碰撞测试，但若星型图案太多，还是会出现星型图案的重叠现象。
5. `font_size: f64`
   * 可选参数
   * 单位：像素
   * 验证码单个字符的最大尺寸
   * 默认值`22`。
6. `check_code_len: u8`
   1. 可选参数
   2. 单位：个
   3. 验证码的字符个数
   4. 默认值`5`。字符太多也会出现重叠现象，虽然程序也对单个验证码字符的`BBox`做过碰撞测试了。
7. `on_check_code_change: Callback<CheckCode>`
   1. 必填参数
   2. 类型：事件回调函数。
      1. 形参`CheckCode`是枚举值
         1. `CheckCode::Initialize(String)`代表控件初始化过程生成的图形验证码
         2. `CheckCode::Update(String)`代表由`UI`点击事件或程序触发生成的图形验证码
      2. 没有返回值
   3. 功能：向父控制反馈最新生成的验证码字符串。
8. `reversed_hook: Callback<Scope<CanvasCheckCode>>`
   1. 可选参数
   2. 类型：事件回调函数。
      1. 形参`Scope<CanvasCheckCode>`代表`<CanvasCheckCode>`控件的【作用域】对象。
      2. 没有返回值
   3. 功能：向父控件传递`<CanvasCheckCode>`【作用域】对象（复本），以允许从控件外部程序地触发新图形验证码的生成操作。比如，
      1. 首先，由`Rc<RefCell<Option<Scope<CanvasCheckCode>>>>`字段值，缓存被上传的`<CanvasCheckCode>`【作用域】对象复本。
      2. 再，在`fn view(..) -> bool`生命周期函数内，程序地触发`<CanvasCheckCode>`子控件重新生成图形验证码。

      ```rust
      self.check_code_scope.borrow().as_ref().map(|scope| {
         scope.send_message(CanvasCheckCodeMessage::UpdateCheckCode);
      });
      ```

   4. 缺省值代表什么都不做

总结，最后两个控件输入参数都是回调函数。其功能都是自下而向，从`<CanvasCheckCode>`向父控件传递返回值的

1. `on_check_code_change`返回最新的图形验证码字符串。
2. `reversed_hook`返回程序触发生成新图形验证码的“操作句柄”。

## 控件输出回调函数钩子

`<CanvasCheckCode>`控件以回调函数的方式向父控件回传

1. 最新被生成的图形验证码字符串 —— 用以校对`UI`用户敲入的图形验证码字符串是否正确。
2. 自身作用域对象`Scope<CanvasCheckCode>` —— 用以程序地从父控件触发新图形验证码的生成。

### 从父控件获取最新图形验证码字符串

回调函数签名`on_check_code_change: |check_code: CheckCode| -> () { .. }`

* 回调函数的返回值是`unit type`
* 形参是枚举类`CheckCode`
  * `CheckCode::Initialize(String)`代表控件初始化过程生成的图形验证码
  * `CheckCode::Update(String)`代表由
    * `UI`鼠标点击事件或
    * 父控件程序触发

    生成的图形验证码

#### 例程

```rust
use ::wasm_yew_canvas_checkcode::CheckCode;
//
yew::props![CanvasCheckCodeProps {
   on_check_code_change: |check_code| {
      let check_code = match check_code {
            CheckCode::Initialize(value) => value,
            CheckCode::Update(value) => value,
      };
      console::info!("从父组件收到的校验码", check_code);
   }
}]
```

### 从父控件程序地刷新图形验证码

回调函数签名`reversed_hook: |my_scope: Scope<CanvasCheckCode>| -> () { .. }`

* 回调函数的返回值是`unit type`
* 形参是`Scope<CanvasCheckCode>`

首先，获取`<CanvasCheckCode>`子控件的作用域对象`Scope<CanvasCheckCode>`。

然后，在父控件的结构体字段`child1_scope: Rc<RefCell<Option<Scope<CanvasCheckCode>>>>`内缓存该作用域对象。

```rust
use ::wasm_yew_canvas_checkcode::CheckCode;
//
yew::props![CanvasCheckCodeProps {
   reversed_hook: |my_scope| {
      // 在父控件中，缓存子控件的`scope`对象
      child1_scope.borrow_mut().replace(my_scope);
   }
}]
```

最后，在父控件的`fn update( .. ) -> bool`生命周期函数内，修改子控件`<CanvasCheckCode>`的状态集以触发新的图型验证码被生成。

```rust
fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
   let props = ctx.props();
   match msg {
      Message::SubmitForm => {
         self.child1_scope.borrow().as_ref().map(|child1_scope| { // 从父控件，触发子控件刷新图形验证码
            child1_scope.send_message(CanvasCheckCodeMessage::UpdateCheckCode);
         });
         return false;
      }
      _ => ()
   }
   true
}
```

## 附赠两个例程

此`crate`以【**（有脸）**集成测试】的方式，呈送两个例程

### 仅图形验证码控制简单例程

例程文件：`tests\simple.rs`

启动命令行指令：`wasm-pack test --chrome --test=simple`

演示内容：

1. `<CanvasCheckCode>`控件被做为整个`wasm-webapp`的根组件直接加到`DOM`流中。
2. 点击`<CanvasCheckCode>`控件会刷新图形验证码字符串

![例程1](https://github.com/stuartZhang/deferred-future/assets/13935927/8f5584a2-3cbc-4f27-a66c-23f1d2fea907)

### 登录表单半成品（实战）例程

例程文件：`tests\form.rs`

启动命令行指令：`wasm-pack test --chrome --test=form`

演示内容：

1. `wasm-webapp`的根组件是一个【用户名/密码/图形验证码】的常规网页登录表单。
2. 集成测试执行流被阻塞。并且，仅当`UI`用户录入了正常的图形验证码才能开始表单验证。
3. 点击`<CanvasCheckCode>`控件会刷新图形验证码字符串，同时置空图形验证码的文本输入框。
4. 点击【登录】按钮，会比较从`<CanvasCheckCode>`控件回传给父控件的图形验证码“本尊”与用户录入的图形验证码字符串是否一致。
5. 若两个图形验证码字符串一致，则集成测试成功通过。
6. 否则，集成测试失败

![image](https://github.com/stuartZhang/deferred-future/assets/13935927/0e132d84-a50d-41b1-b322-9b4929cf1d0c)
