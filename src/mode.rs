use crate::canvas::{BufColor, CanvasBuffer, RgbColor};
use crate::util::Point;

use minifb::Window;
use serde::Deserialize;

// Enter crop mode with C: select crop area with mouse
// Enter brush mode with B: paint with mouse, resize brush with scroll, 1, 2, 3, 4, 5, 6, 7 and Ctrl+Scrollwheel to switch colors
// Esc to enter view mode and hide all UI(exit of other modes)

#[derive(Clone, Deserialize, Debug)]
pub enum Mode {
    Brush { last_brush_pos: Option<Point> },
    // Eraser
    // Text { text: String, pos: Point, size: f32 },
    //
    Crop { selection: Option<CropSelection> },
    View,
}

#[derive(Clone, Deserialize, PartialEq, Eq, Debug)]
pub struct Brush {
    pub size: u32,
    pub color: RgbColor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShouldSave {
    Continue,
    Save,
}

#[derive(Debug)]
pub struct Input<'w> {
    pub mouse_pos: Point,
    pub mouse_down: bool,
    pub scroll: i32,

    pub window: &'w Window,
}

pub fn brush_mode(
    input: &Input,
    brush: &mut Brush,
    last_brush_pos: &mut Option<Point>,
    temporal_buffer: &mut CanvasBuffer,
    composite_buffer: &mut CanvasBuffer,
) -> ShouldSave {
    let brush_pos = Some(input.mouse_pos).filter(|_| input.mouse_down);
    let mut should_save = ShouldSave::Continue;

    if *last_brush_pos != brush_pos {
        if let Some(pos) = brush_pos {
            let color = BufColor::from(brush.color);
            let mut paint_pos =
                |x, y| temporal_buffer.draw_square_s(x, y, brush.size as i32, color);

            if let Some(last) = last_brush_pos {
                let (pos_x, pos_y) = (pos.0 as i32, pos.1 as i32);
                let (last_x, last_y) = (last.0 as i32, last.1 as i32);

                let vec = ((pos_x - last_x) as f32, (pos_y - last_y) as f32);
                let dist = (vec.0.powi(2) + vec.1.powi(2)).sqrt().abs();
                let vec = ((vec.0 / dist), (vec.1 / dist));

                for i in 1..=(dist as i32) {
                    let x = (last.0 as f32 + vec.0 * i as f32) as i32;
                    let y = (last.1 as f32 + vec.1 * i as f32) as i32;

                    paint_pos(x, y);
                }
            } else {
                paint_pos(pos.0, pos.1);
            }
        } else {
            should_save = ShouldSave::Save;
        }
    }
    brush.size = (brush.size as i32 + input.scroll).clamp(2, i32::MAX) as u32;

    // Brush preview
    composite_buffer.draw_square_s(
        input.mouse_pos.0,
        input.mouse_pos.1,
        brush.size as i32,
        brush.color.into(),
    );

    *last_brush_pos = brush_pos;
    should_save
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Debug)]
pub struct CropSelection {
    pub start: Point,
    pub end: Point,
}

pub fn crop_mode(
    input: &Input,
    selection: &mut Option<CropSelection>,
    temporal_buffer: &mut CanvasBuffer,
    composite_buffer: &mut CanvasBuffer,
) -> ShouldSave {
    let mut should_save = ShouldSave::Continue;
    let select_pos = Some(input.mouse_pos).filter(|_| input.mouse_down);

    if let Some(CropSelection { start, end }) = *selection {
        if let Some(end) = select_pos {
            *selection = Some(CropSelection { start, end });

            // Display crop guides
            composite_buffer.draw_guides(start, end);
        } else {
            // Do the crop when the user releases the button
            let x = start.0.min(end.0);
            let y = start.1.min(end.1);
            let w = start.0.max(end.0) - x;
            let h = start.1.max(end.1) - y;

            if w > 0
                && h > 0
                && w < temporal_buffer.width() as i32
                && h < temporal_buffer.height() as i32
            {
                let (x, y, w, h) = (x as u32, y as u32, w as u32, h as u32);
                temporal_buffer.crop(x, y, w, h);
                should_save = ShouldSave::Save;
            }

            *selection = None;
        }
    } else {
        *selection = select_pos.map(|a| CropSelection { start: a, end: a });
    }

    should_save
}
