#![no_std]

use core::ops::{BitAnd, BitOr, Not, Shl, Shr};

pub trait Readable {}
pub trait Writeable {}

pub trait Register<T> {
    fn write(value: T)
    where
        Self: Writeable;
    fn read() -> T
    where
        Self: Readable;
}

pub trait Bitssor<T>
where
    Self: Register<T>,
    T: Shl<Output = T>
        + BitAnd<Output = T>
        + Shl<Output = T>
        + BitOr<Output = T>
        + Not<Output = T>
        + Shr<Output = T>,
{
    fn read<Field: BitField<T> + Readable>(&self) -> T
    where
        Self: Readable,
    {
        let raw_data = <Self as Register<T>>::read();
        raw_data & Field::MASK >> Field::OFFSET
    }
    fn write<Field: BitField<T> + Writeable + Readable>(&mut self, value: T)
    where
        Self: Writeable + Readable,
    {
        let mut raw_data = <Self as Register<T>>::read();
        raw_data = (raw_data & !(Field::MASK)) | (value << Field::OFFSET);
        <Self as Register<T>>::write(raw_data);
    }
}

pub trait BitField<T> {
    const OFFSET: T;
    const MASK: T;
}
