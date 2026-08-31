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

/// The widest 3:2 box that fits inside a `width` x `height` photo, centred
/// horizontally and anchored to the **top** edge.
///
/// Top anchoring is the load-bearing half: it is what makes a diary cover show
/// the top of the photo instead of its middle. The box is exactly 3:2 by
/// construction, so a later resize never has to correct the ratio.
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

/// The size the cropped box is written at. Large photos come down to
/// [`COVER_MAX_WIDTH`] x [`COVER_MAX_HEIGHT`]; small ones are never blown up.
pub const fn cover_size(frame: CropBox) -> (u32, u32) {
    if frame.width > COVER_MAX_WIDTH {
        (COVER_MAX_WIDTH, COVER_MAX_HEIGHT)
    } else {
        (frame.width, frame.height)
    }
}
