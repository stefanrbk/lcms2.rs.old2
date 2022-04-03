mod date_time_number;
mod encoded_xyz_number;
mod icc_header;
mod profile;
mod profile_id;
mod signature;

pub use date_time_number::DateTimeNumber;
pub use encoded_xyz_number::EncodedXYZNumber;
pub use icc_header::ICCHeader;
pub use profile::Profile;
pub use profile_id::ProfileID;
pub use signature::Signature;

pub mod signatures;

pub const MAX_TABLE_TAG: usize = 100;
