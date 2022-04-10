/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::*;

const NUM_KEYCODES: usize = 164;

pub enum ControllerScheme {
    KeyboardMouse {
        jump_key: VirtualKeyCode,
        crouch_key: VirtualKeyCode,
        left_key: VirtualKeyCode,
        right_key: VirtualKeyCode,
    },
}

pub struct Controller {
    pressed: [bool; NUM_KEYCODES],
    cursor_x: f64,
    cursor_y: f64,
    scheme: ControllerScheme,
}

#[derive(Clone, Copy, Debug)]
pub struct UserInput {
    pub jump: bool,
    pub crouch: bool,
    pub left: bool,
    pub right: bool,
    pub angle: f32,
}

impl Controller {
    pub fn new(scheme: ControllerScheme) -> Self {
        Controller {
            pressed: [false; NUM_KEYCODES],
            cursor_x: 0.0,
            cursor_y: 0.0,
            scheme,
        }
    }

    pub fn process_window_event(
        &mut self,
        event: &WindowEvent,
        size: &winit::dpi::PhysicalSize<u32>,
    ) -> bool {
        match (event, &self.scheme) {
            (
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                },
                ControllerScheme::KeyboardMouse { .. },
            ) => {
                self.pressed[*keycode as usize] = *state == ElementState::Pressed;
                true
            }
            (
                WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y },
                    ..
                },
                ControllerScheme::KeyboardMouse { .. },
            ) => {
                self.cursor_x = *x - size.width as f64 / 2.0;
                self.cursor_y = *y - size.height as f64 / 2.0;
                true
            }
            _ => false,
        }
    }

    pub fn get_user_input(&self, ax: f32, ay: f32) -> UserInput {
        match self.scheme {
            ControllerScheme::KeyboardMouse {
                jump_key,
                crouch_key,
                left_key,
                right_key,
            } => {
                let adjusted_x = self.cursor_x as f32 - ax;
                let adjusted_y = self.cursor_y as f32 - ay;
                UserInput {
                    jump: self.pressed[jump_key as usize],
                    crouch: self.pressed[crouch_key as usize],
                    left: self.pressed[left_key as usize],
                    right: self.pressed[right_key as usize],
                    angle: if adjusted_x == 0.0 {
                        if adjusted_y > 0.0 {
                            std::f32::consts::PI / 2.0
                        } else {
                            3.0 * std::f32::consts::PI / 2.0
                        }
                    } else {
                        if adjusted_x < 0.0 {
                            (adjusted_y / adjusted_x).atan() + std::f32::consts::PI
                        } else {
                            (adjusted_y / adjusted_x).atan()
                        }
                    },
                }
            }
        }
    }
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            jump: false,
            crouch: false,
            left: false,
            right: false,
            angle: 0.0,
        }
    }
}
