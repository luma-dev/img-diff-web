use image::guess_format;
use leptos::*;

#[derive(Debug)]
pub struct ImageLoadInfo {
    pub image: image::DynamicImage,
    pub detected_format: image::ImageFormat,
    pub file: web_sys::File,
    pub preffered_format: Option<image::ImageFormat>,
}
#[derive(Debug, Clone)]
pub enum ImageLoadError {
    FormatError,
    LoadError,
}

pub fn load_image(
    preffered_format: Option<image::ImageFormat>,
    file: web_sys::File,
    data: Vec<u8>,
) -> Result<ImageLoadInfo, ImageLoadError> {
    let detected_format = preffered_format.map_or_else(
        || guess_format(&data).map_err(|_| ImageLoadError::FormatError),
        Ok,
    )?;
    let image = image::load_from_memory_with_format(&data, detected_format)
        .map_err(|_| ImageLoadError::LoadError)?;
    Ok(ImageLoadInfo {
        image,
        preffered_format,
        detected_format,
        file,
    })
}
