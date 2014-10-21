extern crate iron;
extern crate getopts;
extern crate static_file;
extern crate mount;
extern crate logger;

use std::os;
use std::io::net::ip::Ipv4Addr;

use iron::{Chain, ChainBuilder, Iron};
use static_file::Static;
use mount::Mount;
use getopts::{optopt, optflag, getopts, usage};
use logger::Logger;

fn main() {
    let args = os::args();
    let opts = [
        optopt("f", "folder", "foler to share", "FOLDER"),
        optopt("p", "port", "set server port", "PORT"),
        optflag("h", "help", "print this help menu"),
        optflag("l", "log", "log each request")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("{}",usage("Simple Iron file server", opts));
        return;
    }

    let port_opt = matches.opt_str("p");
    let port_str = match port_opt {
        Some(string)    => string,
        None            => "3000".to_string()
    };
    let port = match from_str(port_str.as_slice()) {
        Some(num)   => num,
        None        => {
            println!("Couldn't parse port number: {}", port_str);
            return;
        }
    };

    let path_opt = matches.opt_str("f");
    let path_str = match path_opt {
        Some(string)    => string,
        None            => ".".to_string()
    };
    let path = Path::new(path_str);
    let path_abs = os::make_absolute(&path);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(path));

    let mut chain = ChainBuilder::new(mount);
    if matches.opt_present("l") {
        let (logger_before, logger_after) = Logger::new(None);
        chain.link_before(logger_before);
        chain.link_after(logger_after);
    }

    Iron::new(chain).listen(Ipv4Addr(127, 0, 0, 1), port);

    println!("Running simple Iron file server on port {}", port);
    println!("Sharing folder {}", path_abs.display());
    println!("Press Ctrl-C to quit");
}
