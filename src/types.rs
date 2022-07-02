macro_rules! byte_conversion {
    ($type:ty, $size:expr) => {
        paste::paste! {
            union [<$type Bytes>] {
                pub value: $type,
                pub bytes: [u8; $size],
            }
            impl $type {
                pub(crate) fn to_bytes(&self) -> [u8; $size] {
                    unsafe{
                        [<$type Bytes>] { value: *self }.bytes
                    }
                }
            
                pub(crate) fn from_bytes(ptr: &[u8]) -> Self {
                    let len = ptr.len();
                    let mut new_ptr = [0u8; $size];
            
                    if len < $size {
                        new_ptr[0..len].copy_from_slice(ptr);
                    } else {
                        new_ptr.copy_from_slice(&ptr[0..$size]);
                    }
            
                    unsafe {
                        [<$type Bytes>] { bytes: new_ptr }.value
                    }
                }
            }
        }
    };
}

mod cie_jch;
mod cie_lab;
mod cie_lch;
mod cie_xyy;
mod cie_xyz;
mod curve_segment;
mod date_time_number;
mod encoded_xyz_number;
mod icc_header;
mod mlu;
mod named_color_list;
mod pipeline;
mod profile;
mod profile_id;
mod seq;
mod signature;
mod tag_entry;
mod tone_curve;

pub use cie_jch::CIEJCh;
pub use cie_lab::CIELab;
pub use cie_lch::CIELCh;
pub use cie_xyy::CIExyY;
pub use cie_xyy::CIExyYTriple;
pub use cie_xyz::CIEXYZ;
pub use curve_segment::CurveSegment;
pub use date_time_number::DateTimeNumber;
pub use encoded_xyz_number::EncodedXYZNumber;
pub use icc_header::ICCHeader;
pub use mlu::Mlu;
pub use mlu::MluEntry;
pub use named_color_list::NamedColor;
pub use named_color_list::NamedColorList;
pub use pipeline::Pipeline;
pub use pipeline::PipelineEvalFn;
pub use pipeline::Stage;
pub use pipeline::StageEvalFn;
pub use profile::Profile;
pub use profile_id::ProfileID;
pub use seq::Sequence;
pub use seq::SequenceDescriptor;
pub use signature::Signature;
pub use tag_entry::TagEntry;
pub use tone_curve::ToneCurve;

#[allow(missing_docs)]
pub mod signatures;

pub const MAX_TABLE_TAG: usize = 100;
pub const MAX_CHANNELS: usize = 16;
