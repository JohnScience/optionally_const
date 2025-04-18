#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use optionally_const_macros::FieldlessEnumConstType;

// struct ConstType<T, const VAL: T>;

// type ConstTypeBool<const VAL: bool> = ConstType<bool, VAL>;

#[doc(hidden)]
pub mod hidden {
    #[derive(Clone, Copy, PartialEq)]
    pub struct ConstTypeBool<const VAL: bool>;
}

/// A convenience type alias that represents a constant boolean value.
///
/// Ideally, this should be a partial parametrization of
/// `struct ConstType<T, const VAL: T>` with `T = bool`.
///
/// However, defining such a struct is impossible in Rust at the time
/// of writing this code.
pub type ConstTypeBool<const VAL: bool> = hidden::ConstTypeBool<VAL>;

/// A trait that can be used to represent a type that is either
/// type `T` or a type that represents a constant value of type `T`.
///
/// I.e. `OptionallyConst<T>` is either `T` or `U: Const<T>`.
///
/// See the [`Const`] trait for more information.
#[diagnostic::on_unimplemented(
    message = "`OptionallyConst<T>` is implemented for `T: Clone, Copy, PartialEq`\
        and *should* be implemented for `U: Const<T>`",
    label = "Optionally const type instance expected here",
    note = "If `T` doesn't implement `OptionallyConst<T>` (that is, if `{Self}`==`{T}`) \
        and `{Self}` is local to the crate, \
        consider adding `#[derive(Clone, Copy, PartialEq)]` on the definition of `T`."
)]
pub trait OptionallyConst<T>: Clone + Copy + PartialEq + Sized {
    /// An optional constant value of type `T`.
    ///
    /// If the type does not represent a constant
    /// (i.e. does not implement [`Const`]), this will be `None`.
    const MAYBE_CONST: Option<T>;

    /// Converts the instance of the type into a value of type `T`.
    fn into_value(self) -> T;

    /// Converts the value of type `T` into an instance of the type.
    ///
    /// # Errors
    ///
    /// If the value does not match the value of the optionally constant
    /// type instance, this function will return an error.
    fn try_from_value(value: T) -> Result<Self, T>;

    /// Converts the value of type `U` into an instance of the type.
    ///
    /// # Errors
    ///
    /// If the `other` value is an instance of a [const type] and `Self` is a parametrization
    /// of a [const type], this function will return an error if the associated constants on
    /// `Self` and `U` do not match.
    ///
    /// [const type]: https://github.com/JohnScience/optionally_const/tree/main/optionally_const#const-type
    fn try_from_another<U>(another: U) -> Result<Self, U>
    where
        U: OptionallyConst<T>,
        T: Clone + Copy + PartialEq,
    {
        Self::try_from_value(another.into_value())
            .ok()
            .ok_or(another)
    }
}

/// A trait whose types-implementors represent a constant value of type `T`.
pub trait Const<T> {
    /// The constant value of type `T`.
    const VALUE: T;
}

impl<const VAL: bool> Const<bool> for ConstTypeBool<VAL> {
    const VALUE: bool = VAL;
}

// TODO: redefine the impls once negative trait bounds are available

impl<T> OptionallyConst<T> for T
where
    T: Clone + Copy + PartialEq,
{
    const MAYBE_CONST: Option<T> = None;

    fn into_value(self) -> T {
        self
    }

    fn try_from_value(value: T) -> Result<Self, T> {
        Ok(value)
    }
}

// The following impl requires negative trait bounds.
// impl<T,U> OptionallyConst<T> for U
// where
//     U: Const<T>,
// {
//     const MAYBE_CONST: Option<T> = Some(U::VALUE);
//
//     fn into_value(self) -> T {
//         U::VALUE
//     }
// }

impl<const VAL: bool> OptionallyConst<bool> for ConstTypeBool<VAL> {
    const MAYBE_CONST: Option<bool> = Some(VAL);

    fn into_value(self) -> bool {
        VAL
    }

    fn try_from_value(value: bool) -> Result<Self, bool> {
        if value == VAL {
            Ok(crate::hidden::ConstTypeBool::<VAL>)
        } else {
            Err(value)
        }
    }
}

