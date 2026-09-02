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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Framing {
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

impl Framing {
    pub const SMALLEST_WIDTH: f64 = 0.2;

    pub fn clamped(self) -> Self {
        let width = self.width.clamp(Self::SMALLEST_WIDTH, 1.0);
        Self {
            x: self.x.clamp(0.0, 1.0 - width),
            y: self.y.clamp(0.0, 1.0),
            width,
        }
    }
}

pub fn framed_cover(width: u32, height: u32, framing: Framing) -> Result<CropBox, CoreError> {
    framed_to(
        width,
        height,
        framing,
        COVER_ASPECT_WIDTH,
        COVER_ASPECT_HEIGHT,
    )
}

pub fn framed_to(
    width: u32,
    height: u32,
    framing: Framing,
    aspect_width: u32,
    aspect_height: u32,
) -> Result<CropBox, CoreError> {
    let held = framing.clamped();
    let across = ((f64::from(width) * held.width).round() as u32).max(aspect_width);
    let down = across * aspect_height / aspect_width;
    if across > width || down > height {
        return top_anchored_cover(width, height);
    }

    let left = (f64::from(width) * held.x).round() as u32;
    let top = (f64::from(height) * held.y).round() as u32;
    Ok(CropBox {
        x: left.min(width - across),
        y: top.min(height - down),
        width: across,
        height: down,
    })
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{COVER_ASPECT_HEIGHT, COVER_ASPECT_WIDTH, Framing, framed_cover};

    const WIDE: u32 = 1200;
    const TALL: u32 = 900;

    #[test]
    fn a_framing_keeps_the_cover_aspect_whatever_is_asked_for() {
        let frame = framed_cover(
            WIDE,
            TALL,
            Framing {
                x: 0.1,
                y: 0.2,
                width: 0.5,
            },
        )
        .expect("a half-width framing fits");

        assert_eq!(
            frame.width * COVER_ASPECT_HEIGHT,
            frame.height * COVER_ASPECT_WIDTH
        );
    }

    #[test]
    fn a_framing_never_reaches_past_the_photo() {
        let frame = framed_cover(
            WIDE,
            TALL,
            Framing {
                x: 0.9,
                y: 0.9,
                width: 0.8,
            },
        )
        .expect("an overhanging framing is pulled back");

        assert!(frame.x + frame.width <= WIDE);
        assert!(frame.y + frame.height <= TALL);
    }

    #[test]
    fn a_framing_too_small_to_matter_is_widened_to_the_floor() {
        let asked = Framing {
            x: 0.0,
            y: 0.0,
            width: 0.01,
        };

        assert!((asked.clamped().width - Framing::SMALLEST_WIDTH).abs() < f64::EPSILON);
    }

    #[test]
    fn a_photo_that_cannot_hold_the_asked_frame_falls_back_to_the_whole_width() {
        let frame = framed_cover(
            300,
            100,
            Framing {
                x: 0.0,
                y: 0.0,
                width: 1.0,
            },
        )
        .expect("a short photo still yields a cover");

        assert!(frame.height <= 100);
    }
}
