#![no_std]

/// Flush the buffer into register;
/// You can impl either RegWrite for Regs
/// or RegBuffFlush for RegBufferType or both.
pub trait RegBufferFlush {
    /// It's not necessary to make it mutable,
    /// but we want to sure use it after RegWriteField::write
    /// rather than RegReadField::output.
    fn flush(&mut self);
}
pub trait RegisterBufferReader {
    fn read<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(&self) -> T::ValueType {
        T::read(self)
    }
    fn output<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(
        &self,
        out: &mut T::ValueType,
    ) -> &Self {
        *out = T::read(self);
        self
    }
}
pub trait RegisterBufferWriter {
    #[must_use = "The modified value works after flushed into register"]
    unsafe fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
    #[must_use = "The modified value works after flushed into register"]
    unsafe fn revert<T>(&mut self) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::revert(self);
        self
    }
}
