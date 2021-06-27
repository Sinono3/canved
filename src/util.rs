use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};

pub type Point = (i32, i32);

pub fn create_window(width: usize, height: usize) -> Window {
    let mut window = Window::new(
        "canved",
        width,
        height,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // NOTE: Limit is removed because it causes choppy brush strokes.
    // This can be fixed with proper velocity-based brush smoothing
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));
    window.set_cursor_style(CursorStyle::Crosshair);
    window
}

pub fn window_point_to_buffer_point(
    x: i32,
    y: i32,
    ww: i32,
    wh: i32,
    bw: i32,
    bh: i32,
) -> Point {
    let mut bar_x = 0;
    let mut bar_y = 0;

    let mut biw_width = ww as f32;
    let mut biw_height = wh as f32;

    let window_aspect = ww as f32 / wh as f32;
    let buffer_aspect = bw as f32 / bh as f32;

    if window_aspect > buffer_aspect {
        // picture has vertical black bars
        biw_width = bw as f32 * wh as f32 / bh as f32;
        bar_x = ((ww as f32 / 2.0) - (biw_width as f32 / 2.0)) as i32;
    } else {
        // picture has horizontal black bars
        biw_height = bh as f32 * ww as f32 / bw as f32;
        bar_y = ((wh as f32 / 2.0) - (biw_height as f32 / 2.0)) as i32;
    }

    (
        ((x - bar_x) as f32 * bw as f32 / biw_width) as i32,
        ((y - bar_y) as f32 * bh as f32 / biw_height) as i32,
    )
}
