mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use wasmuri_components::input::text::TextEditField;
use wasmuri_components::button::text::TextButton;
use wasmuri_components::behavior::render::*;
use wasmuri_components::macros::*;
use wasmuri_container::*;
use wasmuri_container::layer::*;
use wasmuri_core::color::*;
use wasmuri_core::util::*;
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
        default_font = text_renderer.add_font(FontDetails::from_str("", "Arial"));
    }

    let mut container_manager = container_manager_cell.borrow_mut();

    let main_menu = create_main_menu(default_font);

    container_manager.set_container_cell(main_menu);
}

fn create_main_menu(font: Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(Color::from_rgb(100, 200, 50)));

    let font_clone = Rc::clone(&font);

    /*
    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple("Simple components", &font,
        Box::new(AlignedTextLocation::new(Region::new(-8000, 5000, -4000, 7000), TextAlignment::Center, false)),
        TextColors::create_simple_button(Color::BLUE)
    ), Box::new(move |agent, _controller, _helper| {
        agent.change_container(create_simple_menu(Rc::clone(&font_clone)));
    })));*/
    add_simple_text_button(&mut layer, -8000, 5000, -4000, 7000, "Simple components", Color::BLUE, &font, TextAlignment::Center, move |agent, _, _| {
        agent.change_container(create_simple_menu(Rc::clone(&font_clone)));
    });

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}

fn create_simple_menu(font: Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(Color::from_rgb(100, 200, 50)));

    let font_clone = Rc::clone(&font);

    /*
    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple("Back", &font,
        Box::new(AlignedTextLocation::new(Region::new(-8000, 5000, -7000, 7000), TextAlignment::Center, false)),
        TextColors::create_simple_button(Color::BLUE)
    ), Box::new(move |agent, _controller, _helper| {
        agent.change_container(create_main_menu(Rc::clone(&font_clone)));
    })));*/
    add_simple_text_button(&mut layer, -8000, 5000, -7000, 7000, "Back", Color::BLUE, &font, TextAlignment::Center, move |agent, _, _| {
        agent.change_container(create_main_menu(Rc::clone(&font_clone)));
    });

    /*
    layer.add_component(TextEditField::celled(EditTextRenderController::simple_tuple("Type...", &font, 
        Box::new(AlignedTextLocation::new(Region::new(-2000, -1500, 2000, 1500), TextAlignment::LeftCenter, true)), 
        TextColors::new(Color::BLACK, Color::BLACK, Color::from_rgb(200, 200, 200)))));*/
    add_simple_edit_field(&mut layer, -2000, -1500, 2000, 1500, "Type...", &font);

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}