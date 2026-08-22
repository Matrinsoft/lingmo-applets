// SPDX-License-Identifier: GPL-3.0-or-later

mod app;
mod config;
mod localize;

use crate::localize::localize;

pub fn run() -> lingmo::iced::Result {
    localize();
    app::run()
}
