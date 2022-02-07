pub mod server;
use std::env;
use std::sync::Arc;
use crate::server::Website;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("2 command line args needed: <website files location> <addr:port>")
    };
    let addr = args.remove(2);
    let site = args.remove(1);
    let site = Arc::new(Website::new(site));
    server::main(Arc::clone(&site), &addr)
}

#[cfg(test)]
mod test {
    use crate::server::left_right_parse_demo::run_parse_demo;

    #[test]
    fn parse_test () {
        println!("HELLO WORLD///////////////////////");
        panic!("{:?}", run_parse_demo("x + y", "expr", "json"));
    }
}