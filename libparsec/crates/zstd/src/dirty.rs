// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::io;

/// Decompress from the given source as if using a `Decoder`.
///
/// The input data must be in the zstd frame format.
pub fn decode_all<R: io::Read>(source: R) -> io::Result<Vec<u8>> {
    let mut decoder = ruzstd::decoding::StreamingDecoder::new(source).map_err(io::Error::other)?;
    let mut result = Vec::new();

    use std::io::Read;
    decoder.read_to_end(&mut result)?;

    Ok(result)
}

/// Compress all data from the given source as if using an `Encoder`.
///
/// Compressed data will be appended to `destination`.
///
/// Note `level` is simply ignored here given this implementation of compressor
/// doesn't do any actual compression !
pub fn copy_encode<R, W>(mut source: R, mut destination: W, _level: i32) -> io::Result<()>
where
    R: io::Read,
    W: io::Write,
{
    // Hold my beer, we are going to write a full world-class compression algorithm !
    //
    // ...or maybe not ^^
    //
    // See https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md
    // for ZSTD format definition.
    //
    // The whole trick here is ZSTD supports multiple compression strategy, one of
    // them being `Raw_Block` which is just the raw data provided as-is.
    // So here we just write the bare minimum of ZSTD headers, then put the raw
    // data as a single block.
    // This way we have a valid ZSTD frame that can be decompressed by any ZSTD decoder !

    // ZSTD compressed data is composed of frames:
    //
    // Magic_Number    Frame_Header    Data_Block    [More data blocks]    [Content_Checksum]
    //    4 bytes        2-14 bytes      n bytes                                0-4 bytes
    //
    // Now to simplify as much as possible our generated data:
    // - We use a single frame...
    // - ...containing a single data block.
    // - Content checksum is optional (so we don't use it).
    // - Data block contains a 3 bytes header that we set to indicate the content is raw.

    let mut source_buf = Vec::new();
    source.read_to_end(&mut source_buf)?;

    // 1) Magic number

    const ZSTD_MAGIC_NUMBER: u32 = 0xFD2FB528;
    destination.write_all(&ZSTD_MAGIC_NUMBER.to_le_bytes())?;

    // 2) Frame header

    // Frame_Header_Descriptor    [Window_Descriptor]    [Dictionary_ID]    [Frame_Content_Size]
    //     1 byte                      0-1 byte              0-4 bytes          0-8 bytes
    //
    // Frame_Header_Descriptor:
    // Bit number    Field name
    //    7-6         Frame_Content_Size_flag
    //    5           Single_Segment_flag
    //    4           Unused_bit
    //    3           Reserved_bit
    //    2           Content_Checksum_flag
    //    1-0         Dictionary_ID_flag

    let frame_content_size_flag: u8 = 0x3; // 0x3: Frame_Content_Size is encoded over 8 bytes
    let single_segment_flag: u8 = 0x1; // Single segment is set, window system is disabled
    let content_checksum_flag: u8 = 0x0; // No checksum at the end of the frame
    let dictionary_id_flag: u8 = 0; // No dictionary

    let frame_header_descriptor: u8 = (frame_content_size_flag << 6)
        | (single_segment_flag << 5)
        // bit 4 is unused
        // bit 3 is reserved
        | (content_checksum_flag << 2)
        | dictionary_id_flag;

    destination.write_all(&[frame_header_descriptor])?;

    // No window descriptor
    // No dictionary

    // Frame_Content_Size is encoded over 8 bytes
    destination.write_all(&(source_buf.len() as u64).to_le_bytes())?;

    // 3) Data block

    // Block header:
    // Last_Block    Block_Type    Block_Size
    // bit 0          bits 1-2      bits 3-23

    let is_last_block: u32 = 0x1; // Our block is the last one (we only have one !)
    let block_type: u32 = 0x0; // 0x0 stands for "Raw_Block"
    assert!(source_buf.len() <= (0xFFFFFF >> 3)); // Sanity check given size is encoded over 21 bits
    let block_size: u32 = source_buf.len() as u32;
    let block_header: u32 = is_last_block | (block_type << 1) | (block_size << 3);
    // Note `block_header` is a u32 (so 4 bytes), but we should only take the 3 first bytes !
    let block_header_as_bytes = block_header.to_le_bytes();
    assert_eq!(block_header_as_bytes[3], 0); // Sanity check given last byte is dropped
    destination.write_all(&block_header_as_bytes[..3])?;

    // Block content

    destination.write_all(&source_buf)?;

    // All done \o/

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim \
        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip \
        ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
        velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
        cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
        est laborum.";

    #[test]
    fn roundtrip() {
        let mut compressed = Vec::new();
        copy_encode(INPUT, &mut compressed, 0).unwrap();
        let decompressed = decode_all(&compressed[..]).unwrap();

        assert_eq!(INPUT, &decompressed[..]);
    }

    #[test]
    fn real_compression_dirty_decompression() {
        let mut compressed = Vec::new();
        zstd::stream::copy_encode(INPUT, &mut compressed, 0).unwrap();
        let decompressed = decode_all(&compressed[..]).unwrap();

        assert_eq!(INPUT, &decompressed[..]);
    }

    #[test]
    fn dirty_compression_real_decompression() {
        let mut compressed = Vec::new();
        copy_encode(INPUT, &mut compressed, 0).unwrap();
        let decompressed = zstd::stream::decode_all(&compressed[..]).unwrap();

        assert_eq!(INPUT, &decompressed[..]);
    }
}
