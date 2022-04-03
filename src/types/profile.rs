use std::{fs::File, io::{Read, self}, io::Write};

use chrono::Utc;

use crate::state::Context;

use super::{Signature, ProfileID, MAX_TABLE_TAG};

pub struct Profile {
    context: Option<Context>,
    reader: Option<Box<dyn Read>>,
    writer: Option<Box<dyn Write>>,
    created: chrono::NaiveDateTime,
    version: u32,
    device_class: Signature,
    color_space: Signature,
    pcs: Signature,
    rendering_intent: u32,
    flags: u32,
    manufacturer: u32,
    model: u32,
    attributes: u64,
    creator: u32,
    profile_id: ProfileID,
    tag_count: u32,
    tag_names: [Signature; MAX_TABLE_TAG],
    tag_linked: [Option<Signature>; MAX_TABLE_TAG],
    tag_sizes: [usize; MAX_TABLE_TAG],
    tag_offsets: [usize; MAX_TABLE_TAG],
    tag_save_as_raw: [bool; MAX_TABLE_TAG],
    is_write: bool
}

impl Profile {
    pub fn create_placeholder(context: Context) -> Box<Self> {
        Box::new(Self {
            context: Some(context),
            tag_count: 0,
            version: 0x02100000,
            created: Utc::now().naive_utc(),
            // mutex stuff for the future goes here
            device_class: Signature::default(),
            color_space: Signature::default(),
            pcs: Signature::default(),
            rendering_intent: 0,
            flags: 0,
            manufacturer: 0,
            model: 0,
            attributes: 0,
            creator: 0,
            profile_id: ProfileID { id16: [0u16; 8] },
            tag_names: [Signature::default(); MAX_TABLE_TAG],
            tag_linked: [None; MAX_TABLE_TAG],
            tag_sizes: [0; MAX_TABLE_TAG],
            tag_offsets: [0; MAX_TABLE_TAG],
            tag_save_as_raw: [false; MAX_TABLE_TAG],
            is_write: false,
            reader: None,
            writer: None,
        })
    }

    pub fn open_from_file(context: Context, filename: String, mode: FileMode) -> io::Result<()> {
        let mut profile = Self::create_placeholder(context);

        match mode {
            FileMode::Read => {
                profile.reader = Some(Box::new(File::open(filename)?));
            },
            FileMode::Write => {
                profile.writer = Some(Box::new(File::create(filename)?));
                profile.is_write = true;
            }
        };

        Ok(())
    }
}

pub enum FileMode {
    Read,
    Write
}
