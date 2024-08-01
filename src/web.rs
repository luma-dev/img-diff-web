use leptos::ev::Event;
use leptos::wasm_bindgen::prelude::{Closure, JsCast};
use leptos::web_sys::js_sys;
use std::cell::RefCell;
use std::rc::Rc;

mod expose_to_window;

pub use expose_to_window::expose_to_window;

pub fn file_to_binary(file: web_sys::File, on_load: impl Fn(Vec<u8>) + 'static) {
    let reader = Rc::new(RefCell::new(web_sys::FileReader::new().unwrap()));
    {
        let closure = {
            let reader = reader.clone();
            Closure::wrap(Box::new(move |_ev: Event| {
                let reader = reader.borrow();
                let reader_result = reader.result().unwrap();
                let reader_result = reader_result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                let reader_result = js_sys::Uint8Array::new(&reader_result);
                let reader_result = reader_result.to_vec();
                on_load(reader_result);
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
    {
        let reader = reader.borrow();
        reader.read_as_array_buffer(&file).unwrap();
    }
}
