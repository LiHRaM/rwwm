//! RWWM is an [Anvil](https://github.com/smithay/smithay/) fork which adds tiling window management.

#![warn(rust_2018_idioms)]

#[macro_use]
extern crate glium;
#[macro_use]
extern crate slog;
#[macro_use(define_roles)]
extern crate smithay;

use std::{cell::RefCell, rc::Rc};

use slog::{Drain, Logger};
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};

#[macro_use]
mod shaders;
mod buffer_utils;
mod glium_drawer;
mod input_handler;
mod shell;
mod shm_load;
mod state;
#[cfg(feature = "udev")]
mod udev;
mod window_map;
#[cfg(feature = "winit")]
mod winit;

use state::AnvilState;

static DISPLAY: &str = "DISPLAY";

fn start_logger() -> Logger {
    slog::Logger::root(
        slog_async::Async::default(slog_term::term_full().fuse()).fuse(),
        o!(),
    )
}

fn main() {
    let log = start_logger();

    let mut event_loop = EventLoop::<AnvilState>::new().unwrap();
    let display = Rc::new(RefCell::new(Display::new()));

    let is_tty = std::env::var_os(DISPLAY).is_none();
    if is_tty {
        info!(log, "Starting anvil on a tty using udev");
        #[cfg(feature = "udev")]
        if let Err(()) = udev::run_udev(display, &mut event_loop, log.clone()) {
            crit!(log, "Failed to initialize tty backend.");
        }
    } else {
        info!(log, "Starting anvil with winit backend");
        #[cfg(feature = "winit")]
        if let Err(()) = winit::run_winit(display, &mut event_loop, log.clone()) {
            crit!(log, "Failed to initialize winit backend.");
        }
    }
}
