use crate::types::{Transform, Signature};

pub enum FormatterDirection {
    Input,
    Output,
}

pub enum FormatterPrecision {
    U16,
    Float
}

pub enum Formatter {
    Fmt16(fn(cargo: &Transform, values: &[u16], buffer: &mut [u8], stride: u32) -> Box<[u8]>),
    FmtFloat(fn(cargo: &Transform, values: &[f32], buffer: &mut [u8], stride: u32) -> Box<[u8]>),
}

pub type FormatterFactory = fn(r#type: Signature, dir: FormatterDirection, flags: FormatterPrecision) -> Formatter;
