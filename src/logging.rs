//! Minimal `log` implementation for mdriver.
//!
//! mdriver's dependencies emit diagnostics through the `log` crate, but a `log`
//! call is a no-op until a logger is installed. Without one, every message is
//! discarded — most importantly the ~60 `warn!` sites in `usvg` that fire when it
//! skips SVG content it cannot render. Those warnings are the difference between
//! "this Mermaid diagram lost all its labels for an unknown reason" and a one-line
//! explanation, so we install a logger and let the user turn it on.
//!
//! Sources of `log` output we consume today:
//!
//! - `usvg` / `resvg`: skipped SVG elements, unresolvable fonts, invalid filters
//! - `fontdb`: font loading failures
//! - `ureq` / `ureq-proto` / `rustls`: HTTP and TLS activity when fetching URLs
//! - mdriver itself: rendering fallbacks that would otherwise be invisible
//!
//! Two independent sinks, either or both of which may be enabled:
//!
//! - stderr, at `WARN`, via `--verbose`
//! - a file, at `TRACE`, via `--debug-log <FILE>`
//!
//! Deliberately not `env_logger`: it pulls in a regex-based filter that is far
//! more machinery than two fixed levels need.
//!
//! Diagnostics never touch stdout. stdout carries the ANSI stream and terminal
//! image payloads, and interleaving log lines with a kitty escape sequence would
//! corrupt the image.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

/// Level for the stderr sink when `--verbose` is given.
///
/// `WARN` rather than `INFO` because the interesting messages (content skipped,
/// font unresolved) are warnings, while `INFO` and below is mostly HTTP and TLS
/// chatter from ureq that has nothing to do with rendering.
const VERBOSE_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

/// Level for the `--debug-log` file sink. A file is not competing with the
/// user's terminal, so capture everything for bug reports.
const DEBUG_FILE_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

struct Logger {
    /// Enabled by `--verbose`. Writes at [`VERBOSE_LEVEL`].
    stderr: bool,
    /// Enabled by `--debug-log`. Writes at [`DEBUG_FILE_LEVEL`].
    ///
    /// A `Mutex` because `log::Log` requires `Sync` and mdriver renders Mermaid
    /// on a worker thread, so records genuinely arrive from more than one thread.
    file: Option<Mutex<File>>,
    /// Process start, used to stamp file records with elapsed time. Cheaper than
    /// taking on a date/time dependency for what is only ever read relative to
    /// other lines in the same run.
    start: Instant,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let level = metadata.level();
        (self.stderr && level <= VERBOSE_LEVEL)
            || (self.file.is_some() && level <= DEBUG_FILE_LEVEL)
    }

    fn log(&self, record: &log::Record) {
        let level = record.level();

        if self.stderr && level <= VERBOSE_LEVEL {
            // Dependency targets (`usvg::tree`) identify where a warning came from and
            // are worth showing. Our own target is always `mdriver`, which would just
            // repeat the prefix, so drop it.
            let target = record.target();
            let origin = if target == env!("CARGO_PKG_NAME") {
                String::new()
            } else {
                format!("{target}: ")
            };
            // Ignore write errors: stderr may be closed, and failing to report a
            // warning must never take down a render.
            let _ = writeln!(
                io::stderr(),
                "mdriver: {}: {origin}{}",
                level.as_str().to_lowercase(),
                record.args()
            );
        }

        if level <= DEBUG_FILE_LEVEL {
            if let Some(file) = &self.file {
                // A poisoned lock means another thread panicked mid-write. The
                // file may have a torn line, but dropping all later records is
                // worse than continuing, so recover the guard either way.
                let mut file = match file.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                let elapsed = self.start.elapsed().as_millis();
                let _ = writeln!(
                    file,
                    "[+{:>6}ms {:<5} {}] {}",
                    elapsed,
                    level.as_str(),
                    record.target(),
                    record.args()
                );
                // Flush per record. Debug logs are most valuable when the run
                // ends badly, which is exactly when buffered tail lines are lost.
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            let mut file = match file.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            let _ = file.flush();
        }
    }
}

/// Install the global logger.
///
/// `verbose` sends warnings to stderr; `debug_log` sends everything to that
/// path, truncating it. Both may be enabled at once. With neither, no logger is
/// installed at all, so `log` calls stay at their default no-op cost and mdriver
/// remains silent — usvg's benign-but-frequent warnings (`Fallback from X to Y.`
/// fires routinely for emoji) should not appear unless asked for.
///
/// Returns an error only if `debug_log` cannot be opened, which is worth
/// reporting: the user explicitly asked for a log at that path.
pub fn init(verbose: bool, debug_log: Option<&Path>) -> io::Result<()> {
    let file = match debug_log {
        Some(path) => Some(File::create(path)?),
        None => None,
    };

    if !verbose && file.is_none() {
        return Ok(());
    }

    let max = if file.is_some() {
        DEBUG_FILE_LEVEL
    } else {
        VERBOSE_LEVEL
    };

    let logger = Logger {
        stderr: verbose,
        file: file.map(Mutex::new),
        start: Instant::now(),
    };

    // set_boxed_logger only fails if a logger is already installed, which can
    // only happen if init is called twice. Nothing to report, and no reason to
    // fail the run.
    if log::set_boxed_logger(Box::new(logger)).is_ok() {
        log::set_max_level(max);
    }

    Ok(())
}
