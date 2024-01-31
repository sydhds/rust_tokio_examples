#![allow(clippy::bool_assert_comparison)]

use std::env;
use std::io::Read;
use std::process;

use nom::bytes::complete::{take, take_while};
use nom::combinator::map;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many_m_n;
use nom::number::complete::be_u16;
use nom::IResult;

// Jpeg file format description:
// https://en.wikipedia.org/wiki/JPEG#Syntax_and_structure
// https://en.wikibooks.org/wiki/JPEG_-_Idea_and_Practice/The_header_part
// https://www.ccoderun.ca/programming/2017-01-31_jpeg/
// https://exiftool.org/TagNames/JPEG.html

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Please provide a jpeg file path to read...");
        process::exit(1);
    }
    let jpeg_file_path = args[1].clone();

    println!("Reading jpeg file: {}...", jpeg_file_path);

    // Step 0 - read jpeg file
    let mut fp = std::fs::File::open(jpeg_file_path).expect("Cannot open jpeg file");
    let mut content = Vec::new();
    let read_bytes = fp
        .read_to_end(&mut content)
        .expect("Cannot read jpeg file content");

    println!("Just read {} bytes from file", read_bytes);

    // Step 1 - Read start of Image (aka JPEG file start identification bytes)
    let segment_res = take_segment_id(&content);
    let soi = segment_res.unwrap().1;
    println!("soi: {:?}", soi);

    // Same as previous but with explicit type
    let segment_res_2: IResult<&[u8], &[u8], nom::error::Error<_>> = take(2usize)(&content);
    let soi_2 = segment_res_2.unwrap().1;
    assert_eq!(soi, soi_2);

    // Same as previous but with explicit full type
    // Note:
    //   According to the documentation of take (https://docs.rs/nom/latest/nom/bytes/complete/fn.take.html)
    //   take<C, Input, Error: ParseError<Input>> -> Error should implement the
    //   trait ParseError (https://docs.rs/nom/latest/nom/error/trait.ParseError.html)
    //   from https://docs.rs/nom/latest/nom/error/struct.Error.html, we can see that it impl the trait ParseError
    let segment_res_2_2: IResult<&[u8], &[u8], nom::error::Error<&[u8]>> =
        take(2usize)(&content[..]);
    let soi_2_2 = segment_res_2_2.unwrap().1;
    assert_eq!(soi, soi_2_2);

    // Same as previous but with explicit type (on parser)
    let segment_res_3 = take::<_, _, nom::error::Error<_>>(2usize)(&content[..]);
    let soi_3 = segment_res_3.unwrap().1;
    assert_eq!(soi, soi_3);

    // Same as previous but with full explicit type (on parser)
    let segment_res_4 = take::<usize, &[u8], nom::error::Error<&[u8]>>(2usize)(&content[..]);
    let soi_4 = segment_res_4.unwrap().1;
    assert_eq!(soi, soi_4);

    // Same as previous but with full explicit type (on parser)
    // Note: here we pass a u8 to "take" as it could be converted to usize
    //       cf https://docs.rs/nom/latest/nom/trait.ToUsize.html
    let segment_res_5 = take::<u8, &[u8], nom::error::Error<&[u8]>>(2)(&content[..]);
    let soi_5 = segment_res_5.unwrap().1;
    assert_eq!(soi, soi_5);

    // Step 1 - 2: combine parser (map)
    let take_soi_res = take_soi(&content[..]);
    let soi_6 = take_soi_res.unwrap().1;
    assert_eq!(soi_6, true);

    // Step 1 - 3: combine parser 2
    let take_soi_res_2 = take_soi_2(&[0xFF, 0xD9]);
    assert!(take_soi_res_2.is_err());
    assert_eq!(
        take_soi_res_2.err(),
        Some(nom::Err::Error(SoiError::InvalidSoi))
    );

    // Step 2: read all jpeg segments (max=32)
    println!("Reading all...");
    if let Ok((_content, jpeg_segments)) = read_segments(&content[..]) {
        println!("jpeg_segments: {:?}", jpeg_segments);
    }
}

fn take_segment_id(content: &[u8]) -> IResult<&[u8], &[u8]> {
    // Take 2 bytes from given content
    // This will return IResult<content_left_bytes, segment_id, _>
    take(2usize)(content)
}

fn take_soi(content: &[u8]) -> IResult<&[u8], bool> {
    // Take 2 bytes and check if these 2 bytes (aka segment_id) is valid JPEG start
    map(take(2usize), |segment_id: &[u8]| {
        segment_id[0] == 0xFF && segment_id[1] == 0xD8
    })(content)
}

