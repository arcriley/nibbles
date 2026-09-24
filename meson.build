#  meson.build  Build specification for nibbles
#
#  Copyright 2026 Emerge Cooperative
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU Affero General Public License as published
#  by the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License
#  along with this program.  If not, see <https://www.gnu.org/licenses/>.

project('nibbles', 'rust',
          version: '0.1.0',
    meson_version: '>= 1.0.0',
  default_options: [ 'warning_level=2', 'werror=false' ],
)

sh_bin = find_program('sh', required: true)
cargo_bin = find_program('cargo', required: true)

cargo_build_dir = meson.project_build_root() / 'src'
cargo_home_dir = meson.project_build_root() / 'cargo-home'
manifest_path = meson.project_source_root() / 'Cargo.toml'

if get_option('buildtype') == 'release'
  cargo_release_flag = '--release'
  rust_target = 'release'
else
  cargo_release_flag = ''
  rust_target = 'debug'
endif

# Build and install static library artifact
cargo_lib_target = custom_target(
  'nibbles-lib',
  output: 'libnibbles.a',
  build_by_default: true,
  build_always_stale: true,
  console: true,
  install: true,
  install_dir: get_option('libdir'),
  command: [
    sh_bin, '-c',
    'CARGO_HOME=' + cargo_home_dir + ' ' +
    cargo_bin.full_path() + ' build ' +
    '--manifest-path ' + manifest_path + ' ' +
    '--target-dir ' + cargo_build_dir + ' ' +
    '--lib ' + cargo_release_flag + ' && ' +
    'cp ' + cargo_build_dir / rust_target / 'libnibbles.a' + ' @OUTPUT@'
  ],
)