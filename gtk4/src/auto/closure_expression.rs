// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Expression;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkClosureExpression")]
    pub struct ClosureExpression(Shared<ffi::GtkClosureExpression>);

    match fn {
        ref => |ptr| ffi::gtk_expression_ref(ptr as *mut ffi::GtkExpression),
        unref => |ptr| ffi::gtk_expression_unref(ptr as *mut ffi::GtkExpression),
    }
}
impl glib::StaticType for ClosureExpression {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gtk_closure_expression_get_type()) }
    }
}

impl fmt::Display for ClosureExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ClosureExpression")
    }
}
