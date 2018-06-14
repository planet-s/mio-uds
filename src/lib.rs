//! MIO bindings for Unix Domain Sockets

#![cfg(any(unix, target_os = "redox"))]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/mio-uds/0.6")]

extern crate iovec;
#[cfg(not(target_os = "redox"))]
extern crate libc;
extern crate mio;

#[cfg(not(target_os = "redox"))]
use std::io;

#[cfg(not(target_os = "redox"))]
mod datagram;
mod listener;
#[cfg(not(target_os = "redox"))]
mod socket;
mod stream;

#[cfg(not(target_os = "redox"))]
pub use datagram::UnixDatagram;
pub use listener::UnixListener;
pub use stream::UnixStream;

#[cfg(not(target_os = "redox"))]
fn cvt(i: libc::c_int) -> io::Result<libc::c_int> {
    if i == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}
