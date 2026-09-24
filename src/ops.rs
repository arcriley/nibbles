/*  src/ops.rs  Math operations and kernel dispatch for nibbles.
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

use crate::slice;

/// Matrix-Vector multiplication (`GEMV`) for 4-bit packed weight matrices.
pub fn dispatch_gemv(
    weights: &[u8],
    scales: &[f32],
    block_size: usize,
    x: &[f32],
    out: &mut [f32],
    rows: usize,
    cols: usize,
) {
    assert_eq!(out.len(), rows, "Output vector length must match matrix rows");
    assert_eq!(x.len(), cols, "Input vector length must match matrix cols");

    for r in 0..rows {
        let mut sum = 0.0f32;
        let row_offset = r * cols;

        for c in 0..cols {
            let global_idx = row_offset + c;
            let block_idx = global_idx / block_size;
            let scale = scales[block_idx];

            if let Some(nibble) = slice::get(weights, global_idx) {
                let weight_val = (nibble.value() as f32 - 8.0) * scale;
                sum += weight_val * x[c];
            }
        }
        out[r] = sum;
    }
}

/// Dot product of a 4-bit packed slice against a float vector.
pub fn dispatch_dot(weights: &[u8], scale: f32, x: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    let len = slice::len(weights).min(x.len());

    for i in 0..len {
        if let Some(nibble) = slice::get(weights, i) {
            let weight_val = (nibble.value() as f32 - 8.0) * scale;
            sum += weight_val * x[i];
        }
    }
    sum
}
