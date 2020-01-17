mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use wasmuri_components::*;
use wasmuri_container::*;
use wasmuri_core::color::*;
use wasmuri_text::*;

use web_sys::{
    HtmlCanvasElement,
    window
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn default_font_details() -> FontDetails {
    FontDetails::from_str("", "Arial")
}

fn default_font(params: &MouseClickParams) -> Rc<Font> {
    let renderer = params.manager.get_text_renderer().borrow();
    renderer.get_font_by_details(default_font_details()).expect("Default font should be available")
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    let container_manager_cell = ContainerManager::start(window().expect("Should have window").document().expect("Should have document")
    .get_element_by_id("wasm-canvas").expect("Should have element with id 'wasm-canvas'").dyn_into::<HtmlCanvasElement>()
    .expect("Element with id 'wasm-canvas' should be a canvas"), None, true);

    let default_font;

    {
        let container_manager = container_manager_cell.borrow();

        let mut text_renderer = container_manager.get_text_renderer().borrow_mut();
        default_font = text_renderer.add_font(default_font_details());
    }

    let mut container_manager = container_manager_cell.borrow_mut();

    let main_menu = create_main_menu(default_font);

    container_manager.set_container_cell(main_menu);
}

fn create_main_menu(font: Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = SimpleLayer::new(Some(Color::from_rgb(100, 200, 50)));

    let font_clone = Rc::clone(&font);
    add_simple_text_button(&mut layer, -8000, 5000, -4000, 7000, "Simple components", Color::BLUE, &font, TextAlignment::Center, move |agent, _, _| {
        agent.change_container(create_simple_menu(Rc::clone(&font_clone)));
    });

    let font_clone = Rc::clone(&font);
    add_simple_text_button(&mut layer, -8000, 2000, -4000, 4000, "overlapping edit fields", Color::BLUE, &font, TextAlignment::Center, move |agent, _, _| {
        agent.change_container(create_overlapping_edit_menu(Rc::clone(&font_clone)));
    });

    Rc::new(RefCell::new(FlatContainer::new(Box::new(layer))))
}

fn create_simple_menu(font: Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = SimpleLayer::new(Some(Color::from_rgb(100, 200, 50)));

    add_simple_text_button(&mut layer, -8000, 5000, -7000, 7000, "Back", Color::BLUE, &font, TextAlignment::Center, move |agent, _, params| {
        agent.change_container(create_main_menu(default_font(params)));
    });

    add_simple_edit_field(&mut layer, -2000, -1500, 2000, 1500, "Type...", &font);

    Rc::new(RefCell::new(FlatContainer::new(Box::new(layer))))
}

fn create_overlapping_edit_menu(font: Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer1 = SimpleLayer::new(Some(Color::from_rgb(100, 200, 200)));
    let mut layer2 = SimpleLayer::new(None);
    let mut layer3 = SimpleLayer::new(None);

    add_simple_text_button(&mut layer1, -8000, 5000, -7000, 7000, "Back", Color::BLUE, &font, TextAlignment::Center, move |agent, _, params| {
        agent.change_container(create_main_menu(default_font(params)));
    });

    add_simple_edit_field(&mut layer1, -3000, -4000, 3000, -2000, "Layer1", &font);
    add_simple_edit_field(&mut layer2, -3000, -3000, 3000, -1000, "Layer2", &font);
    add_simple_edit_field(&mut layer3, -3000, -2000, 3000, 0, "Layer3", &font);

    Rc::new(RefCell::new(LayeredContainer::new(vec![Box::new(layer1), Box::new(layer2), Box::new(layer3)])))
}