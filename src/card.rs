use nalgebra as na;
use util;
use web_sys::HtmlImageElement;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: &'static str,
    pub image: HtmlImageElement,
    pub pos: na::Vector2<f64>,
    pub dim: na::Vector2<f64>,
    pub grabbing: bool,
}

impl Card {
    pub fn is_inside(&self, point: &na::Vector2<f64>) -> bool {
        util::is_within(point.x, self.pos.x, self.dim.x)
            && util::is_within(point.y, self.pos.y, self.dim.y)
    }
}
