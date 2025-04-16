use std::io::Cursor;

use anyhow::Ok;
use image::{ DynamicImage, ImageFormat, ImageReader};

pub async fn edit_rotate(para : &str, path : &str,) -> Result<Vec<u8>, anyhow::Error > {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut rotated_image;

    if para == "90"{
        rotated_image = img.rotate90();
    } else if para == "180" {
        rotated_image = img.rotate180();        
    } else if para == "270" {
        rotated_image = img.rotate270();
    } else {
        return Err(anyhow::anyhow!("Invalid parameter"));
    }

    let mut img_buf = Cursor::new(Vec::new());
    rotated_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
} 

pub async fn edit_blur(para : &str, path : &str,)-> Result<Vec<u8>, anyhow::Error>{

    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut blured_image = img.blur(para.parse::<f32>().unwrap());

    let mut img_buf = Cursor::new(Vec::new());
    blured_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())

}