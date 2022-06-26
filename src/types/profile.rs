use std::{
    fs::File,
    io::{self, Error, ErrorKind},
    path::Path,
};

use chrono::Utc;

use crate::{
    io::{AccessMode, IOHandler},
    state::{Context, ErrorCode},
};

use super::{
    icc_header::ICCHeaderConverter, signatures, tag_entry::TagEntryConverter, ProfileID, Signature,
    MAX_TABLE_TAG,
};

#[derive(Debug)]
pub struct Profile {
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

    pub fn new() -> Self {
        Self {
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
    pub fn open_from_file<P: AsRef<Path>>(
        context: &mut Context,
        filename: P,
        mode: AccessMode,
    ) -> io::Result<Box<Profile>> {
        let mut profile = Box::new(Self::new());

        if let AccessMode::Read = mode {
            profile.io = Some(Box::new(File::open(filename)?));
        } else {
            profile.io = Some(Box::new(File::create(filename)?));
            profile.is_write = true;
        };

        profile.read_header(context)?;

        Ok(profile)
    }

    fn read_header(&mut self, context: &mut Context) -> io::Result<()> {
        let err = Err(Error::from(ErrorKind::InvalidData));
        let io = match self.io {
            Some(ref mut b) => b.as_mut(),
            None => return err,
        };

        let mut buf = [0u8; 128];
        io.read(&mut buf)?;

        let header = ICCHeaderConverter::from_bytes(buf);

        // Validate file as an ICC profile
        if header.magic != signatures::MAGIC_NUMBER {
            context.signal_error(
                ErrorCode::BadSignature,
                "not an ICC profile, invalid signature".to_string(),
            );
            return err;
        }

        self.device_class = header.device_class;
        self.color_space = header.color_space;
        self.pcs = header.pcs;

        self.rendering_intent = header.rendering_intent.into();
        self.flags = header.flags;
        self.manufacturer = header.manufacturer.into();
        self.model = header.model.into();
        self.creator = header.creator.into();

        self.attributes = header.attributes;
        self.version = header.version;

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
            context
                .signal_error(ErrorCode::Range, format!("Too many tags {}", tag_count));
            return err;
        }

        for _ in 0..tag_count {
            let mut buf = [0u8; 12];
            io.read(&mut buf)?;

            let tag = TagEntryConverter::from_bytes(buf);

            // Perform some sanity check. Offset + size should fall inside file.
            if tag.offset + tag.size > header_size as u32 || tag.offset + tag.size < tag.offset {
                continue;
            }

            self.tag_names[self.tag_count] = tag.sig;
            self.tag_offsets[self.tag_count] = tag.offset as usize;
            self.tag_sizes[self.tag_count] = tag.size as usize;

            // Search for links
            for j in 0..self.tag_count {
                if self.tag_offsets[j] == tag.offset as usize
                    && self.tag_sizes[j] == tag.size as usize
                {
                    self.tag_linked[j] = Some(self.tag_names[j]);
                }
            }

            self.tag_count += 1;
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

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.created == other.created
            && self.version == other.version
            && self.device_class == other.device_class
            && self.color_space == other.color_space
            && self.pcs == other.pcs
            && self.rendering_intent == other.rendering_intent
            && self.flags == other.flags
            && self.manufacturer == other.manufacturer
            && self.model == other.model
            && self.attributes == other.attributes
            && self.creator == other.creator
            && self.profile_id == other.profile_id
            && self.tag_count == other.tag_count
            && self.tag_names == other.tag_names
            && self.tag_linked == other.tag_linked
            && self.tag_sizes == other.tag_sizes
            && self.tag_offsets == other.tag_offsets
            && self.tag_save_as_raw == other.tag_save_as_raw
            && self.is_write == other.is_write
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use super::*;
    use std::io;

    use crate::{state::Context, testing::get_test_resource_path};

    #[test]
    fn test_load_file() -> io::Result<()> {
        let mut context = Context::new(None);
        Profile::open_from_file(
            &mut context,
            get_test_resource_path("sRGB_v4_ICC_preference.icc"),
            AccessMode::Read,
        )?;

        Ok(())
    }

    #[test]
    fn test_file_loads_with_proper_data_and_endianness() -> io::Result<()> {
        let mut context = Context::new(None);
        let mut expected_names = [Signature::default(); MAX_TABLE_TAG];
        expected_names[..9].copy_from_slice(&[
            Signature::new(b"desc"),
            Signature::new(b"A2B0"),
            Signature::new(b"A2B1"),
            Signature::new(b"B2A0"),
            Signature::new(b"B2A1"),
            Signature::new(b"rig0"),
            Signature::new(b"wtpt"),
            Signature::new(b"cprt"),
            Signature::new(b"chad"),
        ]);
        let mut expected_sizes = [0usize; MAX_TABLE_TAG];
        expected_sizes[..9].copy_from_slice(&[118, 29712, 436, 29748, 508, 12, 20, 118, 44]);
        let mut expected_offsets = [0usize; MAX_TABLE_TAG];
        expected_offsets[..9]
            .copy_from_slice(&[240, 360, 30072, 30508, 60256, 60764, 60776, 60796, 60916]);
        let expected = Box::new(Profile {
            io: None,
            created: NaiveDate::from_ymd(2007, 07, 25).and_hms(0, 5, 37),
            version: 0x04200000,
            device_class: Signature::new(b"spac"),
            color_space: Signature::new(b"RGB "),
            pcs: Signature::new(b"Lab "),
            rendering_intent: 0,
            flags: 0,
            manufacturer: 0,
            model: 0,
            attributes: 0,
            creator: 0,
            profile_id: ProfileID {
                id8: [
                    0x34, 0x56, 0x2a, 0xbf, 0x99, 0x4c, 0xcd, 0x06, 0x6d, 0x2c, 0x57, 0x21, 0xd0,
                    0xd6, 0x8c, 0x5d,
                ],
            },
            tag_count: 9,
            tag_names: expected_names,
            tag_linked: [None; MAX_TABLE_TAG],
            tag_sizes: expected_sizes,
            tag_offsets: expected_offsets,
            tag_save_as_raw: [false; MAX_TABLE_TAG],
            is_write: false,
        });
        let actual = Profile::open_from_file(
            &mut context,
            get_test_resource_path("sRGB_v4_ICC_preference.icc"),
            AccessMode::Read,
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
