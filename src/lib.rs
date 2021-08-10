#![no_std]

/// impl for Regs. All the work start from buff;
pub trait RegBuff {
    type RegBuffType; // : RegWriteField + RegReadField + RegBuffFlush;
    /// Read the register and copy the content into the buff which's type is [`Self::RegBuffType`].
    fn buff() -> Self::RegBuffType;
}

/// impl for writeable Regs;
pub trait RegWrite: RegBuff {
    fn write(buff: Self::RegBuffType);
}

/// Flush the buff into register;
/// You can impl either RegWrite for Regs
/// or RegBuffFlush for [`RegBuff::RegBuffType`] or both.
pub trait RegBuffFlush {
    /// It's not necessary to make it mutable,
    /// but we want to sure use it after RegWriteField::write
    /// rather than RegReadField::output.
    fn flush(&mut self);
}
/// Impl for RegBuff::Regbuff if you want to config field.
pub trait RegWriteField {
    fn write<T: RegField + RegFieldWrite>(&mut self, value: T::ValueType) -> &mut Self;
}
/// impl for RegBuff::Regbuff if you want to get field;
pub trait RegReadField {
    fn read<T: RegField + RegFieldRead>(&self) -> T::ValueType;
    fn output<T: RegField + RegFieldRead>(&self, out: &mut T::ValueType) -> &Self;
}

/// impl for Reg's fields;
/// RegFieldWrite and RegFieldRead use the same ValueType and Regbuff to keep consistent.
pub trait RegField {
    type ValueType;
    /// RegBuffType is the same as [`RegBuff::RegBuffType`]
    type RegBuffType;
}

/// impl for RegField's instance
pub trait RegFieldWrite: RegField {
    fn write(reg_buff: &mut Self::RegBuffType, value: Self::ValueType);
}
/// impl for RegField's instance
pub trait RegFieldRead: RegField {
    fn read(reg_buff: &Self::RegBuffType) -> Self::ValueType;
}
