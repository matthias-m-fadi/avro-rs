use crate::{AvroResult, Error};
use num_bigint::{BigInt, Sign};

#[derive(Debug, Clone)]
pub struct Decimal {
    value: BigInt,
    len: usize,
}

// We only care about value equality, not byte length.
impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Decimal {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    fn to_vec(&self) -> Vec<u8> {
        // self.len is the byte length this decimal was deserialized from.
        // It is therefore guaranteed that it can be serialized into it again.
        self.to_sign_extended_bytes_with_len(self.len).unwrap()
    }

    pub(crate) fn to_sign_extended_bytes_with_len(&self, len: usize) -> AvroResult<Vec<u8>> {
        let sign_byte = 0xFF * u8::from(self.value.sign() == Sign::Minus);
        let mut decimal_bytes = vec![sign_byte; len];
        let raw_bytes = self.value.to_signed_bytes_be();
        let num_raw_bytes = raw_bytes.len();
        let start_byte_index = len.checked_sub(num_raw_bytes).ok_or(Error::SignExtend {
            requested: len,
            needed: num_raw_bytes,
        })?;
        decimal_bytes[start_byte_index..].copy_from_slice(&raw_bytes);
        Ok(decimal_bytes)
    }
}

impl std::convert::From<&Decimal> for Vec<u8> {
    fn from(decimal: &Decimal) -> Self {
        decimal.to_vec()
    }
}

impl<T: AsRef<[u8]>> From<T> for Decimal {
    fn from(bytes: T) -> Self {
        let bytes_ref = bytes.as_ref();
        Self {
            value: BigInt::from_signed_bytes_be(bytes_ref),
            len: bytes_ref.len(),
        }
    }
}
