use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;

use crate::domain::crop::{cover_size, top_anchored_cover};
use crate::domain::error::{ERR_PHOTO_UNDECODABLE, ERR_PHOTO_UNENCODABLE};
use crate::domain::{CoreError, ThumbnailMaker};

pub const COVER_QUALITY: u8 = 82;

#[derive(Debug, Default, Clone, Copy)]
pub struct ImageThumbnailer;

impl ThumbnailMaker for ImageThumbnailer {
    fn cover(&self, bytes: &[u8]) -> Result<Vec<u8>, CoreError> {
        let source = image::load_from_memory(bytes)
            .map_err(|cause| CoreError::Photo(format!("{ERR_PHOTO_UNDECODABLE}: {cause}")))?;

        let frame = top_anchored_cover(source.width(), source.height())?;
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
}
