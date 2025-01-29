use nom::bytes::complete::take;
use nom::error::{ErrorKind, FromExternalError};
use nom::{
    error::{context, ContextError, ParseError},
    multi::length_data,
    number::complete::{be_u16, le_u64},
    sequence::tuple,
    Finish, IResult, Parser,
};
use std::string::FromUtf8Error;

trait Serializer<T, E> {
    fn serialize(&self, value: &T, buffer: &mut Vec<u8>) -> Result<(), E>;
}

trait Deserializer<T> {
    fn deserialize<
        'a,
        E: ParseError<&'a [u8]>
            + ContextError<&'a [u8]>
            // + FromStringUtf8Error<&'a [u8]>
            + FromExternalError<&'a [u8], FromUtf8Error>,
    >(
        &self,
        buffer: &'a [u8],
    ) -> IResult<&'a [u8], T, E>;
}

#[derive(Debug, PartialEq, Eq)]
struct Task {
    id: u64,
    index: u16,
    name: String,
    is_completed: bool,
}

struct TaskSerializer {}

#[derive(Debug)]
enum TaskSerializerError {
    #[allow(dead_code)]
    InvalidNameLength(usize),
}

impl Serializer<Task, TaskSerializerError> for TaskSerializer {
    fn serialize(&self, value: &Task, buffer: &mut Vec<u8>) -> Result<(), TaskSerializerError> {
        buffer.extend(value.id.to_le_bytes());
        buffer.extend(value.index.to_be_bytes());
        let name_length = u64::try_from(value.name.len())
            .map_err(|_e| TaskSerializerError::InvalidNameLength(value.name.len()))?;
        buffer.extend(name_length.to_le_bytes());
        buffer.extend(value.name.as_bytes());
        buffer.push(u8::from(value.is_completed));
        Ok(())
    }
}

struct TaskDeserializer {}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TaskDeserializerError<I> {
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Invalid bool value: {0}")]
    InvalidBool(u8),
    #[error("Nom error: {0}")]
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for TaskDeserializerError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        TaskDeserializerError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for TaskDeserializerError<I> {}

impl<I> FromExternalError<I, FromUtf8Error> for TaskDeserializerError<I> {
    fn from_external_error(_input: I, _kind: ErrorKind, e: FromUtf8Error) -> Self {
        TaskDeserializerError::Utf8Error(e)
    }
}

// This works too but is generic than FromExternalError
/*
pub trait FromStringUtf8Error<I> {
    fn from_utf8_error(e: FromUtf8Error) -> Self;
}

impl<I> FromStringUtf8Error<I> for TaskDeserializerError<I> {
    fn from_utf8_error(e: FromUtf8Error) -> Self {
        TaskDeserializerError::Utf8Error(e)
    }
}
*/

impl Deserializer<Task> for TaskDeserializer {
    fn deserialize<
        'a,
        E: ParseError<&'a [u8]>
            + ContextError<&'a [u8]>
            // + FromStringUtf8Error<&'a [u8]>
            + FromExternalError<&'a [u8], FromUtf8Error>,
    >(
        &self,
        buffer: &'a [u8],
    ) -> IResult<&'a [u8], Task, E> {
        context(
            "Failed Task de",
            tuple((
                context("Fail Task.id de", |input| le_u64(input)),
                context("Fail Task.index de", |input| be_u16(input)),
                context("Fail Task.name", |input: &'a [u8]| {
                    let (input, data) = length_data(le_u64).parse(input)?;
                    #[rustfmt::skip]
                    let data = String::from_utf8(data.to_vec())
                        .map_err(|e| {
                            nom::Err::Error(E::from_external_error(input, ErrorKind::Eof, e))
                        })?;
                    Ok((input, data))
                }),
                context("Fail Task.is_completed de", |input: &'a [u8]| {
                    let (input, content) = take(1usize)(input)?;
                    let value = !matches!(content[0], 0);
                    Ok((input, value))
                }),
            )),
        )
        .map(|(id, index, name, is_completed)| Task {
            id,
            index,
            name,
            is_completed,
        })
        .parse(buffer)
    }
}

fn main() {
    let t1 = Task {
        id: 4242,
        index: 26,
        name: "A wonderful task".to_string(),
        is_completed: false,
    };

    let mut buffer = Vec::new();
    let task_serializer = TaskSerializer {};

    task_serializer
        .serialize(&t1, &mut buffer)
        .expect("Cannot serialize task t1");

    let task_deserializer = TaskDeserializer {};
    let (content, task_de) = task_deserializer
        .deserialize::<TaskDeserializerError<&[u8]>>(&buffer)
        .unwrap();

    assert!(content.is_empty());
    assert_eq!(t1, task_de);

    {
        // id
        let mut bad_serialization = 42u64.to_le_bytes().to_vec();
        // bad index
        bad_serialization.push(9);
        let task_deserializer = TaskDeserializer {};
        let res = task_deserializer.deserialize::<TaskDeserializerError<&[u8]>>(&bad_serialization);
        assert!(res.is_err());
        let res2 = res.finish();
        println!("result: {:?}", res2);
    }

    {
        // id
        let mut bad_serialization2 = 42u64.to_le_bytes().to_vec();
        // index
        bad_serialization2.extend(3u16.to_be_bytes());
        // name len
        bad_serialization2.extend(2u64.to_le_bytes());
        // bad name
        bad_serialization2.extend(vec![0, 159]);

        let task_deserializer = TaskDeserializer {};
        let res = task_deserializer
            .deserialize::<TaskDeserializerError<&[u8]>>(&bad_serialization2)
            // .unwrap();
            ;
        assert!(res.is_err());
        // let res2 = res.finish();
        println!("result: {:?}", res);
    }
}
