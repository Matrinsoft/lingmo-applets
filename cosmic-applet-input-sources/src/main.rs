// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

fn main() -> lingmo::iced::Result {
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    cosmic_applet_input_sources::run()
}
