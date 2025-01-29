use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::be_u16;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
struct DoubleU16<'a> {
    first: &'a [u8; 2],
    second: &'a [u8; 2],
}

#[derive(Debug)]
struct ListU16<'a> {
    l: Vec<&'a [u8; 2]>,
}

fn main() {
    {
        println!("read_double_u16...");
        let bytes = vec![0, 1, 3, 0, 4, 2, 4, 2];
        let (res, d) = read_double_u16(bytes.as_slice()).unwrap();
        println!("d: {:?}", d);
        println!("res: {:?}", res);

        assert_eq!(d.first, &[0, 1]);
        assert_eq!(d.second, &[3, 0]);
    }

    {
        println!("read_list_u16...");
        let bytes = vec![0, 3, 0, 2, 0, 8, 0, 4];
        let (res, l) = read_list_u16(bytes.as_slice()).unwrap();
        println!("l: {:?}", l);
        println!("res: {:?}", res);

        assert!(res.is_empty());
        assert_eq!(l.l.len(), 3);
        assert_eq!(l.l[0], &[0, 2]);
        assert_eq!(l.l[1], &[0, 8]);
        assert_eq!(l.l[2], &[0, 4]);
    }
}

fn read_double_u16(content: &[u8]) -> IResult<&[u8], DoubleU16> {
    // Use (2x) nom take to read 2 bytes
    let (content, bytes_1) = take(2usize)(content)?;
    let (content, bytes_2) = take(2usize)(content)?;
    let d = DoubleU16 {
        first: bytes_1.try_into().unwrap(),
        second: bytes_2.try_into().unwrap(),
    };
    Ok((content, d))
}

fn read_list_u16(content: &[u8]) -> IResult<&[u8], ListU16> {
    // Use nom length_count with nom map & nom take
    #[rustfmt::skip]
    let (content, res): (&[u8], Vec<&[u8; 2]>) =
        length_count(
            be_u16, // read number of elements to read as u16
            map(
                take(2usize), // read elements as &[u8] 
                |r: &[u8]| r.try_into().unwrap() // &[u8] -> &[u8; 2]
            )
        ).parse(content)?;
    let l = ListU16 { l: res };
    Ok((content, l))
}
