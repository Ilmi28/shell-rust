use std::{ffi::{c_int, c_uint}, io};

const NCCS: usize = 32;

const ICANON:  c_uint = 0x00002;
const ECHO:    c_uint = 0x00008;
const TCSANOW: c_int  = 0x00000;

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_cc: [c_uint; NCCS]
}

unsafe extern "C" {
    fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios);
}

pub struct RawTerminal {
    original: Termios
}

impl RawTerminal {
    pub fn enable_raw_mode() -> io::Result<Self> {
        unsafe {
            let mut original: Termios = std::mem::zeroed();
            tcgetattr(0, &mut original);
            let mut raw = original;
            raw.c_lflag &= !(ICANON | ECHO);
            tcsetattr(0, TCSANOW, &raw); 
            Ok(Self { original })
        }
    }   
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        unsafe {
            tcsetattr(0, 0, &self.original);
        }
    }
}