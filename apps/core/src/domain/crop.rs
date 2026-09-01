use super::error::{CoreError, ERR_PHOTO_TOO_SMALL};

pub const COVER_ASPECT_WIDTH: u32 = 3;
pub const COVER_ASPECT_HEIGHT: u32 = 2;
pub const COVER_MAX_WIDTH: u32 = 1080;
pub const COVER_MAX_HEIGHT: u32 = 720;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CropBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn top_anchored_cover(width: u32, height: u32) -> Result<CropBox, CoreError> {
    let across = width / COVER_ASPECT_WIDTH;
    let down = height / COVER_ASPECT_HEIGHT;
    let scale = across.min(down);
    if scale == 0 {
        return Err(CoreError::Photo(format!(
            "{ERR_PHOTO_TOO_SMALL}: {width}x{height}"
        )));
    }

    let cropped_width = scale * COVER_ASPECT_WIDTH;
    Ok(CropBox {
        x: (width - cropped_width) / 2,
        y: 0,
        width: cropped_width,
        height: scale * COVER_ASPECT_HEIGHT,
    })
}

pub const fn cover_size(frame: CropBox) -> (u32, u32) {
    if frame.width > COVER_MAX_WIDTH {
        (COVER_MAX_WIDTH, COVER_MAX_HEIGHT)
    } else {
        (frame.width, frame.height)
    }
}
