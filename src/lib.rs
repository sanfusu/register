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
    #[must_use = "The modified value works after flushed into register"]
    fn write<T: RegField<RegBuffType = Self> + RegFieldWrite>(
        &mut self,
        value: T::ValueType,
    ) -> &mut Self {
        T::write(self, value);
        self
    }
}
/// impl for RegBuff::Regbuff if you want to get field;
pub trait RegReadField {
    fn read<T: RegField<RegBuffType = Self> + RegFieldRead>(&self) -> T::ValueType {
        T::read(self)
    }
    fn output<T: RegField<RegBuffType = Self> + RegFieldRead>(
        &self,
        out: &mut T::ValueType,
    ) -> &Self {
        *out = T::read(self);
        self
    }
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

#[macro_export]
macro_rules! impl_bool_fields {
    ($(($field:path, $position:literal)),*) => {
        $(impl $field {
            const POSITION: u32 = $position;
        }
        impl RegField for $field {
            type ValueType = bool;

            type RegBuffType = super::Cr0Buff;
        }
        impl RegFieldWrite for $field {
            #[inline]
            fn write(reg_buff: &mut Self::RegBuffType, value: Self::ValueType) {
                reg_buff.data = reg_buff.data.bits(Self::POSITION).write(value.into());
            }
        }
        impl RegFieldRead for $field {
            #[inline]
            fn read(reg_buff: &Self::RegBuffType) -> Self::ValueType {
                reg_buff.data.bits(Self::POSITION).read() == 1
            }
        })*
    };
}
