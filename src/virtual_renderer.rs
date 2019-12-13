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
    let width = 210.0 / 25.4 * 600.0;
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
    render_node_to_context(
        element,
        &context,
        &(client_rect.x(), client_rect.y()),
        &ratio,
        false,
        &mut 0,
        1,
    );
    canvas
}

fn render_node_to_context(
    node: &web_sys::Node,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    bg_black: bool,
    line_num: &mut usize,
    column_count: usize,
) {
    if let Some(element) = node.dyn_ref::<web_sys::Element>() {
        if element.tag_name() == "BR" {
            *line_num += 1;
        } else {
            render_element_to_context(
                element,
                context,
                offset,
                ratio,
                bg_black,
                line_num,
                column_count,
            );
        }
    } else if let Some(text) = node.dyn_ref::<web_sys::Text>() {
        render_text_to_context(
            text,
            context,
            offset,
            ratio,
            bg_black,
            line_num,
            column_count,
        );
    }
}

fn render_element_to_context(
    element: &web_sys::Element,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    mut bg_black: bool,
    line_num: &mut usize,
    mut column_count: usize,
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
    if let Ok(cn) = computed_style.get_property_value("column-count") {
        if let Ok(cn) = cn.parse() {
            column_count = cn;
        }
    }
    let children = element.child_nodes();
    let len = children.length();
    *line_num = 0;
    for i in 0..len {
        if let Some(child) = children.item(i) {
            render_node_to_context(
                &child,
                context,
                offset,
                ratio,
                bg_black,
                line_num,
                column_count,
            );
        }
    }
}

fn render_text_to_context(
    text: &web_sys::Text,
    context: &web_sys::CanvasRenderingContext2d,
    offset: &(f64, f64),
    ratio: &f64,
    bg_black: bool,
    line_num: &mut usize,
    column_count: usize,
) {
    let client_rect = text.parent_element().unwrap().get_bounding_client_rect();
    let h = client_rect.height() * ratio;
    let w = client_rect.width() * ratio;
    let font_size = 4.0 / (25.4 / 600.0);
    let x = (client_rect.x() - offset.0) * ratio;
    let y = (client_rect.y() - offset.1) * ratio + font_size * 1.1;
    if bg_black {
        context.set_fill_style(&JsValue::from("#FFF"));
    } else {
        context.set_fill_style(&JsValue::from("#000"));
    }
    context.set_font(&(font_size.to_string() + "px fot-tsukuardgothic-std"));
    let text = text.data();
    if column_count == 1 {
        context.fill_text(&text, x, y + font_size * (*line_num as f64) * 1.1);
    } else {
        let mut alloced = String::new();
        for c in text.chars() {
            alloced.push(c);
            let text_metrics = context.measure_text(&alloced).unwrap();
            if text_metrics.width() > w / 2.0 - font_size / 2.0 {
                if let Some(c) = alloced.pop() {
                    let hh = font_size * (*line_num as f64) * 1.3;
                    if hh + font_size * 2.6 < h {
                        context.fill_text(&alloced, x, y + hh);
                    } else {
                        let hh = hh - h + font_size * 2.6;
                        context.fill_text(&alloced, x + w / 2.0 + font_size / 2.0, y + hh);
                    }
                    alloced.clear();
                    alloced.push(c);
                    *line_num += 1;
                }
            }
        }
        if alloced.len() > 0 {
            let hh = font_size * (*line_num as f64) * 1.3;
            if hh + font_size * 2.6 < h {
                context.fill_text(&alloced, x, y + hh);
            } else {
                let hh = hh - h + font_size * 2.6;
                context.fill_text(&alloced, x + w / 2.0 + font_size / 2.0, y + hh);
            }
        }
    }
}