#[derive(Debug, PartialEq)]
pub enum SoiError<I> {
    InvalidSoi,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for SoiError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        SoiError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn take_soi_2(content: &[u8]) -> IResult<&[u8], &[u8], SoiError<&[u8]>> {
    // Take 2 bytes
    // Return a custom error if these 2 bytes are not valid (for JPEG)

    let res = take(2usize)(content);

    if let Ok((content_left, segment_id)) = res {
        if segment_id[0] == 0xFF && segment_id[1] == 0xD8 {
            Ok((content_left, segment_id))
        } else {
            Err(nom::Err::Error(SoiError::InvalidSoi))
        }
    } else {
        res
    }
}

//

#[derive(Debug, PartialEq)]
pub enum JpegParseSegmentsError<I> {
    InvalidApp0Identifier,
    InvalidSegmentSize(u16, u16), // expected n bytes, got n bytes
    UnhandledSegment((u8, u8)),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for JpegParseSegmentsError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        JpegParseSegmentsError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

const JPEG_IDENTIFIER: &[u8] = "JFIF\0".as_bytes();
const JPEG_APP0_SEGMENT_SIZE: usize = 14; // Without thumbnail data

#[derive(Debug)]
enum DensityUnit {
    NoUnits,
    DotsPerInch,
    DotsPerCm,
}

#[allow(dead_code)]
#[derive(Debug)]
struct App0 {
    file_identifier_mark: [u8; 5],
    major_revision_number: u8,
    minor_revision_number: u8,
    // units_for_density: DensityUnit,
    units_for_density: u8,
    x_density: u16,
    y_density: u16,
    thumbnail_width: u8,
    thumbnail_height: u8,
    // thumbnail_data: Vec<u8>,
}

impl App0 {
    #[allow(dead_code)]
    fn get_density_unit(&self) -> Result<DensityUnit, &str> {
        match self.units_for_density {
            0 => Ok(DensityUnit::NoUnits),
            1 => Ok(DensityUnit::DotsPerInch),
            2 => Ok(DensityUnit::DotsPerCm),
            _ => Err("Unknown density unit"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Frame {
    data_precision: u8,
    image_height: u16,
    image_width: u16,
    components: u8,
}

#[derive(Debug)]
enum JpegSegment {
    StartOfImage,
    App(App0), // Application data
    StartOfFrame(u8, Frame),
    Com(String), // Comment
    Dqt,         // Define Quantization Table
    Dht,         // Define Huffman Table
    StartOfScan,
    EndOfImage,
}

fn take_segment_id_2(content: &[u8]) -> IResult<&[u8], &[u8], JpegParseSegmentsError<&[u8]>> {
    // Read jpeg segment id
    take(2usize)(content)
}

fn take_segment_size(content: &[u8]) -> IResult<&[u8], u16, JpegParseSegmentsError<&[u8]>> {
    // Read jpeg segment size
    be_u16(content)
}

fn take_segment_data(
    segment_id: (u8, u8),
    content: &[u8],
) -> IResult<&[u8], JpegSegment, JpegParseSegmentsError<&[u8]>> {
    let (content, segment) = match segment_id {
        (0xFF, 0xD8) => {
            // Start of Image
            (content, JpegSegment::StartOfImage)
        }
        (0xFF, 0xE0) => {
            // App0

            // Note: segment_size is the size of the current_segment including the length
            let (bytes_after, segment_size_) = take_segment_size(content)?;

            let segment_size = usize::from(segment_size_);
            if segment_size < JPEG_APP0_SEGMENT_SIZE {
                return Err(nom::Err::Error(JpegParseSegmentsError::InvalidSegmentSize(
                    segment_size_,
                    u16::try_from(JPEG_APP0_SEGMENT_SIZE).unwrap(), // safe as it's a const value (could static assert)
                )));
            }

            let (bytes_after_app0, app0_content) = take(JPEG_APP0_SEGMENT_SIZE)(bytes_after)?;

            let file_identifier_mark = [
                app0_content[0],
                app0_content[1],
                app0_content[2],
                app0_content[3],
                app0_content[4],
            ];

            if file_identifier_mark != JPEG_IDENTIFIER {
                return Err(nom::Err::Error(
                    JpegParseSegmentsError::InvalidApp0Identifier,
                ));
            }

            let thumbnail_width = app0_content[12];
            let thumbnail_height = app0_content[13];
            let thumbnail_data_size: usize =
                usize::from(thumbnail_width) * usize::from(thumbnail_height) * 3;

            let app0 = App0 {
                file_identifier_mark,
                major_revision_number: app0_content[5],
                minor_revision_number: app0_content[6],
                units_for_density: app0_content[7],
                x_density: u16::from_be_bytes([app0_content[8], app0_content[9]]),
                y_density: u16::from_be_bytes([app0_content[10], app0_content[11]]),
                thumbnail_width,
                thumbnail_height,
            };

            // Discard thumbnail data
            let (bytes_after_thumbnail, _thumbnail_data) =
                take(thumbnail_data_size)(bytes_after_app0)?;

            // APP0
            (bytes_after_thumbnail, JpegSegment::App(app0))
        }
        (0xFF, 0xC0) => {
            // Start Of Frame (baseline DCT)

            let (bytes_after, _segment_size) = take_segment_size(content)?;
            // XXX: could check against segment size as well
            let (bytes_after_frame0, frame_0_content) = take(8usize)(bytes_after)?;

            let frame = Frame {
                data_precision: frame_0_content[0],
                image_height: u16::from_be_bytes([frame_0_content[1], frame_0_content[2]]),
                image_width: u16::from_be_bytes([frame_0_content[3], frame_0_content[4]]),
                components: frame_0_content[5],
            };

            (bytes_after_frame0, JpegSegment::StartOfFrame(0, frame))
        }
        (0xFF, 0xC2) => {
            // Start Of Frame (progressive DCT)
            let (bytes_after, segment_size) = take_segment_size(content)?;
            let (bytes_after_frame2, frame_0_content) = take(segment_size - 2)(bytes_after)?;

            let frame = Frame {
                data_precision: frame_0_content[0],
                image_height: u16::from_be_bytes([frame_0_content[1], frame_0_content[2]]),
                image_width: u16::from_be_bytes([frame_0_content[3], frame_0_content[4]]),
                components: frame_0_content[5],
            };

            (bytes_after_frame2, JpegSegment::StartOfFrame(2, frame))
        }
        (0xFF, 0xD9) => (content, JpegSegment::EndOfImage),
        (0xFF, 0xFE) => {
            // COM
            let (bytes_after, segment_size) = take_segment_size(content)?;
            let (bytes_after_comment, comment) = take(segment_size - 2)(bytes_after)?;
            // Note: as segment size is u16, the comment size is bounded as well
            (
                bytes_after_comment,
                JpegSegment::Com(String::from_utf8_lossy(comment).to_string()),
            )
        }
        (0xFF, 0xC4) => {
            // DHT
            let (bytes_after, segment_size) = take_segment_size(content)?;
            let (bytes_after_dht, _dht_data) = take(segment_size - 2)(bytes_after)?;
            (bytes_after_dht, JpegSegment::Dht)
        }
        (0xFF, 0xDB) => {
            // DQT
            let (bytes_after, segment_size) = take_segment_size(content)?;
            let (bytes_after_dqt, _dqt_data) = take(segment_size - 2)(bytes_after)?;
            (bytes_after_dqt, JpegSegment::Dqt)
        }
        (0xFF, 0xDA) => {
            // SOS
            let (bytes_after, segment_size) = take_segment_size(content)?;
            let (bytes_after_sos_header, _sos_header) = take(segment_size - 2)(bytes_after)?;
            // let components_in_scan = sos_header_[0];

            // FIXME / TODO: keep track of bytes count read? check <= segment_size
            let mut ba = bytes_after_sos_header;
            loop {
                // Read until we found a byte == OxFF
                let (ba1, _br) = take_while(|i| i != 0xFF)(ba)?;
                // Read the next 2 bytes (0xFF, 0x??)
                let (ba2, br2) = take(2usize)(ba1)?;

                if br2[0] == 0xFF && br2[1] == 0x00 {
                    // Skip byte stuffing
                    ba = ba2
                } else {
                    ba = ba1;
                    break;
                }
            }

            (ba, JpegSegment::StartOfScan)
        }
        _ => {
            return Err(nom::Err::Error(JpegParseSegmentsError::UnhandledSegment((
                segment_id.0,
                segment_id.1,
            ))));
        }
    };

    Ok((content, segment))
}

fn read_segment(content: &[u8]) -> IResult<&[u8], JpegSegment, JpegParseSegmentsError<&[u8]>> {
    // Read one JPEG segment
    take_segment_id_2(content).and_then(|(content, segment_id)| {
        take_segment_data((segment_id[0], segment_id[1]), content)
    })
}

fn read_segments(
    content: &[u8],
) -> IResult<&[u8], Vec<JpegSegment>, JpegParseSegmentsError<&[u8]>> {
    // Note: should compute max "number of segment to read" according to file size (with a max file size allowed)
    //       as segment_size is u16
    // Note 2: not all segment types are supported but it can read this jpeg image:
    //         https://upload.wikimedia.org/wikipedia/commons/3/3f/JPEG_example_flower.jpg
    many_m_n(0, 32, read_segment)(content)
    // Note: based on the many_m_n result, we could do some additional checks like:
    // * start with SOI, end with SOI
    // * not too many COM?
    // * has the required segments (APP0, DQT...)
    // * check that the bytes after SOI are empty
}
