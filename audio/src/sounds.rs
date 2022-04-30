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

use rodio::Source;

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Sound {
    TestSound1,
}

const SOUND_DATA: [&[u8]; 1] = [include_bytes!("../../assets/gen/test-sound1.wav")];

pub fn play_sound(sound: Sound) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let cursor = std::io::Cursor::new(SOUND_DATA[sound as usize]);
    let source = rodio::Decoder::new(cursor).unwrap();
    _ = stream_handle.play_raw(source.convert_samples());
}
