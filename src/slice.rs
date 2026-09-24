/*  src/slice.rs  Slice helper functions operating on packed nibble buffers.
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

use crate::u4;

/// Returns the total number of 4-bit elements in a packed byte slice.
#[inline]
pub fn len(bytes: &[u8]) -> usize {
    bytes.len() * 2
}

/// Returns `true` if the byte slice contains no elements.
#[inline]
pub fn is_empty(bytes: &[u8]) -> bool {
    bytes.is_empty()
}

/// Retrieves a `u4` at the specified logical nibble index from a packed byte slice.
#[inline]
pub fn get(bytes: &[u8], index: usize) -> Option<u4> {
    if index >= len(bytes) {
        return None;
    }
    // Safety: bounds check performed above
    let byte = unsafe { *bytes.get_unchecked(index >> 1) };
    let val = if (index & 1) == 0 {
        byte & 0x0F
    } else {
        (byte >> 4) & 0x0F
    };
    Some(u4::new(val))
}

/// An iterator over `u4` elements in a packed byte slice.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    bytes: &'a [u8],
    index: usize,
    len: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u4;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let item = get(self.bytes, self.index);
        self.index += 1;
        item
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

/// Creates a `u4` iterator over a packed byte slice.
#[inline]
pub fn iter(bytes: &[u8]) -> Iter<'_> {
    Iter {
        bytes,
        index: 0,
        len: len(bytes),
    }
}
