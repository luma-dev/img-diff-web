use crate::image::image_format_to_string;
use crate::image::{ImageLoadError, ImageLoadInfo};
use leptos::*;
use std::rc::Rc;

pub type ImageSumaryInfo = Option<Result<Rc<ImageLoadInfo>, ImageLoadError>>;

#[component]
pub fn ImageSummary(
    info: impl Fn() -> ImageSumaryInfo + 'static,
    image_name: String,
) -> impl IntoView {
    let f = move || {
        let info = info()?;
        Some(match info {
            Ok(info) => {
                let ImageLoadInfo {
                    file,
                    image,
                    detected_format,
                    ..
                } = info.as_ref();
                let dimensions = format!("{} x {}", image.width(), image.height());
                let pixels = image.width() * image.height();
                let format = image_format_to_string(detected_format);
                let size = file.size();
                view! {
                    <>
                        <ion-card>
                            <ion-card-header>
                                <ion-card-title>{image_name.clone()}</ion-card-title>
                            </ion-card-header>
                            <ion-card-content>
                                <p>
                                    <span>Dimensions: {dimensions}</span>
                                    <span>{format!(" ({pixels} pixels)")}</span>
                                </p>
                                <p>Format: {format}</p>
                                <p>Size: {size} bytes</p>
                            </ion-card-content>
                        </ion-card>
                    </>
                }
            }
            Err(err) => {
                view! {
                    <>
                        <ImageSummaryError err=move || err.clone()/>
                    </>
                }
            }
        })
    };
    view! { <div style:width="320px">{f}</div> }
}

#[component]
fn ImageSummaryError(err: impl Fn() -> ImageLoadError) -> impl IntoView {
    let message = match err() {
        ImageLoadError::FormatError => "Failed to determine image format",
        ImageLoadError::LoadError => "Failed to load image. HINT: Try a different format",
    };
    view! {
        <ion-card>
            <ion-card-content>{message}</ion-card-content>
        </ion-card>
    }
}
