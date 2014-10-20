extern crate iron;
extern crate getopts;
extern crate static_file;
extern crate mount;

use std::os;
use std::io::net::ip::Ipv4Addr;

use iron::Iron;
use static_file::Static;
use mount::Mount;
use getopts::{optopt, optflag, getopts, usage};

fn main() {
    let args = os::args();
    let opts = [
        optopt("p", "port", "set server port", "PORT"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("{}",usage("Hello world server", opts));
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

    let mut mount = Mount::new();
    let path = Path::new(".");
    let path_abs = os::make_absolute(&path);
    mount.mount("/", Static::new(path));
    Iron::new(mount).listen(Ipv4Addr(127, 0, 0, 1), port);
    println!("Running simple server on port {}", port);
    println!("Sering folder {}", path_abs.display());
    println!("Press Ctrl-C to quit");
}
