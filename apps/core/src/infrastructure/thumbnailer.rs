use std::fmt::Display;
use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageReader};

use crate::domain::crop::{
    CropBox, Framing, cover_size, framed_cover, framed_to, top_anchored_cover,
};
use crate::domain::error::{ERR_PHOTO_UNDECODABLE, ERR_PHOTO_UNENCODABLE};
use crate::domain::{CoreError, ThumbnailMaker};

pub const COVER_QUALITY: u8 = 82;

#[derive(Debug, Default, Clone, Copy)]
pub struct ImageThumbnailer;

impl ThumbnailMaker for ImageThumbnailer {
    fn cover(&self, bytes: &[u8]) -> Result<Vec<u8>, CoreError> {
        let source = decode(bytes)?;
        let frame = top_anchored_cover(source.width(), source.height())?;
        encode(&source, frame)
    }

    fn framed_cover(&self, bytes: &[u8], framing: Framing) -> Result<Vec<u8>, CoreError> {
        let source = decode(bytes)?;
        let frame = framed_cover(source.width(), source.height(), framing)?;
        encode(&source, frame)
    }

    fn framed_square(&self, bytes: &[u8], framing: Framing) -> Result<Vec<u8>, CoreError> {
        let source = decode(bytes)?;
        let frame = framed_to(source.width(), source.height(), framing, 1, 1)?;
        encode(&source, frame)
    }
}

fn decode(bytes: &[u8]) -> Result<DynamicImage, CoreError> {
    let mut decoder = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(undecodable)?
        .into_decoder()
        .map_err(undecodable)?;
    let orientation = decoder.orientation().map_err(undecodable)?;
    let mut source = DynamicImage::from_decoder(decoder).map_err(undecodable)?;
    source.apply_orientation(orientation);
    Ok(source)
}

fn undecodable(cause: impl Display) -> CoreError {
    CoreError::Photo(format!("{ERR_PHOTO_UNDECODABLE}: {cause}"))
}

fn encode(source: &DynamicImage, frame: CropBox) -> Result<Vec<u8>, CoreError> {
    let cropped = source.crop_imm(frame.x, frame.y, frame.width, frame.height);
    let (width, height) = cover_size(frame);
    let scaled = if width == frame.width {
        cropped
    } else {
        cropped.resize_exact(width, height, FilterType::Lanczos3)
    };

    let mut out = Cursor::new(Vec::new());
    JpegEncoder::new_with_quality(&mut out, COVER_QUALITY)
        .encode_image(&scaled.to_rgb8())
        .map_err(|cause| CoreError::Photo(format!("{ERR_PHOTO_UNENCODABLE}: {cause}")))?;
    Ok(out.into_inner())
}
