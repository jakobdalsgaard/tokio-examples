extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::stream::{Stream, MergedItem};

fn main() {
  println!("Starting up, will listen on ports 8000 and 8010");
  let addr1 = "127.0.0.1:8000".parse().unwrap();
  let addr2 = "127.0.0.1:8010".parse().unwrap();
  let mut core = Core::new().unwrap();
  let handle = core.handle();
  let socket1 = TcpListener::bind(&addr1, &handle).unwrap();
  let socket2 = TcpListener::bind(&addr2, &handle).unwrap();

  let merged = socket1.incoming().merge(socket2.incoming());
  let srv = merged.for_each(move |m| {
     match m {
       MergedItem::First((_, addr)) => {
         println!("New connection on 8000, from {}", addr);
         Ok(())
       },
       MergedItem::Second((_, addr)) => {
         println!("New connection on 8010, from {}", addr);
         Ok(())
       },
       MergedItem::Both((_, addr1), (_, addr2)) => {
         println!("New connections on both 8000 and 8010, from {} and {}", addr1, addr2);
         Ok(())
       }
     }
   });
  core.run(srv).unwrap();
}

