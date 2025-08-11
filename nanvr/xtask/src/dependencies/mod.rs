pub mod android;
pub mod linux;

pub enum OpenXRLoadersSelection {
    OnlyGeneric,
    OnlyPico,
    All,
}
