use image::GenericImageView;

#[derive(Debug)]
pub struct ImageDiffInfo {
    pub diff_pixel_count: usize,
    pub diff_highlight: image::DynamicImage,
}

#[derive(Debug, Clone)]
pub enum ImageDiffError {
    DimensionMismatch,
}

pub fn diff_image(
    image1: &image::DynamicImage,
    image2: &image::DynamicImage,
) -> Result<ImageDiffInfo, ImageDiffError> {
    if image1.dimensions() != image2.dimensions() {
        return Err(ImageDiffError::DimensionMismatch);
    }
    let (width, height) = image1.dimensions();
    let mut diff_image = image::ImageBuffer::new(width, height);
    let mut diff_pixel_count = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);
            if pixel1 != pixel2 {
                diff_pixel_count += 1;
                diff_image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            } else {
                diff_image.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
    }
    let diff_highlight = image::DynamicImage::ImageRgba8(diff_image);
    Ok(ImageDiffInfo {
        diff_pixel_count,
        diff_highlight,
    })
}
