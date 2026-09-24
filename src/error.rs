/*  src/error.rs  Error types for the shaled crate.
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
 *                                                                          */

use thiserror::Error;

/// Errors returned by `shaled` layer loading and tensor execution routines.
#[derive(Debug, Error)]
pub enum NibbleError {
    /// Standard I/O failures when opening or reading model shards.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid model architecture metadata or corrupt header bytes.
    #[error("Invalid model format: {0}")]
    InvalidFormat(String),
}
