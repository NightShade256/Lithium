mod logic;
mod rendering;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    platform::web::WindowBuilderExtWebSys,
    window::WindowBuilder,
};

const SCALE: usize = 40;

fn main() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from_str("black"));
    context.set_stroke_style(&JsValue::from_str("grey"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .with_inner_size(PhysicalSize::new(1280, 720))
        .build(&event_loop)
        .expect("failed to create a window");

    let window_size = window.inner_size();

    let scaled_w = (window_size.width as usize) / SCALE;
    let scaled_h = (window_size.height as usize) / SCALE;

    let cells = Rc::new(RefCell::new(vec![vec![false; scaled_w]; scaled_h]));
    let cells_clone = Rc::clone(&cells);
    let cells_clone_two = Rc::clone(&cells);

    let mut mouse_position = PhysicalPosition::default();

    let is_playing = Rc::new(Cell::new(false));
    let is_playing_clone = Rc::clone(&is_playing);

    let start_button = Rc::new(RefCell::new(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("start")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap(),
    ));

    let start_button_clone = Rc::clone(&start_button);

    let start_handler = Closure::wrap(Box::new(move || {
        let playing = is_playing_clone.get();
        is_playing_clone.set(!playing);

        if playing {
            start_button_clone.borrow().set_inner_text("Start");
        } else {
            start_button_clone.borrow().set_inner_text("Stop");
        }
    }) as Box<dyn Fn()>);

    start_button
        .borrow()
        .set_onclick(Some(start_handler.as_ref().unchecked_ref()));

    let reset_button = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("reset")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();

    let reset_handler = Closure::wrap(Box::new(move || {
        cells_clone
            .borrow_mut()
            .iter_mut()
            .flatten()
            .for_each(|cell| *cell = false);
    }) as Box<dyn Fn()>);

    reset_button.set_onclick(Some(reset_handler.as_ref().unchecked_ref()));

    let interval_handler = Closure::wrap(Box::new(move || {
        if is_playing.get() {
            let mut next = vec![vec![false; scaled_w]; scaled_h];

            for (i, slice) in cells_clone_two.borrow().iter().enumerate() {
                for (j, cell) in slice.iter().enumerate() {
                    let neighbours = logic::count_neighbours(&cells_clone_two.borrow(), i, j);

                    if *cell {
                        if !(2..=3).contains(&neighbours) {
                            next[i][j] = false;
                        } else {
                            next[i][j] = true;
                        }
                    } else if neighbours == 3 {
                        next[i][j] = true;
                    }
                }
            }

            *cells_clone_two.borrow_mut() = next;
        }
    }) as Box<dyn Fn()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            interval_handler.as_ref().unchecked_ref(),
            250,
        )
        .unwrap();

    event_loop.run(move |event, _, _| match event {
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            mouse_position = position;
        }

        Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    ..
                },
            ..
        } => {
            let x = (mouse_position.x / (SCALE as f64)).trunc() as usize;
            let y = (mouse_position.y / (SCALE as f64)).trunc() as usize;

            if let Some(slice) = cells.borrow_mut().get_mut(y) {
                if let Some(cell) = slice.get_mut(x) {
                    *cell = !(*cell);
                }
            }
        }

        Event::MainEventsCleared => {
            window.request_redraw();
        }

        Event::RedrawRequested(_) => {
            for (i, rows) in cells.borrow().iter().enumerate() {
                for (j, cell) in rows.iter().enumerate() {
                    if *cell {
                        context.set_fill_style(&JsValue::from_str("black"));
                    } else {
                        context.set_fill_style(&JsValue::from_str("white"));
                    }

                    rendering::draw_cell(&context, j as u32, i as u32);
                }
            }

            rendering::draw_grid(&context, window_size.width, window_size.height);
        }

        _ => {}
    });
}
