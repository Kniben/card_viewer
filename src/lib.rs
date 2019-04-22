extern crate kdtree;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate web_sys;

use card::Card;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use nalgebra as na;
use std::cmp::Ordering;
use std::iter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Event, HtmlCanvasElement, HtmlImageElement, MouseEvent,
    UiEvent, Window,
};

mod card;
mod util;

#[wasm_bindgen]
pub struct State {
    cards: Vec<Card>,
    mouse_pos: na::Vector2<f64>,
    prev_mouse_pos: na::Vector2<f64>,
    last_tick: f64,
    avg_fps: f64,
}

#[wasm_bindgen]
impl State {
    pub fn init() -> State {
        State {
            cards: iter::repeat(Card {
                name: "Omniscience",
                pos: na::Vector2::new(0.0, 0.0),
                dim: na::Vector2::new(240.0, 340.0),
                image: document()
                    .get_element_by_id("omniscience_png")
                    .expect("Couldn't find image to draw!")
                    .dyn_into::<HtmlImageElement>()
                    .expect("Unable to cast image-element!"),
                grabbing: false,
            })
            .take(100)
            .collect(),
            mouse_pos: na::zero(),
            prev_mouse_pos: na::zero(),
            last_tick: 0.0,
            avg_fps: 0.0,
        }
    }

    pub fn on_document_event(&mut self, event: Event) {
        let event_type = event.type_();

        if let Some(mouse_event) = event.dyn_into::<MouseEvent>().ok() {
            match (event_type.as_str(), mouse_event.button()) {
                ("mousemove", _) => {
                    let bounding_rect = canvas().get_bounding_client_rect();
                    self.mouse_pos.x = mouse_event.client_x() as f64 - bounding_rect.x();
                    self.mouse_pos.y = mouse_event.client_y() as f64 - bounding_rect.y();
                }
                ("mousedown", 0) => grab_card(&mut self.cards, &self.mouse_pos),
                ("mouseup", 0) => {
                    for card in &mut self.cards {
                        card.grabbing = false;
                    }
                }
                ("DOMMouseScroll", _) => log(&format!("{:?}", mouse_event.detail())),
                _ => (),
            }
        }
    }

    pub fn animate(&mut self) {
        let canvas = canvas();
        let ctx = canvas_context();
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        layout(&mut self.cards);

        if let Some(card) = self.cards.iter_mut().find(|card| card.grabbing) {
            card.pos += self.mouse_pos - self.prev_mouse_pos;
        }
        self.prev_mouse_pos = self.mouse_pos;

        self.cards.sort_by(card_render_cmp);
        for card in self.cards.iter().rev() {
            render_card(&ctx, card);
        }

        let now = window().performance().unwrap().now();
        self.avg_fps = 0.95 * self.avg_fps + 0.05 * (1_000.0 / (now - self.last_tick));
        ctx.fill_text(&format!("{:.0?}", self.avg_fps), 10.0, 10.0)
            .unwrap();
        self.last_tick = now;
    }
}

fn layout(cards: &mut Vec<Card>) {
    let approx_card_title_height = 36.0;

    let mut tree = KdTree::new(2);

    cards
        .iter()
        .for_each(|card| tree.add([card.pos.x, card.pos.y], card.pos).unwrap());

    for card in cards.iter_mut() {
        let neighbors: Vec<_> = tree
            .nearest(&[card.pos.x, card.pos.y], 3, &squared_euclidean)
            .unwrap()
            .iter()
            .skip(1)
            .take_while(|(sq_dist, _)| sq_dist < &10000.0)
            .map(|(_, &neighbor)| neighbor)
            .collect();

        let below_pos = neighbors
            .iter()
            .find(|&neighbor| neighbor.y - card.pos.y < 0.0);

        below_pos.into_iter().for_each(|other_pos| {
            let offset = card.pos - other_pos;
            let offset_y_sign = if offset.y == 0.0 {
                random()
            } else {
                offset.y.signum()
            };
            let target_offset = na::Vector2::new(0.0, approx_card_title_height * offset_y_sign);
            if !card.grabbing {
                card.pos += (target_offset - offset) * 0.5;
            }
        });
    }
}

fn card_render_cmp(a: &Card, b: &Card) -> Ordering {
    match (a.grabbing, b.grabbing) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (_, _) => b
            .pos
            .y
            .partial_cmp(&a.pos.y)
            .expect("Couldn't compare card y-pos"),
    }
}

fn grab_card(cards: &mut Vec<Card>, point: &na::Vector2<f64>) {
    let found_card = cards.iter_mut().find(|card| card.is_inside(point));

    if let Some(card) = found_card {
        card.grabbing = true;
    }
}

fn render_card(ctx: &CanvasRenderingContext2d, card: &Card) {
    ctx.save();
    ctx.translate(card.pos.x - card.dim.x * 0.5, card.pos.y - card.dim.y * 0.5)
        .expect("Failed to translate when drawing card!");
    ctx.draw_image_with_html_image_element_and_dw_and_dh(
        &card.image,
        0.0,
        0.0,
        card.dim.x,
        card.dim.y,
    )
    .expect("Failed drawing card image!");
    ctx.restore();
}

fn canvas_context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .expect("No 2d context for canvas!")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Coundn't cast into 2dcontext!")
}

fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("glCanvas")
        .expect("glCanvas was not found!")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Couldn't cast to HtmlCanvasElement!")
}

fn document() -> Document {
    window().document().expect("No document found in window!")
}

fn window() -> Window {
    web_sys::window().expect("No window found!")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f64;
}
