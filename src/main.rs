extern crate iron;
extern crate getopts;

use std::os;
use std::io::net::ip::Ipv4Addr;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use getopts::{optopt, optflag, getopts, usage};

fn hello_world(_:&mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok, "Hello world!"))
}

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
    Iron::new(hello_world).listen(Ipv4Addr(127, 0, 0, 1), port);
    println!("Running server on port {}", port);
}
