pub mod server;
use std::env;
use std::sync::Arc;
use chrono;
use crate::server::Website;
use crate::server::log::Logger;

/// slower but convenient way to log messages
#[macro_export]
macro_rules! info {
    ( $( $p:expr),* ) => {
        Logger::default().info(
            format!("[{}] {}",
                chrono::Local::now().format("%Y/%d/%m; %H:%M:%S"),
                format!($($p,)*)
            )
        )
    }
}

#[macro_export]
macro_rules! warn {
    ( $( $p:expr),* ) => {
        Logger::default().warn(
            format!("[{}] {}",
                chrono::Local::now().format("%Y/%d/%m; %H:%M:%S"),
                format!($($p)*)
            )
        )
    }
}

#[macro_export]
macro_rules! debug {
    ( $( $p:expr),* ) => {
        Logger::default().debug(
            format!("[{}] {}",
                chrono::Local::now().format("%Y/%d/%m; %H:%M:%S"),
                format!($($p)*)
            )
        )
    }
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("2 command line args needed: <website files location> <addr:port>")
    };
    let addr = args.remove(2);
    let site = args.remove(1);
    let site = Arc::new(Website::new(site));
    Logger::default().clear_all_logs();
    server::main(Arc::clone(&site), &addr)
}

#[cfg(test)]
mod test {
    use crate::server::left_right_parse_demo::run_parse_demo;
    use crate::Logger;

    #[test]
    fn parse_test() {
        println!("HELLO WORLD///////////////////////");
        panic!("{:?}", run_parse_demo("x + y", "expr", "json"));
    }

    #[test]
    fn log_1() {

    }

    #[test]
    fn log_macro() {
        Logger::default().clear_all_logs();
        info!("Hello world!")
    }
}