/// Returns an instance of the type that represents the constant.
///
/// At the moment of writing, the macro cannot support user-defined types
/// implementing the [`Const`] trait.
///
/// However, you still can construct instances of types that represent
/// constant values of type `T` manually.
#[macro_export]
macro_rules! const_type_instance {
    (true $(: bool)?) => {
        $crate::hidden::ConstTypeBool::<true>
    };
    (false $(: bool)?) => {
        $crate::hidden::ConstTypeBool::<false>
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_type() {
        let a: ConstTypeBool<true> = const_type_instance!(true);
        let b: ConstTypeBool<false> = const_type_instance!(false);

        let a_value: bool = a.into_value();
        let b_value: bool = b.into_value();

        assert_eq!(a_value, true);
        assert_eq!(b_value, false);
    }

    fn print_flag<T: OptionallyConst<bool>>(flag: T) {
        if let Some(flag) = T::MAYBE_CONST {
            println!("flag is const: {flag}");
        } else {
            let flag: bool = flag.into_value();
            println!("flag is not const: {flag}");
        };
    }

    #[derive(Clone, Copy, PartialEq)]
    enum MyEnum {
        A,
        B,
        C,
    }

    impl std::fmt::Display for MyEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyEnum::A => write!(f, "A"),
                MyEnum::B => write!(f, "B"),
                MyEnum::C => write!(f, "C"),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    struct MyEnumAConstType;
    #[derive(Clone, Copy, PartialEq)]
    struct MyEnumBConstType;
    #[derive(Clone, Copy, PartialEq)]
    struct MyEnumCConstType;

    // Ideally, OptionallyConst<T> should be implemented for
    // all types that implement Const<T>.
    //
    // However, impl of OptionallyConst<T> for all `T` conflicts with the
    // impl of OptionallyConst<T> for all `U: Const<T>` in the absence of
    // negative trait bounds.

    impl Const<MyEnum> for MyEnumAConstType {
        const VALUE: MyEnum = MyEnum::A;
    }

    impl OptionallyConst<MyEnum> for MyEnumAConstType {
        const MAYBE_CONST: Option<MyEnum> = Some(MyEnum::A);

        fn into_value(self) -> MyEnum {
            MyEnum::A
        }

        fn try_from_value(value: MyEnum) -> Result<Self, MyEnum> {
            if matches!(value, MyEnum::A) {
                Ok(MyEnumAConstType)
            } else {
                Err(value)
            }
        }
    }

    impl Const<MyEnum> for MyEnumBConstType {
        const VALUE: MyEnum = MyEnum::B;
    }

    impl OptionallyConst<MyEnum> for MyEnumBConstType {
        const MAYBE_CONST: Option<MyEnum> = Some(MyEnum::B);

        fn into_value(self) -> MyEnum {
            MyEnum::B
        }

        fn try_from_value(value: MyEnum) -> Result<Self, MyEnum> {
            if matches!(value, MyEnum::B) {
                Ok(MyEnumBConstType)
            } else {
                Err(value)
            }
        }
    }

    impl Const<MyEnum> for MyEnumCConstType {
        const VALUE: MyEnum = MyEnum::C;
    }

    impl OptionallyConst<MyEnum> for MyEnumCConstType {
        const MAYBE_CONST: Option<MyEnum> = Some(MyEnum::C);

        fn into_value(self) -> MyEnum {
            MyEnum::C
        }

        fn try_from_value(value: MyEnum) -> Result<Self, MyEnum> {
            if matches!(value, MyEnum::C) {
                Ok(MyEnumCConstType)
            } else {
                Err(value)
            }
        }
    }

    fn print_my_enum<T: OptionallyConst<MyEnum>>(value: T) {
        if let Some(value) = T::MAYBE_CONST {
            println!("value is const: {value}");
        } else {
            let value: MyEnum = value.into_value();
            println!("value is not const: {value}");
        };
    }

    #[test]
    fn test_output_bool() {
        print_flag(true);
        print_flag(false);
        print_flag(const_type_instance!(true));
        print_flag(const_type_instance!(false));
    }

    #[test]
    fn test_output_my_enum() {
        print_my_enum(MyEnum::A);
        print_my_enum(MyEnum::B);
        print_my_enum(MyEnum::C);
        print_my_enum(MyEnumAConstType);
        print_my_enum(MyEnumBConstType);
        print_my_enum(MyEnumCConstType);
    }
}
