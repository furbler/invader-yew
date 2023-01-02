use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

// 対応するキーが押されている時真
#[derive(Debug)]
pub struct KeyDown {
    pub left: bool,  // プレイヤーを左へ移動させる
    pub right: bool, // プレイヤーを右へ移動させる
    pub shot: bool,  // プレイヤーが弾を撃つ
    pub pause: bool, // ポーズする
}

pub fn input_setup(input_key: &Rc<RefCell<KeyDown>>) {
    // キー入力処理
    let key_down = Rc::clone(&input_key);
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    body.append_child(&canvas).unwrap();
    // キー押し下げ

    let closure_key_down = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        function_key_down(e, &mut key_down.borrow_mut());
    }) as Box<dyn FnMut(_)>);

    body.add_event_listener_with_callback("keydown", closure_key_down.as_ref().unchecked_ref())
        .unwrap();
    closure_key_down.forget();

    // キー押し上げ
    let key_up = Rc::clone(&input_key);
    let closure_key_up = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        function_key_up(e, &mut key_up.borrow_mut());
    }) as Box<dyn FnMut(_)>);
    body.add_event_listener_with_callback("keyup", closure_key_up.as_ref().unchecked_ref())
        .unwrap();
    closure_key_up.forget();
}
// キー押し下げ時に呼び出し
fn function_key_down(event: web_sys::KeyboardEvent, input_data: &mut KeyDown) {
    match &*event.key() {
        "ArrowLeft" | "a" => {
            input_data.left = true;
        }
        "ArrowRight" | "d" => {
            input_data.right = true;
        }
        "Space" | "Enter" | "z" => {
            input_data.shot = true;
        }
        "Escape" => {
            input_data.pause = true;
        }
        _ => (),
    };
}

// キー押し上げ時に呼び出し
fn function_key_up(event: web_sys::KeyboardEvent, input_data: &mut KeyDown) {
    match &*event.key() {
        "ArrowLeft" | "a" => {
            input_data.left = false;
        }
        "ArrowRight" | "d" => {
            input_data.right = false;
        }
        "Space" | "Enter" | "z" => {
            input_data.shot = false;
        }
        "Escape" => {
            input_data.pause = false;
        }
        _ => (),
    };
}
