use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::js_sys;
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

static COUNTER_GLOBAL_NAME: &str = "_leptos_image_dropzone_counter";

#[component]
pub fn ImageDropzone(on_image_selected: impl Fn(web_sys::File) + 'static) -> impl IntoView {
    let window = web_sys::window().unwrap();

    let c = js_sys::Reflect::get(&window, &JsValue::from(COUNTER_GLOBAL_NAME)).unwrap();
    let is_c_int = js_sys::Number::is_safe_integer(&JsValue::from(&c));
    let count = if is_c_int {
        c.as_f64().unwrap() as i32
    } else {
        0
    } + 1;
    js_sys::Reflect::set(
        &window,
        &JsValue::from(COUNTER_GLOBAL_NAME),
        &JsValue::from(count),
    )
    .unwrap();

    let name = format!("_leptos_image_dropzone_props_fn_{}", count);
    let on_image_selected_closure = Closure::<dyn Fn(web_sys::File)>::wrap(Box::new(move |file| {
        on_image_selected(file);
    }));
    let props_fn_closure = Closure::<dyn Fn() -> JsValue>::wrap(Box::new(move || {
        let v = JsCast::unchecked_into(js_sys::Object::new());
        js_sys::Reflect::set(
            &v,
            &JsValue::from("onImageSelected"),
            &JsValue::from(on_image_selected_closure.as_ref()),
        )
        .unwrap();
        v
    }));
    let f = props_fn_closure.into_js_value();
    js_sys::Reflect::set(&window, &JsValue::from(&name), &f).unwrap();

    view! { <x-image-dropzone props-fn=name></x-image-dropzone> }
}
