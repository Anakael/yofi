use std::f32::consts;

use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};

use super::{Drawable, Space};
use crate::font::{Font, FontBackend};
use crate::style::{Margin, Padding};

pub struct Params {
    pub font: Font,
    pub font_size: u16,
    pub bg_color: SolidSource,
    pub font_color: SolidSource,
    pub margin: Margin,
    pub padding: Padding,
}

pub struct InputText<'a> {
    text: &'a str,
    params: Params,
}

impl<'a> InputText<'a> {
    pub fn new(text: &'a str, params: Params) -> Self {
        Self { text, params }
    }
}

impl<'a> Drawable for InputText<'a> {
    fn draw(self, mut dt: &mut DrawTarget, scale: u16, space: Space, point: Point) -> Space {
        let mut pb = PathBuilder::new();

        let font_size = f32::from(self.params.font_size * scale);

        let padding = self.params.padding * f32::from(scale);
        let margin = self.params.margin * f32::from(scale);

        let border_diameter = padding.top + font_size + padding.bottom;
        let border_radius = border_diameter / 2.0;

        let left_x_center = point.x + margin.left + border_radius;
        let y_center = point.y + margin.top + border_radius;

        pb.arc(
            left_x_center,
            y_center,
            border_radius,
            consts::FRAC_PI_2,
            consts::PI,
        );
        let right_x_center = (point.x + space.width - border_radius - margin.right)
            .max(left_x_center - border_radius);
        pb.arc(
            right_x_center,
            y_center,
            border_radius,
            3.0 * consts::FRAC_PI_2,
            consts::PI,
        );
        let path = pb.finish();

        dt.fill(
            &path,
            &Source::Solid(self.params.bg_color),
            &DrawOptions::new(),
        );

        let pos = Point::new(left_x_center + padding.left, point.y + margin.top);
        self.params.font.draw(
            &mut dt,
            self.text,
            font_size,
            pos,
            self.params.font_color,
            &DrawOptions::new(),
        );

        // TODO: use padding.right for text wrapping/clipping

        Space {
            width: space.width,
            height: point.y + margin.top + border_diameter + margin.bottom,
        }
    }
}
