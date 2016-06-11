use std::prelude::v1::*;
use std::fmt::{self, Write};
use std::ptr;

use libctru::console::*;
use libc;

use gfx::Screen;

pub struct Console {
    context: PrintConsole,
}

impl Console {
    pub fn init(screen: Screen) -> Self {
        let ret = unsafe { *(consoleInit(screen.into(), ptr::null_mut())) };
        Console { context: ret }
    }

    pub fn set_window(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { consoleSetWindow(&mut self.context, x, y, width, height) }
    } 

    pub fn clear(&mut self) {
        unsafe { consoleClear() }
    }
}

impl Default for Console {
    fn default() -> Self {
        let ret = unsafe { *(consoleInit(Screen::Top.into(), ptr::null_mut())) };
        Console { context: ret }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { consoleSelect(&mut self.context); }
        let ret = unsafe { libc::write(libc::STDOUT_FILENO, s.as_ptr() as *const _, s.len()) };
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}
