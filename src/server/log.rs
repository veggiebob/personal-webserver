use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Info: purely for checking or curiosity
const INFO_FILE: &str = "info";

/// Warnings: things that should probably be dealt
/// with at some point but aren't fatal.
const WARN_FILE: &str = "warn";

/// For debug (temporary) messages so that they don't get
/// mixed up with other messages which should be more permanent and meaningful.
const DEBUG_FILE: &str = "debug";

/// Everything gets logged here.
const VERBOSE_FILE: &str = "verbose";

/// The logger!
pub struct Logger {
    log_location: PathBuf,
}

impl Logger {
    pub fn new(folder: &str) -> Result<Logger, String> {
        std::fs::create_dir_all(folder)
            .map_err(|e| e.to_string());
        Ok(Logger {
            log_location: {
                let mut path = PathBuf::new();
                path.push(folder);
                path
            }
        })
    }
    pub fn clear_all_logs(&mut self) {
        for filename in vec![INFO_FILE, WARN_FILE, DEBUG_FILE, VERBOSE_FILE] {
            self.open(
                filename,
                OpenOptions::new().create(true).write(true).truncate(true),
                |f| f.flush()
            );
        }
    }

    /// automatically add a new line
    pub fn info<M: Display>(&mut self, message: M) {
        let opt = self.open_append();
        self.open(
            INFO_FILE,
            &opt,
            |f| {
                writeln!(f, "{}", message)
            }
        );
        self.verbose(message);
    }

    /// automatically add a new line
    pub fn debug<M: Display>(&mut self, message: M) {
        let opt = self.open_append();
        self.open(
            DEBUG_FILE,
            &opt,
            |f| {
                writeln!(f, "{}", message)
            }
        );
        self.verbose(message);
    }

    /// automatically adds a new line
    pub fn warn<M: Display>(&mut self, message: M) {
        let opt = self.open_append();
        self.open(
            WARN_FILE,
            &opt,
            |f| {
                writeln!(f, "{}", message)
            }
        );
        self.verbose(message);
    }

    /// Contains ALL logs!
    pub fn verbose<M: Display>(&mut self, message: M) {
        let opt = self.open_append();
        self.open(
            VERBOSE_FILE,
            &opt,
            |f| {
                writeln!(f, "{}", message)
            }
        );
    }

    fn open<T, F: Fn(&mut File) -> T>(&mut self, filename: &str, open_options: &OpenOptions, f: F) -> Result<T, String> {
        self.log_location.push(filename);

        let mut file = open_options.open(&self.log_location).map_err(|e| e.to_string())?;
        let v = f(&mut file);
        // file is closed at the end of scope

        self.log_location.pop();
        Ok(v)
    }

    fn open_append(&self) -> OpenOptions {
        let mut oo = OpenOptions::new();
        oo.append(true);
        oo.create(true);
        oo
    }
}


const DEFAULT_LOG_FOLDER_LOCATION: &str = "log";
impl Default for Logger {
    fn default() -> Self {
        Logger::new(DEFAULT_LOG_FOLDER_LOCATION).unwrap()
    }
}