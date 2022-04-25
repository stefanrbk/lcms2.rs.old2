use std::{
    fs::File,
    io::{self, Error, ErrorKind},
};

use chrono::Utc;

use crate::{
    io::{AccessMode, IOHandler},
    state::{Context, ErrorCode},
};

use super::{icc_header::ICCHeaderConverter, signatures, ProfileID, Signature, MAX_TABLE_TAG};

pub struct Profile {
    context: Box<Context>,
    io: Option<Box<dyn IOHandler>>,
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
    tag_count: usize,
    tag_names: [Signature; MAX_TABLE_TAG],
    tag_linked: [Option<Signature>; MAX_TABLE_TAG],
    tag_sizes: [usize; MAX_TABLE_TAG],
    tag_offsets: [usize; MAX_TABLE_TAG],
    tag_save_as_raw: [bool; MAX_TABLE_TAG],
    is_write: bool,
}

impl Profile {
    pub fn get_io_handler(&self) -> Option<&Box<dyn IOHandler>> {
        self.io.as_ref()
    }
    pub fn get_context(&self) -> &Box<Context> {
        &self.context
    }
    pub fn get_tag_count(&self) -> usize {
        self.tag_count
    }
    pub fn get_tag_signature(&self, n: usize) -> Signature {
        if n > self.tag_count {
            Signature::default()
        } else {
            self.tag_names[n]
        }
    }

    pub fn new(context: Box<Context>) -> Self {
        Self {
            context,
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
            io: None,
        }
    }
    pub fn open_from_file(
        context: Box<Context>,
        filename: String,
        mode: AccessMode,
    ) -> io::Result<Box<Profile>> {
        let mut profile = Box::new(Self::new(context));

        if let AccessMode::Read = mode {
            profile.io = Some(Box::new(File::open(filename)?));
        } else {
            profile.io = Some(Box::new(File::create(filename)?));
            profile.is_write = true;
        };

        profile.read_header()?;

        Ok(profile)
    }

    fn read_header(&mut self) -> io::Result<()> {
        let err = Err(Error::from(ErrorKind::InvalidData));
        let io = match self.io {
            Some(ref mut b) => b.as_mut(),
            None => return err,
        };

        let mut buf = [0u8; 128];
        io.read(&mut buf)?;

        let header = ICCHeaderConverter::from_bytes(buf);

        if header.magic != signatures::MAGIC_NUMBER {
            self.context.signal_error(
                ErrorCode::BadSignature,
                "not an ICC profile, invalid signature".to_string(),
            );
            return err;
        }

        // Get size as reported in header
        let header_size = header.size as usize;

        // Make sure header_size is lower than profile size
        let reported_size = io.reported_size()?;
        let header_size = if header_size >= reported_size {
            reported_size
        } else {
            header_size
        };

        // Get creation date/time
        self.created = header.date.into();

        // The profile ID are 32 raw bytes
        self.profile_id = header.profile_id;

        // Read tag directory
        let tag_count = io.read_u32()? as usize;
        if tag_count > MAX_TABLE_TAG {
            self.context.signal_error(ErrorCode::Range, format!("Too many tags {}", tag_count));
            return err;
        }

        

        Ok(())
    }

    fn search_one_tag(&self, sig: Signature) -> Option<usize> {
        for i in 0..self.tag_count {
            if sig == self.tag_names[i] {
                return Some(i);
            }
        }
        None
    }
    fn search_tag(&self, mut sig: Signature, follow_links: bool) -> Option<usize> {
        loop {
            let n = self.search_one_tag(sig);
            if let Option::None = n {
                return None;
            }
            if !follow_links {
                return n;
            }

            let n = n.unwrap();
            let linked_sig = self.tag_linked[n];

            match linked_sig {
                Some(value) => sig = value,
                None => return Some(n),
            }
        }
    }
}
