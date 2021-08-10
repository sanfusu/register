#![no_std]

/// impl for Regs. All the work start from buff;
pub trait RegBuff {
    type RegBuffType; // : RegWriteField + RegReadField;
    fn buff() -> Self::RegBuffType;
}

/// impl for writeable Regs;
pub trait RegWrite: RegBuff {
    fn write(buff: Self::RegBuffType);
}

/// Impl for RegBuff::Regbuff if you want to config field.
pub trait RegWriteField {
    fn write<T: RegField>(&mut self, value: T::ValueType) -> &mut Self;
}
/// impl for RegBuff::Regbuff if you want to get field;
pub trait RegReadField {
    fn read<T: RegField>(&self) -> T::ValueType;
    fn output<T: RegField>(&self, out: &mut T::ValueType) -> &Self;
}

/// impl for Reg's fields;
/// RegFieldWrite and RegFieldRead use the same ValueType and Regbuff to keep consistency.
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
