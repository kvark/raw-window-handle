use core::ptr;
use libc::c_void;

impl crate::RawWindowHandle {
    pub fn new(handle: IOSHandle) -> Self {
        Self {
            handle: RawWindowHandle { handle },
        }
    }

    pub fn ios_handle(&self) -> IOSHandle {
        self.handle.handle
    }
}

pub(crate) struct RawWindowHandle {
    handle: IOSHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IOSHandle {
    pub ui_window: *mut c_void,
    pub ui_view: *mut c_void,
    pub ui_view_controller: *mut c_void,
    _non_exhaustive: (),
}

impl IOSHandle {
    pub fn empty() -> Self {
        Self {
            ui_window: ptr::null_mut(),
            ui_view: ptr::null_mut(),
            ui_view_controller: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
