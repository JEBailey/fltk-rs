pub use crate::enums::*;
use crate::prelude::*;
use fltk_sys::fl::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Runs the event loop
fn run() -> Result<(), FltkError> {
    unsafe {
        match Fl_run() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Locks the main UI thread
fn lock() -> Result<(), FltkError> {
    unsafe {
        match Fl_lock() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToLock)),
        }
    }
}

/// Set the app scheme
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AppScheme {
    /// Base fltk scheming
    Base,
    /// inspired by the Aqua user interface on Mac OS X
    Plastic,
    /// inspired by the GTK+ theme
    Gtk,
    /// inspired by the Clearlooks Glossy scheme
    Gleam,
}

/// sets the scheme of the application
fn set_scheme(scheme: AppScheme) {
    let name_str = match scheme {
        AppScheme::Base => "base",
        AppScheme::Gtk => "gtk+",
        AppScheme::Gleam => "gleam",
        AppScheme::Plastic => "plastic",
    };
    let name_str= CString::new(name_str).unwrap();
    unsafe {
        Fl_set_scheme(name_str.into_raw() as *const raw::c_char)
    }
}

/// Unlocks the main UI thread
// #[allow(dead_code)]
// fn unlock() {
//     unsafe {
//         Fl_unlock();
//     }
// }

// pub fn awake<'a>(cb: Box<dyn FnMut() + 'a>) {
//     unsafe {
//         unsafe extern "C" fn shim<'a>(data: *mut raw::c_void) {
//             let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
//             let f: &mut (dyn FnMut() + 'a) = &mut **a;
//             f();
//         }
//         let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
//         let data: *mut raw::c_void = mem::transmute(a);
//         let callback: Fl_Awake_Handler = Some(shim);
//         Fl_awake(callback, data);
//     }
// }

/// Basic Application struct, used to instatiate, set the scheme and run the event loop
#[derive(Debug, Copy, Clone)]
pub struct App {}

impl App {
    /// Instantiates an App type
    pub fn default() -> App {
        set_fonts("*");
        App {}
    }
    
    /// Sets the scheme of the application
    pub fn set_scheme(self, scheme: AppScheme) -> App {
        set_scheme(scheme);
        self
    }
    
    /// Runs the event loop
    pub fn run(&self) -> Result<(), FltkError> {
        lock()?;
        return run();
    }
    // pub fn awake<'a>(&'a self, cb: Box<dyn FnMut() + 'a>) {
    //     unsafe {
    //         unsafe extern "C" fn shim<'a>(data: *mut raw::c_void) {
    //             let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
    //             let f: &mut (dyn FnMut() + 'a) = &mut **a;
    //             f();
    //         }
    //         let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
    //         let data: *mut raw::c_void = mem::transmute(a);
    //         let callback: Fl_Awake_Handler = Some(shim);
    //         Fl_awake(callback, data);
    //     }
    // }
}

/// Returns the latest captured event
pub fn event() -> Event {
    unsafe {
        let x = Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}

/// Returns the presed key
pub fn event_key() -> Key {
    unsafe {
        let x = Fl_event_key();
        mem::transmute(x)
    }
}

/// Returns a textual representation of the latest event
pub fn event_text() -> String {
    unsafe {
          CStr::from_ptr(Fl_event_text() as *mut raw::c_char)
            .to_string_lossy().to_string()
    }
}

/// Returns the captured button event
pub fn event_button() -> i32 {
    unsafe { Fl_event_button() }
}

/// Returns the number of clicks
pub fn event_clicks() -> bool {
    unsafe {
        match Fl_event_clicks() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the x and y coordinates of the captured event
pub fn event_coords() -> (i32, i32) {
    unsafe { (Fl_event_dx(), Fl_event_dy()) }
}

/// Determines whether an event was a click
pub fn event_is_click() -> bool {
    unsafe {
        match Fl_event_is_click() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the duration of an event
pub fn event_length() -> u32 {
    unsafe { Fl_event_length() as u32 }
}

/// Returns the state of the event
pub fn event_state() -> Shortcut {
    unsafe { mem::transmute(Fl_event_state()) }
}

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe {
        (
            (Fl_screen_w() as f64 / 0.96).into(),
            (Fl_screen_h() as f64 / 0.96).into(),
        )
    }
}

/// Used for widgets implementing the InputTrait, pastes content from the clipboard
pub fn paste<T>(widget: T)
where
    T: WidgetTrait + InputTrait,
{
    unsafe {
        Fl_paste(widget.as_widget_ptr() as *mut raw::c_void, 1);
    }
}

/// Sets the callback of a widget
pub fn set_callback<'a, W>(widget: &'a W, cb: Box<dyn FnMut() + 'a>)
where
    W: WidgetTrait,
{
    unsafe {
        unsafe extern "C" fn shim<'a>(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
            let f: &mut (dyn FnMut() + 'a) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}

fn set_fonts(name: &str) -> u8 {
    let name = CString::new(name).unwrap();
    unsafe {
        Fl_set_fonts(name.into_raw() as *mut raw::c_char) as u8
    }
} 

pub fn get_font_count() -> u8 {
    set_fonts("*")
}

pub fn get_font_name(idx: u8) -> String {
    unsafe {
        CStr::from_ptr(Fl_get_font(idx as i32) as *mut raw::c_char).to_string_lossy().to_string()
    }
}

pub fn get_font_names() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let cnt = get_font_count();
    for i in 0..cnt {
        vec.push(get_font_name(i));
    }
    vec
}

pub fn get_font_index(name: &str) -> Option<u8> {
    let cnt = set_fonts("*");
    let mut ret: Option<u8> = None;
    for i in 0..cnt {
        if name == get_font_name(i) {
            ret = Some(i);
            break;
        } 
    }
    ret
}