use std::fs::File;
use std::io::Error;
use std::mem;
use std::os::raw::{c_int, c_ushort};
use std::os::unix::io::AsRawFd;

#[repr(C)]
struct Winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

const TIOCGWINSZ: c_int = 0x5413;

extern "C" {
    fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
}

pub fn get_terminal_size() -> Result<(u16, u16), Error> {
    let stdout = File::open("/dev/tty")?;
    let fd = stdout.as_raw_fd();
    let mut ws: Winsize = unsafe { mem::zeroed() };
    let result = unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) };
    if result == -1 {
        return Err(Error::last_os_error());
    }
    Ok((ws.ws_col, ws.ws_row))
}
