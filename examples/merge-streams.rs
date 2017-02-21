extern crate tokio_core;
extern crate futures;
extern crate tokio_signal;

extern crate libc;

use tokio_core::reactor::{Core, Handle};
use tokio_core::io::{IoStream, IoFuture};
use tokio_core::net::TcpListener;
use tokio_signal::unix;
use futures::stream::{Stream, MergedItem};
use futures::Future;
use libc::getpid;


// usr1 signal stream
pub fn sig_usr1(handle: &Handle) -> IoFuture<IoStream<()>> {
    return sig_usr1_imp(handle);

    fn sig_usr1_imp(handle: &Handle) -> IoFuture<IoStream<()>> {
        unix::Signal::new(unix::libc::SIGUSR1, handle).map(|x| {
            x.map(|_| ()).boxed()
        }).boxed()
    }
}

fn main() {
  let pid = unsafe { getpid() };
  println!("Starting up, will listen on port 8000 and usr1 signal, process id is {}", pid);
  let addr = "127.0.0.1:8000".parse().unwrap();
  let mut core = Core::new().unwrap();
  let handle = core.handle();
  let socket = TcpListener::bind(&addr, &handle).unwrap();
  let usr1 = sig_usr1(&handle);
  let usr1_stream = core.run(usr1).unwrap();

  let merged = socket.incoming().merge(usr1_stream);
  let srv = merged.for_each(move |m| {
     match m {
       MergedItem::First((_, addr)) => {
         println!("New connection on 8000, from {}", addr);
         Ok(())
       },
       MergedItem::Second(()) => {
         println!("Signal usr1 received");
         Ok(())
       },
       MergedItem::Both((_, addr), ()) => {
         println!("New connection 8000 from {}, and signal usr1 received", addr);
         Ok(())
       }
     }
   });
  core.run(srv).unwrap();
}

