# tokio-examples
My playground for Tokio examples and snippets

Currently there are two examples in this repository:

* merge-tcplisteners : an example on how to merge the streams from two Tokio Core
   TCP Listeners and have them served using one single event loop.
* merge-streams : an example on how to merge the stream from a Tokio Core TCP
   Listener with the stream of Tokio Signal USR1 signals and have them served
   using one single event loop.


