use std::collections::HashMap;

pub const MAGIC_NUMBER_1: u32 = 0x377f0682;
pub const MAGIC_NUMBER_2: u32 = 0x377f0683;
pub const SUPPORTED_FILE_FORMAT: u32 = 3007000;
pub const MAGIC_STRING: &[u8] = b"SQLite format 3\0";
pub const SQLITE_3_37_2_VERSION: u32 = 3038002;

pub type Page = Vec<u8>;

#[derive(Debug)]
pub struct Db {
    pub header: DbHeader,
    pub pages: HashMap<u32, Page>,
}

#[derive(Debug, Clone)]
pub struct DbHeader {
    pub page_size: u16,
    pub file_format_write_version: u8,
    pub file_format_read_version: u8,
    pub max_embedded_payload_frac: u8,
    pub min_embedded_payload_frac: u8,
    pub leaf_payload_frac: u8,
    pub file_change_counter: u32,
    pub db_size: u32,
    pub page_num_first_freelist: u32,
    pub page_count_freelist: u32,
    pub schema_cookie: u32,
    pub schema_format_number: u32,
    pub default_page_cache_size: u32,
    pub page_num_largest_root_btree: u32,
    pub text_encoding: u32,
    pub user_version: u32,
    pub vaccum_mode: u32,
    pub app_id: u32,
    pub version_valid_for: u32,
    pub sqlite_version: u32,
}

#[derive(Debug, Clone)]
pub struct Wal {
    pub header: WalHeader,
    pub frames: Vec<WalFrame>,
}

#[derive(Debug, Clone)]
pub struct WalHeader {
    pub magic_number: u32,
    pub file_format: u32,
    pub page_size: u32,
    pub checkpoint_seq: u32,
    pub salt_1: u32,
    pub salt_2: u32,
    pub checksum_1: u32,
    pub checksum_2: u32,
}

#[derive(Debug, Clone)]
pub struct WalFrameHeader {
    pub page_number: u32,
    pub db_size_after_commit: u32,
    pub salt_1: u32,
    pub salt_2: u32,
    pub checksum_1: u32,
    pub checksum_2: u32,
}

#[derive(Debug, Clone)]
pub struct WalFrame {
    pub header: WalFrameHeader,
    pub data: Vec<u8>,
}

impl Wal {
    pub fn rewrite_salt_1(mut self, value: u32) -> Self {
        self.header.salt_1 = value;
        for frame in &mut self.frames {
            frame.header.salt_1 = value;
        }

        self
    }

    pub fn rewrite_salt_2(mut self, value: u32) -> Self {
        self.header.salt_2 = value;
        for frame in &mut self.frames {
            frame.header.salt_2 = value;
        }

        self
    }
}

impl WalHeader {
    pub fn checksum(&self) -> (u32, u32) {
        let values = [
            // MAGIC_NUMBER_2 uses big-endian, which we'll assume for now.
            MAGIC_NUMBER_2,
            self.file_format,
            self.page_size,
            self.checkpoint_seq,
            self.salt_1,
            self.salt_2,
        ];
        checksum(&values, None, None)
    }
}

impl WalFrameHeader {
    pub fn checksum(&self, checksum_1: u32, checksum_2: u32) -> (u32, u32) {
        let values = [self.page_number, self.db_size_after_commit];
        checksum(&values, Some(checksum_1), Some(checksum_2))
    }
}

pub fn checksum(input: &[u32], s1: Option<u32>, s2: Option<u32>) -> (u32, u32) {
    let mut s1 = if let Some(s1) = s1 { s1 } else { 0u32 };
    let mut s2 = if let Some(s2) = s2 { s2 } else { 0u32 };

    if input.len() * 4 % 8 != 0 {
        panic!("input must be a multiple of 8 bytes, given {}", input.len())
    }

    let mut i = 0;
    loop {
        s1 = s1.wrapping_add(input[i].wrapping_add(s2));
        s2 = s2.wrapping_add(input[i + 1].wrapping_add(s1));

        i += 2;
        if i >= input.len() {
            break;
        }
    }

    (s1, s2)
}
