use ev::Event;
use ev::InputEvent;
use ev::MouseEvent;
use image;
use image::GenericImageView;
use img_diff_web::SetupIonic;
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::js_sys;
use leptos::web_sys::HtmlInputElement;
use leptos::*;
use web_sys::wasm_bindgen::JsCast;
//use web_sys::HtmlInputElement;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <SetupIonic/>
            <App/>
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let reader = Rc::new(RefCell::new(web_sys::FileReader::new().unwrap()));
    {
        let closure = {
            let reader = reader.clone();
            Closure::wrap(Box::new(move |_ev: Event| {
                let reader = reader.borrow();
                logging::log!("reader: {:?}", reader.result());
                let reader_result = reader.result().unwrap();
                let reader_result = reader_result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                let reader_result = js_sys::Uint8Array::new(&reader_result);
                let reader_result = reader_result.to_vec();
                let a = image::load_from_memory(&reader_result).unwrap();
                let d = a.dimensions();
                logging::log!("a: {:?}", d);
            }) as Box<dyn FnMut(_)>)
        };
        let closure = closure
            .into_js_value()
            .dyn_into::<js_sys::Function>()
            .unwrap();
        {
            let reader = reader.borrow();
            reader.set_onload(Some(&closure));
        }
    }
    let on_input = {
        let reader = reader.clone();
        move |ev: Event| {
            let reader = reader.borrow();
            let target = event_target::<HtmlInputElement>(&ev);
            logging::log!("target: {:?}", target);
            logging::log!("files: {:?}", target.files());
            let file = match target.files().and_then(|files| files.get(0)) {
                Some(file) => file,
                None => return,
            };
            file.name();
            let v = reader.read_as_array_buffer(&file);
            logging::log!("v: {:?}", reader.result());
        }
    };

    view! {
        <div>
            <input type="file" id="file1" accept="image/*" on:input=on_input/>
        </div>
    }
}
