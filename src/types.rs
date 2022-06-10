mod date_time_number;
mod encoded_xyz_number;
mod icc_header;
mod profile;
mod profile_id;
mod signature;
mod cie_xyz;
mod tag_entry;
mod transform;
mod pipeline;
mod named_color_list;
mod mlu;
mod seq;

pub use date_time_number::DateTimeNumber;
pub use encoded_xyz_number::EncodedXYZNumber;
pub use icc_header::ICCHeader;
pub use profile::Profile;
pub use profile_id::ProfileID;
pub use signature::Signature;
pub use cie_xyz::CIEXYZ;
pub use tag_entry::TagEntry;
pub use transform::Transform;
pub use transform::TransformFn;
pub use transform::Transform2Fn;
pub use transform::TransformFactory;
pub use transform::Transform2Factory;
pub use transform::Stride;
pub use transform::Cache;
pub use pipeline::Pipeline;
pub use pipeline::PipelineEvalFn;
pub use pipeline::Stage;
pub use pipeline::StageEvalFn;
pub use named_color_list::NamedColorList;
pub use named_color_list::NamedColor;
pub use mlu::Mlu;
pub use mlu::MluEntry;
pub use seq::Sequence;
pub use seq::SequenceDescriptor;


#[allow(missing_docs)]
pub mod signatures;

pub const MAX_TABLE_TAG: usize = 100;
pub const MAX_CHANNELS: usize = 16;
