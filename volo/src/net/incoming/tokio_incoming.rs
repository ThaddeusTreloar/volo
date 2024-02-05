    use std::{
        fmt,
        future::Future,
        io,
        task::{Context, Poll},
    };
    
    use super::super::{conn::Conn, Address};
    
    use pin_project::pin_project;
    use tokio::net::TcpListener;
    use tokio_stream::wrappers::TcpListenerStream;
    #[cfg(target_family = "unix")]
    use tokio::net::UnixListener;
    #[cfg(target_family = "unix")]
    use tokio_stream::wrappers::UnixListenerStream;


    #[pin_project(project = IncomingProj)]
    #[derive(Debug)]
    pub enum DefaultIncoming {
        Tcp(#[pin] TcpListenerStream),
        #[cfg(target_family = "unix")]
        Unix(#[pin] UnixListenerStream),
    }

    #[cfg(target_family = "unix")]
    impl From<UnixListener> for DefaultIncoming {
        fn from(l: UnixListener) -> Self {
            DefaultIncoming::Unix(UnixListenerStream::new(l))
        }
    }
    
    impl From<TcpListener> for DefaultIncoming {
        fn from(l: TcpListener) -> Self {
            DefaultIncoming::Tcp(TcpListenerStream::new(l))
        }
    }

    pub trait Incoming: fmt::Debug + Send + 'static {
        fn accept(&mut self) -> impl Future<Output = io::Result<Option<Conn>>> + Send;
    }