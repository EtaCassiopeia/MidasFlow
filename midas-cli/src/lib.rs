use std::{
    backtrace::{Backtrace, BacktraceStatus},
    panic, process,
    thread::current,
};

use midas_types::tracing::error;

pub mod errors;

pub fn set_panic_hook() {
    panic::set_hook(Box::new(move |panic_info| {
        // capture errors here
        error!("{}", panic_info);

        let backtrace = Backtrace::capture();
        if backtrace.status() == BacktraceStatus::Captured {
            error!(
                "thread '{}' panicked at '{}'\n stack backtrace:\n{}",
                current()
                    .name()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                panic_info
                    .location()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                backtrace
            );
        }

        process::exit(1);
    }));
}
