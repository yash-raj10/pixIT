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

pub async fn edit_grayscale (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut grayscale_image = img.grayscale();

    let mut img_buf = Cursor::new(Vec::new());
    grayscale_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
}

pub async fn edit_brighten (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut brighten_image = img.brighten(para.parse::<i32>().unwrap());

    let mut img_buf = Cursor::new(Vec::new());
    brighten_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
}

pub async fn edit_contrast (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut contrasted_image = img.adjust_contrast(para.parse::<f32>().unwrap());

    let mut img_buf = Cursor::new(Vec::new());
    contrasted_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
}

pub async fn edit_hue_rotation (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut hue_image = img.huerotate(para.parse::<i32>().unwrap());

    let mut img_buf = Cursor::new(Vec::new());
    hue_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
}


// pub async fn edit_resize (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
//     let img: DynamicImage  = ImageReader::open(path)?.decode()?;

//     let mut resized_image = img.resize(para.parse::<i32>().unwrap());

//     let mut img_buf = Cursor::new(Vec::new());
//     resized_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

//     Ok(img_buf.into_inner())
// }


pub async fn edit_flip (para : &str, path : &str) -> Result<Vec<u8>, anyhow::Error> {
    let img: DynamicImage  = ImageReader::open(path)?.decode()?;

    let mut flipped_image: DynamicImage;

    if para == "hor" {
        flipped_image = img.fliph();
    } else if para == "ver" {
        flipped_image = img.flipv()
        
    }else {
        return  Err(anyhow::anyhow!("Invalid Parameter"));
    }

    let mut img_buf = Cursor::new(Vec::new());
    flipped_image.write_to(&mut img_buf, ImageFormat::Jpeg)?;

    Ok(img_buf.into_inner())
}

