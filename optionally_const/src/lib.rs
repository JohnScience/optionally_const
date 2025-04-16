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
pub trait OptionallyConst<T> {
    /// An optional constant value of type `T`.
    ///
    /// If the type does not represent a constant
    /// (i.e. does not implement [`Const`]), this will be `None`.
    const MAYBE_CONST: Option<T>;

    /// Converts the instance of the type into a value of type `T`.
    fn into_value(self) -> T;
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

impl<T> OptionallyConst<T> for T {
    const MAYBE_CONST: Option<T> = None;

    fn into_value(self) -> T {
        self
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

    struct MyEnumAConstType;
    struct MyEnumBConstType;
    struct MyEnumCConstType;

    impl Const<MyEnum> for MyEnumAConstType {
        const VALUE: MyEnum = MyEnum::A;
    }

    impl OptionallyConst<MyEnum> for MyEnumAConstType {
        const MAYBE_CONST: Option<MyEnum> = Some(MyEnum::A);

        fn into_value(self) -> MyEnum {
            MyEnum::A
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
    }

    impl Const<MyEnum> for MyEnumCConstType {
        const VALUE: MyEnum = MyEnum::C;
    }

    impl OptionallyConst<MyEnum> for MyEnumCConstType {
        const MAYBE_CONST: Option<MyEnum> = Some(MyEnum::C);

        fn into_value(self) -> MyEnum {
            MyEnum::C
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
