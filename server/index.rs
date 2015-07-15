extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::{status, Iron, Request, Response, IronResult, IronError};
use iron::mime::Mime;
use staticfile::Static;
use mount::Mount;

use std::process::{Command, Output};
use std::error::Error;

#[derive(Debug)]
struct ServerError(String);

impl ::std::fmt::Display for ServerError {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        &*self.0
    }
}

impl ::std::convert::From<&'static str> for ServerError {
    fn from(s: &'static str) -> ServerError { ServerError(s.to_owned()) }
}

fn serve(req: &mut Request) -> IronResult<Response> {
    match req.url.query {
        Some(ref param) if param.starts_with("module=") => {
            let module = &param[7..];
            match Command::new(format!("modules/shell_files/{}.sh", module)).output() {
                Ok(Output { stdout, .. }) => Ok(Response::with((status::Ok, stdout, "application/json".parse::<Mime>().unwrap()))),
                Err(err) => Err(IronError::new(err, status::InternalServerError)),
            }
        },
        Some(_) => Err(IronError::new::<ServerError, _>("module parameter required".into(), status::BadRequest)),
        None => Err(IronError::new::<ServerError, _>("object not found".into(), status::NotFound)),
    }
}

fn main() {
    let mut root = Mount::new();
    root.mount("/", Static::new("../"))
        .mount("/server", serve);

    Iron::new(root).http("localhost:8081").unwrap();
}
