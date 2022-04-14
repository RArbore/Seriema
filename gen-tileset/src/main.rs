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

use std::env::*;

use image::io::Reader;

fn main() {
    let args: Vec<String> = args().collect();
    for arg in args[1..].iter() {
        let img = Reader::open(arg)
            .expect(&format!("Couldn't open file: {}.", arg)[..])
            .decode()
            .expect(&format!("File couldn't be coded: {}.", arg)[..]);
    }
}