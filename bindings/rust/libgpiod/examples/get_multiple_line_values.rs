// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
// SPDX-FileCopyrightText: 2023 Kent Gibson <warthog618@gmail.com>
//
// Minimal example of reading multiple lines.

use libgpiod::line::{self, Direction};

fn main() -> libgpiod::Result<()> {
    // Example configuration - customize to suit your situation
    let chip_path = "/dev/gpiochip0";
    let line_offsets = [5, 3, 7];

    let mut lsettings = line::Settings::new()?;
    let mut lconfig = line::Config::new()?;

    lsettings.set_direction(Direction::Input)?;
    lconfig.add_line_settings(&line_offsets, lsettings)?;

    let chip = libgpiod::chip::Chip::open(&chip_path)?;

    let mut rconfig = libgpiod::request::Config::new()?;
    rconfig.set_consumer("get-multiple-line-values")?;

    let request = chip.request_lines(Some(&rconfig), &lconfig)?;
    let values = request.values()?;

    println!("{:?}", values);
    Ok(())
}
