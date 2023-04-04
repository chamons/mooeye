use std::{ffi::OsStr, path::Path, time::Duration};

use ggez::{
    graphics::{self, DrawParam, Drawable, Image, Rect},
    Context, GameError,
};

use crate::{ui_element::Size, UiContent, UiElement};
use std::hash::Hash;


/// A Sprite is an advanced version of an image element, displaying an animated picture that can have multiple states (e.g. a walking, attacking, etc. version of a player character)
/// The sprite is initalized using an image file that contains multiple rows of images (each row representing a variant), where each row contains the same number of animation frames for each variant.
/// Drawing the sprite repeatedly draws every frame of the selected variant in order and then repeats from the beginning.
pub struct Sprite {
    frame_time: Duration,
    w: u32,
    h: u32,
    spritesheet: Image,

    current_frame_time: Duration,
    current_frame: u32,
    current_variant: u32,
}

impl Sprite {

    /// Create a new sprite using the passed [ggez::graphics::Image] and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image within the sheet have to be passed manually.
    /// Will never fail, as the image is already loaded by ggez.
    pub fn new(spritesheet: Image, w: u32, h: u32, frame_time: Duration) -> Self {
        Self {
            frame_time,
            w,
            h,
            spritesheet,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        }
    }

    
    /// Create a new sprite using from the file found at the passed path and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image within the sheet have to be passed manually.
    /// May fail if the image cannot be loaded, because f.e. the path is wrong. Passing 'wrong' size values will yield unexpected behaviour but not panic.
    pub fn from_path(
        path: impl AsRef<Path>,
        ctx: &Context,
        w: u32,
        h: u32,
        frame_time: Duration,
    ) -> Result<Self, GameError> {
        Ok(Self {
            frame_time,
            w,
            h,
            spritesheet: Image::from_path(ctx, path)?,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        })
    }

    /// Create a new sprite using from the file found at the passed path and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image are read from the file name.
    /// The file name needs to be formatted as ```name_possibly_containing_underscores_width_height.extension```.
    /// May fail if the image cannot be loaded (e.g. if the path is wrong) or the file name is not formatted correctly.
    pub fn from_path_fmt(
        path: impl AsRef<Path>,
        ctx: &Context,
        frame_time: Duration,
    ) -> Result<Self, GameError> {
        let pathstring = path
            .as_ref()
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_str().ok_or(GameError::CustomError("Path formatted incorrectly.".to_owned()))?;

        let width_height = pathstring
            .split('.')
            .next()
            .unwrap_or("")
            .split("_")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .take(2)
            .map(|s| *s)
            .rev()
            .collect::<Vec<&str>>();

        let w = *width_height.get(0).ok_or(GameError::CustomError("Filename formatted incorretly - not ending in _width_height.extension. (first element missign)".to_owned()))?;
        let h = *width_height.get(1).ok_or(GameError::CustomError("Filename formatted incorretly - not ending in _width_height.extension. (second element missing)".to_owned()))?;
        let w = w.parse::<u32>().map_err(|_| GameError::CustomError("Filename formatted correctly, but width numbers could not be parsed.".to_owned()))?;
        let h = h.parse::<u32>().map_err(|_| GameError::CustomError("Filename formatted correctly, but height numbers could not be parsed.".to_owned()))?;


        Ok(Self {
            frame_time,
            w,
            h,
            spritesheet: Image::from_path(ctx, path)?,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        })
    }


    /// Sets the variant this sprite currently displays. Numbers that are too large to represent a valid variant will wrap around.
    pub fn set_variant(&mut self, variant: u32) {
        self.current_variant = variant % self.spritesheet.height() / self.h;
    }

    /// Draws this sprite as given by the paramters, advancing the displayed frame as needed.
    pub fn draw_sprite(
        &mut self,
        ctx: &Context,
        canvas: &mut graphics::Canvas,
        param: impl Into<graphics::DrawParam>,
    ) {
        self.current_frame_time += ctx.time.delta();
        while self.current_frame_time >= self.frame_time {
            self.current_frame_time -= self.frame_time;
            self.current_frame = (self.current_frame + 1) % (self.spritesheet.width() / self.w);
        }

        self.draw(canvas, param);
    }
}

impl Drawable for Sprite {
    fn draw(&self, canvas: &mut graphics::Canvas, param: impl Into<graphics::DrawParam>) {
        self.spritesheet.draw(
            canvas,
            (param.into() as DrawParam).src(Rect::new(
                (self.w * self.current_frame) as f32 / self.spritesheet.width() as f32,
                (self.h * self.current_variant) as f32 / self.spritesheet.height() as f32,
                self.w as f32 / self.spritesheet.width() as f32,
                self.h as f32 / self.spritesheet.height() as f32,
            )),
        );
    }

    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        Some(Rect::new(0., 0., self.w as f32, self.h as f32))
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for Sprite {
    fn to_element(self, id: u32) -> UiElement<T>
    where
        Self: Sized + 'static,
    {
        let (w, h) = (self.w, self.h);
        let mut element = UiElement::new(id, self);
        element.layout.x_size = Size::FILL(w as f32, f32::INFINITY);
        element.layout.y_size = Size::FILL(h as f32, f32::INFINITY);
        element.layout.preserve_ratio = true;

        element
    }

    fn draw_content(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        content_bounds: graphics::Rect,
        param: DrawParam,
    ) {
        self.draw_sprite(
            ctx,
            canvas,
            param.dest_rect(Rect::new(
                content_bounds.x,
                content_bounds.y,
                content_bounds.w / self.w as f32,
                content_bounds.h / self.h as f32,
            )),
        );
    }
}
