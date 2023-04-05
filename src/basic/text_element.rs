use ggez::{
    graphics::{Canvas, Drawable, Rect},
    Context,
};
use std::hash::Hash;


use crate::{UiElement, UiContent, ui_element::Size};

impl<T: Copy + Eq + Hash> UiContent<T> for ggez::graphics::Text {
    fn to_element_measured(self, id: u32, ctx: &Context) -> UiElement<T> where Self:Sized + 'static {
        let size = self.dimensions(&ctx.gfx).unwrap_or(Rect { x: 0., y: 0., w: 0., h: 0. });

        let mut element = UiElement::new(id, self);
        element.layout.x_size = Size::FILL(size.w, f32::INFINITY);
        element.layout.y_size = Size::FILL(size.h, f32::INFINITY);
        element.layout.preserve_ratio = true;

        element
    }


    fn draw_content(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        if let Some(dim) = self.dimensions(ctx) {
            canvas.draw(
                self,
                param.param.dest_rect(Rect::new(
                    param.target.x,
                    param.target.y,
                    param.target.w / dim.w,
                    param.target.h / dim.h,
                )),
            );
        }
    }
}
