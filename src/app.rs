use crate::canvas::{CanvasBuffer, ColorbarPos, RgbColor};
use crate::mode::*;
use crate::util::{create_window, window_point_to_buffer_point};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};
use serde::Deserialize;

pub fn edit(initial_buffer: CanvasBuffer, options: EditorOptions) -> CanvasBuffer {
    App::new(initial_buffer, options).run()
}

#[derive(Deserialize)]
pub struct EditorOptions {
    pub brush: Brush,
    pub mode: Mode,
    pub colors: Vec<RgbColor>,
    // TODO: Max version count
    // TODO: Keybinds?
}

impl Default for EditorOptions {
    fn default() -> Self {
        let brush = Brush {
            size: 2,
            color: RgbColor([0xFF, 0x00, 0x00]),
        };
        let mode = Mode::Brush {
            last_brush_pos: None,
        };
        let colors = vec![
            RgbColor([0, 0, 0]),
            RgbColor([255, 255, 255]),
            RgbColor([255, 0, 0]),
            RgbColor([0, 255, 0]),
            RgbColor([0, 0, 255]),
            RgbColor([255, 255, 0]),
            RgbColor([255, 0, 255]),
            RgbColor([0, 255, 255]),
        ];

        Self {
            brush,
            mode,
            colors,
        }
    }
}

struct App {
    window: Window,
    mode: Mode,
    brush: Brush,
    colors: Vec<RgbColor>,
    selected_color: u32,
    temporal_buffer: CanvasBuffer,
    versions: Vec<CanvasBuffer>,
    version_index: usize,
}

impl App {
    fn new(initial_buffer: CanvasBuffer, options: EditorOptions) -> Self {
        let temporal_buffer = initial_buffer.clone();
        let versions = vec![initial_buffer.clone()];
        let version_index = 0;

        let window = create_window(
            initial_buffer.width() as usize,
            initial_buffer.height() as usize,
        );

        Self {
            window,
            mode: options.mode,
            brush: options.brush,

            colors: options.colors,
            selected_color: 0,

            temporal_buffer,
            versions,
            version_index,
        }
    }
    fn run(mut self) -> CanvasBuffer {
        while self.window.is_open() && !self.window.is_key_down(Key::Q) {
            let mut composite_buffer = self.temporal_buffer.clone();

            let window_size = self.window.get_size();
            let mouse_pos = self
                .window
                .get_unscaled_mouse_pos(MouseMode::Pass)
                .map(|(x, y)| {
                    window_point_to_buffer_point(
                        x as i32,
                        y as i32,
                        window_size.0 as i32,
                        window_size.1 as i32,
                        composite_buffer.width() as i32,
                        composite_buffer.height() as i32,
                    )
                });
            let mouse_down = self.window.get_mouse_down(MouseButton::Left);
            let scroll = self
                .window
                .get_scroll_wheel()
                .map_or(0, |(_, y)| (y as i32).signum());

            let input = Input {
                mouse_pos,
                mouse_down,
                scroll,
                window: &self.window,
            };

            // Differ behaviour based on current mode
            let should_save = match self.mode {
                Mode::Brush {
                    ref mut last_brush_pos,
                } => brush_mode(
                    &input,
                    &mut self.brush,
                    last_brush_pos,
                    &mut self.temporal_buffer,
                    &mut composite_buffer,
                ),
                Mode::Crop { ref mut selection } => crop_mode(
                    &input,
                    selection,
                    &mut self.temporal_buffer,
                    &mut composite_buffer,
                ),
                Mode::View => ShouldSave::Continue,
            };

            // Canvas buffer versioning
            if should_save == ShouldSave::Save {
                // Save new version.
                self.version_index += 1;

                // Delete undone versions that still exist.
                if self.version_index < self.versions.len() {
                    self.versions.truncate(self.version_index);
                    self.versions.shrink_to_fit();
                }

                self.versions.push(self.temporal_buffer.clone());
            }

            // Keymaps
            let keys = self.window.get_keys_pressed(KeyRepeat::No);
            let ctrl = self.window.is_key_down(Key::LeftCtrl);
            let shift = self.window.is_key_down(Key::LeftShift);
            let z = self.window.is_key_pressed(Key::Z, KeyRepeat::Yes);

            if ctrl && z {
                let add_version = |i: i32| {
                    let i = self.version_index as i32 + i;

                    if i < 0 {
                        return 0;
                    }
                    if i >= self.versions.len() as i32 {
                        return self.versions.len() - 1;
                    }

                    i as usize
                };

                if !shift {
                    self.version_index = add_version(-1); // Undo
                } else {
                    self.version_index = add_version(1); // Redo
                }

                self.temporal_buffer = self.versions[self.version_index].clone();
            }

            let mut color_change = None;

            if let Some(keys) = keys {
                for key in keys {
                    match key {
                        // Mode switching
                        Key::Escape => self.mode = Mode::View,
                        Key::B => {
                            self.mode = Mode::Brush {
                                last_brush_pos: None,
                            }
                        }
                        Key::C => self.mode = Mode::Crop { selection: None },
                        // Color switching
                        Key::Key1 => color_change = Some(0),
                        Key::Key2 => color_change = Some(1),
                        Key::Key3 => color_change = Some(2),
                        Key::Key4 => color_change = Some(3),
                        Key::Key5 => color_change = Some(4),
                        Key::Key6 => color_change = Some(5),
                        Key::Key7 => color_change = Some(6),
                        Key::Key8 => color_change = Some(7),
                        Key::Key9 => color_change = Some(8),
                        _ => (),
                    }
                }
            }

            if let Some(new_color) = color_change {
                self.selected_color = new_color.clamp(0, self.colors.len() as u32);
                self.brush.color = self.colors[new_color as usize];
            }

            // Colorbar
            let draw_colorbar = match self.mode {
                Mode::Brush { .. } => true,
                Mode::Crop { .. } => false,
                Mode::View => false,
            };

            if draw_colorbar {
                composite_buffer.draw_colorbar(&self.colors, self.selected_color, ColorbarPos::Top);
            }

            self.window
                .update_with_buffer(
                    unsafe { composite_buffer.raw_data() },
                    composite_buffer.width() as usize,
                    composite_buffer.height() as usize,
                )
                .unwrap();
        }

        self.temporal_buffer
    }
}
