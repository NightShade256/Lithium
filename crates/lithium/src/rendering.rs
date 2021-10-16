use crate::SCALE;
use web_sys::CanvasRenderingContext2d;

pub(super) fn draw_cell(context: &CanvasRenderingContext2d, x: u32, y: u32) {
    context.fill_rect(
        (x as usize * SCALE) as f64,
        (y as usize * SCALE) as f64,
        SCALE as f64,
        SCALE as f64,
    );
}

pub(super) fn draw_grid(context: &CanvasRenderingContext2d, w: u32, h: u32) {
    for x in (0..w).step_by(SCALE) {
        for y in (0..h).step_by(SCALE) {
            context.stroke_rect(x as f64, y as f64, SCALE as f64, SCALE as f64);
        }
    }
}
