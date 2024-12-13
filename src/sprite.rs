use image::{DynamicImage, GenericImageView};

#[derive(Debug, thiserror::Error)]
pub enum DownloadImageError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}

pub async fn download_from_url(url: &str) -> Result<DynamicImage, DownloadImageError> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let image = image::load_from_memory(&bytes)?;
    Ok(image)
}

pub fn crop_to_content(image: &DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let get_pixel_alpha = |x, y| {
        let [_, _, _, a] = image.get_pixel(x, y).0;
        a
    };

    let mut row_iter = (0..height).map(|y| (0..width).any(|x| get_pixel_alpha(x, y) > 0));
    let top_margin = row_iter.position(|row| row).unwrap() as u32;
    let bottom_margin = row_iter.rev().position(|row| row).unwrap() as u32;

    let mut column_iter = (0..width).map(|x| (0..height).any(|y| get_pixel_alpha(x, y) > 0));
    let left_margin = column_iter.position(|column| column).unwrap() as u32;
    let right_margin = column_iter.rev().position(|column| column).unwrap() as u32;

    let new_width = width - (left_margin + right_margin);
    let new_height = height - (top_margin + bottom_margin);

    image.crop_imm(left_margin, top_margin, new_width, new_height)
}
