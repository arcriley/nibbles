/*  src/lib.rs  Library entry point and primary interfaces for nibbles.
 *
 *  Copyright 2026 Emerge Cooperative
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *                                                                            */

//! `nibbles` provides optimized math operations and zero-copy slice abstractions
//! on arrays of packed 4-bit unsigned integers (`u4`).

pub mod ops;
pub mod slice;
pub mod types;

pub use types::u4;

/// Accelerated Matrix-Vector multiplication on packed 4-bit weights.
#[inline]
pub fn gemv(
    weights: &[u8],
    scales: &[f32],
    block_size: usize,
    x: &[f32],
    out: &mut [f32],
    rows: usize,
    cols: usize,
) {
    ops::dispatch_gemv(weights, scales, block_size, x, out, rows, cols);
}

/// Dot product of a 4-bit packed slice against a float vector.
#[inline]
pub fn dot(weights: &[u8], scale: f32, x: &[f32]) -> f32 {
    ops::dispatch_dot(weights, scale, x)
}
