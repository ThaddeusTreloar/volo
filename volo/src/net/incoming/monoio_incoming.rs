use std::{
    fmt,
    future::Future,
    io,
    task::{Context, Poll},
};

use super::super::{conn::Conn, Address};

use futures::Stream;
use pin_project::pin_project;

use monoio::net::TcpListener;
#[cfg(target_family = "unix")]
use monoio::net::UnixListener;

#[pin_project(project = IncomingProj)]
#[derive(Debug)]
pub enum DefaultIncoming {
    Tcp(#[pin] TcpListener),
    #[cfg(target_family = "unix")]
    Unix(#[pin] UnixListener),
}

#[cfg(target_family = "unix")]
impl From<UnixListener> for DefaultIncoming {
    fn from(l: UnixListener) -> Self {
        DefaultIncoming::Unix(l)
    }
}

#[cfg(feature = "monoio")]
impl From<TcpListener> for DefaultIncoming {
    fn from(l: TcpListener) -> Self {
        DefaultIncoming::Tcp(l)
    }
}

pub trait Incoming: fmt::Debug + 'static {
    fn accept(&mut self) -> impl Future<Output = io::Result<Option<Conn>>>;
}

impl Stream for DefaultIncoming {
    type Item = io::Result<Conn>;
    
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use monoio::io::stream::Stream;
        match self.project() {
            IncomingProj::Tcp(s) => s.next().poll_unpin(cx).map_ok(Conn::from),
            #[cfg(target_family = "unix")]
            IncomingProj::Unix(s) => s.next().poll_unpin(cx).map_ok(Conn::from),
        }
    }
}