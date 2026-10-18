/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::{ApiError, ErrorCode, Result};
use alloc::{string::String, vec::Vec};

fn decode_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(value) => value,
        Err(error) => error.into_bytes().into_iter().map(char::from).collect(),
    }
}

fn collect_i32(
    mut call: impl FnMut(&mut [i32]) -> Result<spring_sdk_internal::BufferFill>,
) -> Result<Vec<i32>> {
    let mut output = Vec::new();
    loop {
        match call(&mut output)? {
            spring_sdk_internal::BufferFill::Complete(count) => {
                if count > output.len() {
                    return Err(ApiError::new(ErrorCode::InvalidState as i32));
                }
                output.truncate(count);
                return Ok(output);
            }
            spring_sdk_internal::BufferFill::Insufficient { required } => {
                output.resize(required, 0)
            }
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }
    fn align(&mut self, alignment: usize) -> Result<()> {
        self.offset = self
            .offset
            .checked_add(alignment - 1)
            .ok_or(ApiError::new(ErrorCode::InvalidState as i32))?
            & !(alignment - 1);
        if self.offset > self.bytes.len() {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        Ok(())
    }
    fn take<const N: usize>(&mut self) -> Result<[u8; N]> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(ApiError::new(ErrorCode::InvalidState as i32))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or(ApiError::new(ErrorCode::InvalidState as i32))?;
        self.offset = end;
        Ok(bytes.try_into().unwrap())
    }
    fn skip(&mut self, count: usize) -> Result<()> {
        self.offset = self
            .offset
            .checked_add(count)
            .ok_or(ApiError::new(ErrorCode::InvalidState as i32))?;
        if self.offset > self.bytes.len() {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        Ok(())
    }
    fn strings(&mut self) -> Result<Vec<String>> {
        self.align(4)?;
        let count = self.u32()? as usize;
        if count > (self.bytes.len() - self.offset) / 4 {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        let mut strings = Vec::with_capacity(count);
        for _ in 0..count {
            self.align(4)?;
            let length = self.u32()? as usize;
            let begin = self.offset;
            self.skip(length)?;
            strings.push(decode_string(self.bytes[begin..self.offset].to_vec()));
        }
        Ok(strings)
    }
    fn finish(&mut self, alignment: usize) -> Result<()> {
        self.align(alignment)?;
        if self.offset != self.bytes.len() {
            return Err(ApiError::new(ErrorCode::InvalidState as i32));
        }
        Ok(())
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take()?))
    }
    fn bool(&mut self) -> Result<bool> {
        match self.u32()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ApiError::new(ErrorCode::InvalidState as i32)),
        }
    }
}

#[derive(Default)]
struct Writer {
    bytes: Vec<u8>,
}
impl Writer {
    fn align(&mut self, alignment: usize) {
        let size = (self.bytes.len() + alignment - 1) & !(alignment - 1);
        self.bytes.resize(size, 0);
    }
    fn zeroes(&mut self, count: usize) {
        self.bytes.resize(self.bytes.len() + count, 0);
    }
    fn u32(&mut self, value: u32) {
        self.align(4);
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }
}

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/scoped_wasm.rs"
));
