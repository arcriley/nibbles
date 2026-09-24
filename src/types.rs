/*  src/types.rs  Fundamental 4-bit integer types for nibbles.
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

/// Unsigned 4-bit integer type representing values from 0 to 15.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct u4(u8);

impl u4 {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(15);

    /// Creates a `u4` by masking off all but the lower 4 bits.
    #[inline]
    pub const fn new(val: u8) -> Self {
        Self(val & 0x0F)
    }

    /// Returns the underlying primitive `u8` value.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl From<u8> for u4 {
    #[inline]
    fn from(val: u8) -> Self {
        Self::new(val)
    }
}

impl From<u4> for u8 {
    #[inline]
    fn from(val: u4) -> Self {
        val.0
    }
}
