// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod app;
mod backend;
mod config;
mod dgpu;
mod localize;

use localize::localize;

pub fn run() -> lingmo::iced::Result {
    localize();
    app::run()
}
