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

    let rows: Vec<bool> = (0..height)
        .map(|y| {
            for x in 0..width {
                if get_pixel_alpha(x, y) > 0 {
                    return true;
                }
            }
            false
        })
        .collect();

    let columns: Vec<bool> = (0..width)
        .map(|x| {
            for y in 0..height {
                if get_pixel_alpha(x, y) > 0 {
                    return true;
                }
            }
            false
        })
        .collect();

    let top_margin = rows.iter().position(|row| *row).unwrap() as u32;
    let bottom_margin = rows.iter().rev().position(|row| *row).unwrap() as u32;
    let left_margin = columns.iter().position(|column| *column).unwrap() as u32;
    let right_margin = columns.iter().rev().position(|column| *column).unwrap() as u32;

    let new_width = width - (left_margin + right_margin);
    let new_height = height - (top_margin + bottom_margin);

    let mut new_image = DynamicImage::new_rgba8(new_width, new_height);
    let image_buffer = new_image.as_mut_rgba8().unwrap();

    for x in 0..new_width {
        for y in 0..new_height {
            let image_x = x + left_margin;
            let image_y = y + top_margin;
            let pixel = image.get_pixel(image_x, image_y);

            image_buffer.put_pixel(x, y, pixel);
        }
    }

    new_image
}
