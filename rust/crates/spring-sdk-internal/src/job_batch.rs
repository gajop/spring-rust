// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
//! Owned, bounded wire data for synchronous pure jobs. No native struct ABI.

use crate::{ApiError, ErrorCode, Result};
use alloc::vec::Vec;

pub const MAX_JOBS: usize = 256;
pub const MAX_BYTES: usize = 1024 * 1024;
pub const MAX_TOTAL_BYTES: usize = 16 * MAX_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobError(pub u32);

/// One pure invocation and its maximum returned byte length. Exceeding the
/// output bound produces `JobError(4)` in this job's ordered result slot.
#[derive(Debug, Clone, Copy)]
pub struct BatchJob<'a> {
    pub input: &'a [u8],
    pub max_output_bytes: usize,
}

pub type OwnedJobResult = core::result::Result<Vec<u8>, JobError>;

fn invalid() -> ApiError {
    ApiError::new(ErrorCode::InvalidArgument as i32)
}

/// Validate the complete batch before allocating/copying or admitting work.
pub fn bounds(jobs: &[BatchJob<'_>]) -> Result<(usize, usize)> {
    if jobs.is_empty() || jobs.len() > MAX_JOBS {
        return Err(invalid());
    }
    let mut inputs = 0;
    let mut outputs = 0;
    for job in jobs {
        if job.input.len() > MAX_BYTES || job.max_output_bytes > MAX_BYTES {
            return Err(invalid());
        }
        inputs += job.input.len();
        outputs += job.max_output_bytes;
        if inputs > MAX_TOTAL_BYTES || outputs > MAX_TOTAL_BYTES {
            return Err(ApiError::new(ErrorCode::BufferOverflow as i32));
        }
    }
    let header = 8 + 8 * jobs.len();
    Ok((header + inputs, header + outputs))
}

pub fn encode(jobs: &[BatchJob<'_>]) -> Result<(Vec<u8>, usize)> {
    let (input_bytes, output_bytes) = bounds(jobs)?;
    let mut wire = Vec::with_capacity(input_bytes);
    wire.extend_from_slice(&1u32.to_le_bytes());
    wire.extend_from_slice(&(jobs.len() as u32).to_le_bytes());
    for job in jobs {
        wire.extend_from_slice(&(job.input.len() as u32).to_le_bytes());
        wire.extend_from_slice(&(job.max_output_bytes as u32).to_le_bytes());
        wire.extend_from_slice(job.input);
    }
    Ok((wire, output_bytes))
}

fn word(wire: &mut &[u8]) -> Option<u32> {
    let bytes = wire.get(..4)?;
    let value = u32::from_le_bytes(bytes.try_into().ok()?);
    *wire = &wire[4..];
    Some(value)
}

/// Validate every result before publishing any owned result to the caller.
pub fn decode(mut wire: &[u8], jobs: &[BatchJob<'_>]) -> Result<Vec<OwnedJobResult>> {
    bounds(jobs)?;
    let malformed = || ApiError::new(ErrorCode::InvalidState as i32);
    if word(&mut wire) != Some(1) || word(&mut wire) != Some(jobs.len() as u32) {
        return Err(malformed());
    }
    let mut cursor = wire;
    for job in jobs {
        let status = word(&mut cursor).ok_or_else(malformed)?;
        let len = word(&mut cursor).ok_or_else(malformed)? as usize;
        if status > 5
            || len > job.max_output_bytes
            || len > cursor.len()
            || (status != 0 && len != 0)
        {
            return Err(malformed());
        }
        cursor = &cursor[len..];
    }
    if !cursor.is_empty() {
        return Err(malformed());
    }
    let mut results = Vec::with_capacity(jobs.len());
    for _ in jobs {
        let status = word(&mut wire).ok_or_else(malformed)?;
        let len = word(&mut wire).ok_or_else(malformed)? as usize;
        results.push(if status == 0 {
            Ok(wire[..len].to_vec())
        } else {
            Err(JobError(status))
        });
        wire = &wire[len..];
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn bounds_and_wire_are_explicit_and_complete() {
        let jobs = [
            BatchJob {
                input: &[7, 8],
                max_output_bytes: 3,
            },
            BatchJob {
                input: &[],
                max_output_bytes: 0,
            },
        ];
        let (wire, capacity) = encode(&jobs).unwrap();
        assert_eq!(capacity, 27);
        assert_eq!(
            wire,
            [
                1, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        let response = [
            1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 10, 11, 2, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(
            decode(&response, &jobs).unwrap(),
            [Ok(vec![10, 11]), Err(JobError(2))]
        );
        for len in 0..response.len() {
            assert!(decode(&response[..len], &jobs).is_err());
        }
        let mut trailing = response.to_vec();
        trailing.push(0);
        assert!(decode(&trailing, &jobs).is_err());
        for (offset, value) in [(0, 2), (4, 1), (8, 6), (12, 4), (18, 6), (22, 1)] {
            let mut changed = response;
            changed[offset] = value;
            assert!(decode(&changed, &jobs).is_err(), "offset {offset}");
        }
    }

    #[test]
    fn complete_limits_reject_without_truncation() {
        assert!(bounds(&[]).is_err());
        let empty = BatchJob {
            input: &[],
            max_output_bytes: 0,
        };
        assert!(bounds(&vec![empty; MAX_JOBS]).is_ok());
        assert!(bounds(&vec![empty; MAX_JOBS + 1]).is_err());
        let large = vec![0; MAX_BYTES];
        let job = BatchJob {
            input: &large,
            max_output_bytes: MAX_BYTES,
        };
        assert!(bounds(&[job; 16]).is_ok());
        assert!(bounds(&[job; 17]).is_err());
        assert!(
            bounds(&[BatchJob {
                input: &[],
                max_output_bytes: MAX_BYTES + 1
            }])
            .is_err()
        );
        let oversize = vec![0; MAX_BYTES + 1];
        assert!(
            bounds(&[BatchJob {
                input: &oversize,
                max_output_bytes: 0
            }])
            .is_err()
        );
    }
}
