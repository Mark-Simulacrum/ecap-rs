use std::borrow::Cow;

pub struct Delay {
    /// Completed work-fraction in (0, 1) range or `None` if unknown.
    pub progress: Option<f64>,

    /// User-friendly state description, if available
    pub description: Option<Cow<'static, str>>,
}
