use sqlite_types::{
    checksum, Wal, WalFrame, WalFrameHeader, WalHeader, MAGIC_NUMBER_2, SUPPORTED_FILE_FORMAT,
};

type BoxError = Box<dyn std::error::Error>;

pub fn encode(wal: Wal) -> Result<Vec<u8>, BoxError> {
    let mut buff = Vec::new();

    let (checksum_1, checksum_2) = wal.header.checksum();

    write_wal_header(&mut buff, &wal.header, checksum_1, checksum_2)
        .map_err(|err| format!("failed to encode header: {}", err))?;

    let mut checksum_1 = checksum_1;
    let mut checksum_2 = checksum_2;

    for frame in &wal.frames {
        (checksum_1, checksum_2) = frame.header.checksum(checksum_1, checksum_2);
        (checksum_1, checksum_2) = checksum_bytes(frame.data, checksum_1, checksum_2);

        write_wal_frame(&mut buff, frame, checksum_1, checksum_2).map_err(|err| {
            format!(
                "failed to write WAL frame #{}: {}",
                frame.header.page_number, err
            )
        })?;
    }

    Ok(buff)
}

fn write_u32(writer: &mut Vec<u8>, value: u32) {
    writer.extend(value.to_be_bytes());
}

pub fn write_wal_header(
    writer: &mut Vec<u8>,
    header: &WalHeader,
    checksum_1: u32,
    checksum_2: u32,
) -> Result<(), BoxError> {
    // MAGIC_NUMBER_2 uses big-endian, which we'll assume for now.
    write_u32(writer, MAGIC_NUMBER_2);
    write_u32(writer, header.file_format);
    write_u32(writer, header.page_size);
    write_u32(writer, header.checkpoint_seq);
    write_u32(writer, header.salt_1);
    write_u32(writer, header.salt_2);

    write_u32(writer, checksum_1);
    write_u32(writer, checksum_2);
    Ok(())
}

pub fn write_wal_frame(
    writer: &mut Vec<u8>,
    frame: &WalFrame,
    checksum_1: u32,
    checksum_2: u32,
) -> Result<(), BoxError> {
    write_wal_frame_header(writer, &frame.header, checksum_1, checksum_2)?;
    writer.extend(frame.data);
    Ok(())
}

pub fn write_wal_frame_header(
    writer: &mut Vec<u8>,
    header: &WalFrameHeader,
    checksum_1: u32,
    checksum_2: u32,
) -> Result<(), BoxError> {
    write_u32(writer, header.page_number);
    write_u32(writer, header.db_size_after_commit);
    write_u32(writer, header.salt_1);
    write_u32(writer, header.salt_2);
    write_u32(writer, checksum_1);
    write_u32(writer, checksum_2);
    Ok(())
}

fn checksum_bytes(bytes: &[u8], checksum_1: u32, checksum_2: u32) -> (u32, u32) {
    let mut out = Vec::with_capacity(bytes.len() / 4);

    for i in (0..bytes.len()).step_by(4) {
        let v = u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        out.push(v);
    }

    checksum(&out, Some(checksum_1), Some(checksum_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_checksum_wal_frame() {
        let header = WalHeader {
            magic_number: MAGIC_NUMBER_2,
            file_format: SUPPORTED_FILE_FORMAT,
            page_size: 4096,
            checkpoint_seq: 0,
            salt_1: 1,
            salt_2: 2,
            checksum_1: 3,
            checksum_2: 4,
        };
        let (s0, s1) = header.checksum();
        println!("s0 {} s1 {}", s0, s1);
        panic!()
    }
}
