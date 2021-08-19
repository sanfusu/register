#![no_std]

/// impl for writeable Regs;
pub trait RegWrite<RegBufferType> {
    fn write(buff: RegBufferType);
}

/// Flush the buffer into register;
/// You can impl either RegWrite for Regs
/// or RegBuffFlush for RegBufferType or both.
pub trait RegBufferFlush {
    /// It's not necessary to make it mutable,
    /// but we want to sure use it after RegWriteField::write
    /// rather than RegReadField::output.
    fn flush(&mut self);
}
/// Impl for RegBuffer::Regbuff if you want to config field.
pub trait BufferWriter {
    #[must_use = "The modified value works after flushed into register"]
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: Field<Self> + FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}
/// impl for RegBuffer::Regbuff if you want to get field;
pub trait BufferReader {
    fn read<T: Field<Self> + FieldReader<Self>>(&self) -> T::ValueType {
        T::read(self)
    }
    fn output<T: Field<Self> + FieldReader<Self>>(&self, out: &mut T::ValueType) -> &Self {
        *out = T::read(self);
        self
    }
}

/// impl for Reg's fields;
/// RegFieldWrite and RegFieldRead use the same ValueType and Regbuff to keep consistent.
pub trait Field<RegBufferType>
where
    RegBufferType: ?Sized,
{
    type ValueType;
}

/// impl for RegField's instance
pub trait FieldWriter<RegBufferType>: Field<RegBufferType>
where
    RegBufferType: ?Sized,
{
    fn write(reg_buff: &mut RegBufferType, value: Self::ValueType);
}
/// impl for RegField's instance
pub trait FieldReader<RegBufferType>: Field<RegBufferType>
where
    RegBufferType: ?Sized,
{
    fn read(reg_buff: &RegBufferType) -> Self::ValueType;
}
