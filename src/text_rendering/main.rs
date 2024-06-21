use std::any::Any;

use freetype::{self, charmap::CharMap, face::LoadFlag, glyph, Face, GlyphSlot, Library};
use image::{GrayImage, ImageBuffer, Luma};
use opencv::{
    core::CV_8UC1,
    highgui::{destroy_all_windows, imshow, wait_key},
    prelude::*,
};

fn text2image(text: &str, font: Face, font_size: u32) -> GrayImage {
    font.set_pixel_sizes(0, font_size).unwrap();

    let mut full_width = 0;
    let mut y_lower = 10000;
    let mut y_higher = 0;
    let mut top_down_origin = 0;

    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_metrics = font.glyph().metrics();

        full_width += glyph_metrics.horiAdvance >> 6;
        y_higher = y_higher.max(glyph_metrics.horiBearingY);
        y_lower = y_lower.min(glyph_metrics.height - glyph_metrics.horiBearingY);
    }
    y_higher = y_higher >> 6;
    y_lower = y_lower >> 6;
    let full_height = (y_higher) - (y_lower);
    let mut image = GrayImage::new(full_width as _, full_height as u32);

    let mut advances = 0u32;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_bitmap = font.glyph().bitmap();
        let glyph_metrics = font.glyph().metrics();
        let rows = glyph_bitmap.rows() as u32;
        let cols = glyph_bitmap.width() as u32;

        for r in 0..rows {
            for c in 0..cols {
                image.put_pixel(
                    advances + c,
                    y_higher as u32 - (glyph_metrics.horiBearingY >> 6) as u32 + r,
                    Luma([glyph_bitmap.buffer()[(r * glyph_bitmap.width() as u32 + c) as usize]]),
                );
            }
        }
        advances += (glyph_metrics.horiAdvance >> 6) as u32;
    }

    println!("{:?}", (full_width, full_height));
    return image;
}

fn text2image2(text: &str, font: &Face, font_size: u32) -> GrayImage {
    font.set_pixel_sizes(0, font_size);

    let mut x_right = 0;

    let mut y0 = 0i32;
    let mut y1 = 0i32;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER);
        let char_metrics = font.glyph().metrics();
        x_right += char_metrics.horiAdvance >> 6;
        y0 = y0.max((char_metrics.horiBearingY >> 6) as i32);
        y1 = y1.max((char_metrics.height >> 6) as i32 - (char_metrics.horiBearingY >> 6) as i32);
        // let current_down = 2 * (char_metrics.height >> 6) - (char_metrics.horiBearingY >> 6);
        // if current_down > y_down {
        //     y_down = current_down;
        //     origin=y_down -((char_metrics.height>>6) - (char_metrics.horiBearingY >> 6) );
        // }
        // y_down = y_down.max();
    }
    let y_down = y0 + y1;

    let mut img = GrayImage::new(x_right as _, y_down as _);
    let mut advances = 0u32;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_bitmap = font.glyph().bitmap();
        let glyph_metrics = font.glyph().metrics();
        let rows = glyph_bitmap.rows() as u32;
        let cols = glyph_bitmap.width() as u32;

        for r in 0..rows {
            for c in 0..cols {
                img.put_pixel(
                    advances + c,
                    y0 as u32 - (glyph_metrics.horiBearingY >> 6) as u32 + r,
                    Luma([glyph_bitmap.buffer()[(r * glyph_bitmap.width() as u32 + c) as usize]]),
                );
            }
        }
        advances += (glyph_metrics.horiAdvance >> 6) as u32;
    }
    println!("{:?}", (x_right, y_down));

    img
}

fn main() {
    // Init the library
    let lib = Library::init().unwrap();
    // Load a font face

    let face = lib.new_face("LatinmodernmathRegular-z8EBa.otf", 0).unwrap();
    let img = text2image2("displaystyl", &face, 32);

    let mat = unsafe {
        Mat::new_rows_cols_with_data_unsafe(
            img.rows().len() as _,
            img.width() as _,
            CV_8UC1,
            img.as_ptr() as _,
            img.width() as _,
        )
    }
    .unwrap();

    imshow("winname", &mat);
    wait_key(0);
}
