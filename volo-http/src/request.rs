use hyper::body::Incoming;

use crate::body::Body;

pub type Request<B = Incoming> = http::Request<B>;

pub type ClientRequest<B = Body> = http::Request<B>;
pub type ServerRequest<B = Incoming> = http::Request<B>;
