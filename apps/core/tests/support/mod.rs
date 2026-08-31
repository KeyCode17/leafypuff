#![allow(dead_code)]

use std::io::Cursor;

use exif::experimental::Writer;
use exif::{Field, In, Tag, Value};
use image::codecs::jpeg::JpegEncoder;
use image::{ImageDecoder, Rgb, RgbImage};

pub const TOP_BAND: Rgb<u8> = Rgb([214, 38, 38]);
pub const BOTTOM_BAND: Rgb<u8> = Rgb([38, 62, 214]);
pub const TOLERANCE: i32 = 24;

pub fn solid_jpeg(width: u32, height: u32) -> Vec<u8> {
    encode(RgbImage::from_pixel(width, height, TOP_BAND))
}

pub fn banded_jpeg(width: u32, height: u32, top_rows: u32) -> Vec<u8> {
    let mut canvas = RgbImage::new(width, height);
    for (_, y, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = if y < top_rows { TOP_BAND } else { BOTTOM_BAND };
    }
    encode(canvas)
}

pub fn jpeg_taken_on(stamp: &[u8]) -> Vec<u8> {
    splice_app1(&solid_jpeg(48, 32), &exif_blob(stamp))
}

pub fn jpeg_with_broken_exif(stamp: &[u8]) -> Vec<u8> {
    let mut blob = exif_blob(stamp);
    for byte in blob.iter_mut().skip(4) {
        *byte ^= 0xA5;
    }
    splice_app1(&solid_jpeg(48, 32), &blob)
}

pub fn truncated_jpeg() -> Vec<u8> {
    let mut bytes = solid_jpeg(48, 32);
    bytes.truncate(24);
    bytes
}

pub fn not_an_image() -> Vec<u8> {
    b"a note, not a photograph, and not even close to one".to_vec()
}

pub fn decode(bytes: &[u8]) -> RgbImage {
    image::load_from_memory(bytes)
        .expect("the fixture decodes")
        .to_rgb8()
}

pub fn close(left: Rgb<u8>, right: Rgb<u8>) -> bool {
    left.0
        .iter()
        .zip(right.0.iter())
        .all(|(one, other)| (i32::from(*one) - i32::from(*other)).abs() <= TOLERANCE)
}

fn encode(canvas: RgbImage) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut out, 100);
    encoder
        .encode_image(&canvas)
        .expect("the fixture jpeg encodes");
    out.into_inner()
}

fn exif_blob(stamp: &[u8]) -> Vec<u8> {
    let field = Field {
        tag: Tag::DateTimeOriginal,
        ifd_num: In::PRIMARY,
        value: Value::Ascii(vec![stamp.to_vec()]),
    };
    let mut writer = Writer::new();
    writer.push_field(&field);
    let mut blob = Cursor::new(Vec::new());
    writer.write(&mut blob, false).expect("an exif blob writes");
    blob.into_inner()
}

fn splice_app1(jpeg: &[u8], tiff: &[u8]) -> Vec<u8> {
    let segment_len = u16::try_from(tiff.len() + 8).expect("an app1 segment that fits");
    let mut out = Vec::with_capacity(jpeg.len() + tiff.len() + 10);
    out.extend_from_slice(&jpeg[..2]);
    out.extend_from_slice(&[0xFF, 0xE1]);
    out.extend_from_slice(&segment_len.to_be_bytes());
    out.extend_from_slice(b"Exif\0\0");
    out.extend_from_slice(tiff);
    out.extend_from_slice(&jpeg[2..]);
    out
}

pub fn dimensions(bytes: &[u8]) -> (u32, u32) {
    let decoder = image::codecs::jpeg::JpegDecoder::new(Cursor::new(bytes))
        .expect("the derivative is a jpeg");
    decoder.dimensions()
}
