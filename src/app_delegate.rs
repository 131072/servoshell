use cocoa::appkit::*;
use cocoa::base::*;
use cocoa::foundation::*;
use std::sync::{Once, ONCE_INIT};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc_foundation::{INSObject, NSObject};
use std::os::raw::c_void;
use servo::Servo;
static INIT: Once = ONCE_INIT;

pub fn new_app_delegate() -> id {
    unsafe {
        INIT.call_once(|| {
            extern fn flush_gl_context(this: &Object, _sel: Sel) {
                unsafe {
                    let context: &id = {
                        let ivar: *mut c_void = *this.get_ivar("context");
                        &*(ivar as *mut id)
                    };
                    msg_send![*context, flushBuffer];
                    let servo: &Servo = {
                        let ivar: *mut c_void = *this.get_ivar("servo");
                        &*(ivar as *mut Servo)
                    };
                    servo.sync();
                }
            }
            let superclass = Class::get("NSObject").unwrap();
            let mut decl = ClassDecl::new("NSMyAppDelegate", superclass).unwrap();
            decl.add_method(sel!(flushGlContext), flush_gl_context as extern fn(&Object, Sel));
            decl.add_ivar::<*mut c_void>("context");
            decl.add_ivar::<*mut c_void>("servo");
            decl.register();
        });
        msg_send![Class::get("NSMyAppDelegate").unwrap(), alloc]
    }
}