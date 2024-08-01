mod load_image;
pub use load_image::{load_image, ImageLoadError, ImageLoadInfo};

mod diff_image;
pub use diff_image::{diff_image, ImageDiffError, ImageDiffInfo};

pub fn image_format_to_string(format: &image::ImageFormat) -> &'static str {
    match format {
        image::ImageFormat::Gif => "Gif",
        image::ImageFormat::Ico => "Ico",
        image::ImageFormat::Jpeg => "Jpeg",
        image::ImageFormat::Png => "Png",
        image::ImageFormat::Bmp => "Bmp",
        image::ImageFormat::Tiff => "Tiff",
        image::ImageFormat::Tga => "Tga",
        image::ImageFormat::Pnm => "Pnm",
        image::ImageFormat::Farbfeld => "Farbfeld",
        image::ImageFormat::Avif => "Avif",
        image::ImageFormat::WebP => "WebP",
        image::ImageFormat::OpenExr => "OpenExr",
        image::ImageFormat::Qoi => "Qoi",
        image::ImageFormat::Dds => "Dds",
        image::ImageFormat::Hdr => "Hdr",
        _ => "Unknown",
    }
}

pub fn image_string_to_format(format: &str) -> Option<image::ImageFormat> {
    match format {
        "Gif" => image::ImageFormat::Gif,
        "Ico" => image::ImageFormat::Ico,
        "Jpeg" => image::ImageFormat::Jpeg,
        "Png" => image::ImageFormat::Png,
        "Bmp" => image::ImageFormat::Bmp,
        "Tiff" => image::ImageFormat::Tiff,
        "Tga" => image::ImageFormat::Tga,
        "Pnm" => image::ImageFormat::Pnm,
        "Farbfeld" => image::ImageFormat::Farbfeld,
        "Avif" => image::ImageFormat::Avif,
        "WebP" => image::ImageFormat::WebP,
        "OpenExr" => image::ImageFormat::OpenExr,
        "Qoi" => image::ImageFormat::Qoi,
        "Dds" => image::ImageFormat::Dds,
        "Hdr" => image::ImageFormat::Hdr,
        _ => return None,
    }
    .into()
}
