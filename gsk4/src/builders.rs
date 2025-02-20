// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Builder pattern types.

pub use crate::auto::builders::*;
pub use crate::color_stop::ColorStopBuilder;
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
pub use crate::stroke::StrokeBuilder;
pub use crate::ShaderArgsBuilder;
