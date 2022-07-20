use std::{
    any::Any,
    io::{ErrorKind, Result},
};

use crate::{
    io::IOHandler,
    math::{f64_to_u8f8, u8f8_to_f64},
    state::Context,
    types::{signatures::tag_type, Signature, ToneCurve},
};
pub fn read(
    context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let count = io.read_u32()?;

    match count {
        // Linear
        0 => {
            let single_gamma = 1.0;
            if let Some(new_gamma) = ToneCurve::build_parametric(context, 1, &[single_gamma]) {
                return Ok((1, Box::new(new_gamma)));
            }
            return Err(ErrorKind::InvalidData.into());
        }
        // Specified as the exponent of gamma function
        1 => {
            let single_gamma_fixed = io.read_u16()?;
            let single_gamma = u8f8_to_f64(single_gamma_fixed);

            if let Some(new_gamma) = ToneCurve::build_parametric(context, 1, &[single_gamma]) {
                return Ok((1, Box::new(new_gamma)));
            }
            return Err(ErrorKind::InvalidData.into());
        }
        // Curve
        _ => {
            // This is to prevent bad guys from doing bad things
            if count > 0x7FFF {
                return Err(ErrorKind::InvalidData.into());
            }
            if let Some(mut new_gamma) =
                ToneCurve::build_tabulated_16(context, count as usize, &[0u16; 0])
            {
                let mut buf = vec![0u16; count as usize];
                io.read_u16_array(buf.as_mut_slice())?;
                new_gamma.table16 = buf.into_boxed_slice();
            }
            return Err(ErrorKind::InvalidData.into());
        }
    }
}
/// ptr MUST be &Box\<ToneCurve\>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let ptr = ptr.downcast::<ToneCurve>().unwrap();

    if ptr.segments.len() == 1 && ptr.segments[0].segment.r#type == 1 {
        // Single gamma, preserve number
        let single_gamma_fixed = f64_to_u8f8(ptr.segments[0].segment.params[0]);

        io.write_u32(1)?;
        io.write_u16(single_gamma_fixed)?;
        return Ok(());
    }
    io.write_u32(ptr.table16.len() as u32)?;
    io.write_u16_array(&ptr.table16)
}

/// data MUST be &Box<ToneCurve>
pub fn decide(version: f64, data: &Box<dyn Any>) -> Signature {
    let curve = data.downcast_ref::<ToneCurve>().unwrap();
    if version < 4.0 {
        return tag_type::CURVE;
    }
    // Only 1-segment curves can be saved as parametric
    if curve.segments.len() != 1 {
        return tag_type::CURVE;
    }
    // Only non-inverted curves
    if curve.segments[0].segment.r#type < 0 {
        return tag_type::CURVE;
    }
    // Only ICC parametric curves
    if curve.segments[0].segment.r#type > 5 {
        return tag_type::CURVE;
    }
    tag_type::PARAMETRIC_CURVE
}
