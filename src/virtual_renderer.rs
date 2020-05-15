use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn element_to_canvas(element: &web_sys::Element) -> web_sys::HtmlCanvasElement {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    // 300dpi
    let width = 210.0 / 25.4 * 300.0;
    let client_rect = element.get_bounding_client_rect();
    let ratio = width / client_rect.width();
    let height = client_rect.height() * ratio;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.set_fill_style(&JsValue::from("#fff"));
    context.fill_rect(0.0, 0.0, width, height);
    render_node_to_context(
        element,
        &context,
        &(client_rect.x(), client_rect.y()),
        &ratio,
        false,
    );
    canvas
}

fn render_node_to_context(
    node: &web_sys::Node,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    bg_black: bool,
) {
    if let Some(element) = node.dyn_ref::<web_sys::Element>() {
        render_element_to_context(element, context, offset, ratio, bg_black);
    } else if let Some(text) = node.dyn_ref::<web_sys::Text>() {
        render_text_to_context(text, context, offset, ratio, bg_black);
    }
}

fn render_element_to_context(
    element: &web_sys::Element,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    mut bg_black: bool,
) {
    let client_rect = element.get_bounding_client_rect();
    let x = (client_rect.x() - offset.0) * ratio;
    let y = (client_rect.y() - offset.1) * ratio;
    let w = client_rect.width() * ratio;
    let h = client_rect.height() * ratio;
    let computed_style = web_sys::window()
        .unwrap()
        .get_computed_style(element)
        .unwrap()
        .unwrap();
    if let Ok(border_width) = computed_style.get_property_value("border-width") {
        let border_width: String = border_width
            .chars()
            .take_while(|a| a.is_ascii_digit() || *a == '.')
            .collect();
        if let Ok(border_width) = border_width.parse::<f64>() {
            let border_width = border_width * ratio * 2.0;
            if border_width != 0.0 {
                context.set_line_width(border_width);
                context.stroke_rect(x, y, w, h);
            }
        }
    }
    if let Ok(bg_color) = computed_style.get_property_value("background-color") {
        context.set_fill_style(&JsValue::from(&bg_color));
        context.fill_rect(x, y, w, h);
        if bg_color.starts_with("rgb(0") {
            bg_black = true;
        } else if bg_color.starts_with("rgb(255") {
            bg_black = false;
        }
    }
    if let Ok(cn) = computed_style.get_property_value("data-type") {
        if cn == "special-ability-context" {
            if let Ok(seg) = JsValue::from_serde(&[10, 10]) {
                context.set_line_dash(&seg);
                context.set_line_width(0.5 * ratio);
                context.move_to(x + w / 2.0, y);
                context.line_to(x + w / 2.0, y + h);
                context.stroke();
                if let Ok(seg) = JsValue::from_serde(&[0, 0]) {
                    context.set_line_dash(&seg);
                }
            }
        }
    }
    let children = element.child_nodes();
    let len = children.length();
    for i in 0..len {
        if let Some(child) = children.item(i) {
            render_node_to_context(&child, context, offset, ratio, bg_black);
        }
    }
}

fn render_text_to_context(
    text: &web_sys::Text,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    bg_black: bool,
) {
    let client_rect = text.parent_element().unwrap().get_bounding_client_rect();
    let font_size = 4.0 / (25.4 / 300.0);
    let x = (client_rect.x() - offset.0) * ratio;
    let y = (client_rect.y() - offset.1) * ratio + font_size;
    let font_style = font_size.to_string() + "px fot-tsukuardgothic-std";
    if bg_black {
        context.set_fill_style(&JsValue::from("#FFF"));
    } else {
        context.set_fill_style(&JsValue::from("#000"));
    }
    context.set_font(&font_style);
    let text = text.data();
    context.fill_text(&text, x, y);
}
