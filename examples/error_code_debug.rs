use xcb::{x, Xid};

// A simple example to demonstrate that Protocol error code names are printed
// when formatting with Debug, and when the `xlib_xcb` feature is enabled.

fn main() -> xcb::Result<()> {
    let (conn, _) = xcb::Connection::connect(None).unwrap();

    // To trigger a protocol error, we try to map a window that doesn't exist.
    // This will return an `Err(Protocol(X(Window(ValueError { .. }))))`
    // Check the console and you'll see the `error_code_name` field there too!
    conn.send_and_check_request(&x::MapWindow { window: x::Window::none() })?;

    Ok(())
}
