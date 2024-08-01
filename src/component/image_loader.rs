use crate::component::ported::ImageDropzone;
use crate::image::{image_format_to_string, image_string_to_format};
use ev::Event;
use image;
use leptos::web_sys::HtmlSelectElement;
use leptos::*;

#[component]
pub fn ImageLoader(
    on_image_selected: impl Fn(web_sys::File) + 'static,
    on_format_selected: impl Fn(Option<image::ImageFormat>) + 'static,
) -> impl IntoView {
    let (image_format, set_image_format) = create_signal("auto:".to_string());
    let options = image::ImageFormat::all()
        .filter(|format| format.reading_enabled())
        .map(|format| {
            let s = image_format_to_string(&format);
            view! { <ion-select-option value=format!("format:{s}")>{s}</ion-select-option> }
        })
        .collect::<Vec<_>>();
    let on_change = {
        move |ev: Event| {
            let target = event_target::<HtmlSelectElement>(&ev);
            let value = target.value();
            set_image_format(value.clone());
            on_format_selected(if value == "auto:" {
                None
            } else {
                let format = value
                    .strip_prefix("format:")
                    .and_then(image_string_to_format);
                format
            });
        }
    };
    view! {
        <div style:width="320px">
            <ImageDropzone on_image_selected=on_image_selected/>
            <ion-select label="Image Format" value=image_format on:ionChange=on_change>
                <ion-select-option value="auto:">(Auto)</ion-select-option>
                {options}
            </ion-select>
        </div>
    }
}
