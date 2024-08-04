// NOTE: Leptos limitation. view! { <> .. </> } emits unused_braces warning.
#![allow(unused_braces)]

use crate::image::{ImageDiffError, ImageDiffInfo};
use crate::web::expose_to_window;
use leptos::web_sys::js_sys;
use leptos::web_sys::wasm_bindgen::JsValue;
use leptos::*;
use std::rc::Rc;

pub type ImageDiffSumaryInfo = Option<Result<Rc<ImageDiffInfo>, ImageDiffError>>;

#[component]
pub fn ImageDiffSummary(info: impl Fn() -> ImageDiffSumaryInfo + 'static) -> impl IntoView {
    let f = move || {
        let info = info()?;
        Some(match info {
            Ok(info) => {
                let ImageDiffInfo {
                    diff_pixel_count,
                    diff_highlight,
                    ..
                } = info.as_ref();
                let mut writer = std::io::Cursor::new(Vec::new());
                diff_highlight
                    .write_to(&mut writer, image::ImageFormat::Png)
                    .unwrap();
                let v = writer.into_inner();
                let u8_array = js_sys::Uint8Array::new(&JsValue::from(v.len()));
                for (i, &item) in v.iter().enumerate() {
                    u8_array.set_index(i as u32, item);
                }
                expose_to_window(&u8_array, "u8_array").unwrap();
                let mut options = web_sys::BlobPropertyBag::new();
                options.type_("image/png");
                let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &js_sys::Array::of1(&u8_array),
                    &options,
                )
                .unwrap();
                expose_to_window(&blob, "blob").unwrap();
                // TODO(memory leak): data_url should be freed.
                let data_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                view! {
                    <>
                        <ion-card>
                            <ion-card-header>
                                <ion-card-title>{"Image Diff"}</ion-card-title>
                            </ion-card-header>
                            <ion-card-content>
                                <p>{format!("{} different pixels", diff_pixel_count)}</p>

                                {if *diff_pixel_count > 0 {
                                    Some(view! { <img src=data_url style:width="100%"/> })
                                } else {
                                    None
                                }}

                            </ion-card-content>
                        </ion-card>
                    </>
                }
            }
            Err(err) => {
                view! {
                    <>
                        <ImageDiffSummaryError err=move || err.clone()/>
                    </>
                }
            }
        })
    };
    view! { <div style:width="320px">{f}</div> }
}

#[component]
fn ImageDiffSummaryError(err: impl Fn() -> ImageDiffError) -> impl IntoView {
    let message = match err() {
        ImageDiffError::DimensionMismatch => "Image dimensions do not match",
    };
    view! {
        <ion-card>
            <ion-card-content>{message}</ion-card-content>
        </ion-card>
    }
}
