# Optional const

[![Crates.io](https://img.shields.io/crates/v/optionally_const)](https://crates.io/crates/optionally_const)
[![Downloads](https://img.shields.io/crates/d/optionally_const.svg)](https://crates.io/crates/optionally_const)
[![Documentation](https://docs.rs/optionally_const/badge.svg)](https://docs.rs/optionally_const)
[![License](https://img.shields.io/crates/l/optionally_const)](https://crates.io/crates/optionally_const)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/optionally_const/status.svg)](https://deps.rs/repo/github/JohnScience/optionally_const)

Optional constness on stable Rust.

This crate should be superseded by [keyword genericity](#read-more-about-keyword-genericity) in the future.

## Usage

```rust
use optionally_const::{const_type_instance, OptionallyConst};

fn print_flag<T: OptionallyConst<bool>>(flag: T) {
    if let Some(flag) = T::MAYBE_CONST {
        println!("flag is const: {flag}");
    } else {
        let flag: bool = flag.into_value();
        println!("flag is not const: {flag}");
    };
}

fn main() {
    print_flag(true);
    print_flag(false);
    print_flag(const_type_instance!(true));
    print_flag(const_type_instance!(false));
}
```

## Const type

The const type of type `T` is stipulatively defined as a type whose parameterizations represent various constants
of type `T`.

The const type for a constant value `const VAL: T` is stipulatively defined as the parametrization of the constant type of `T` that represents this constant.

## Limitations

* Rust currently doesn't allow defining a type like `struct ConstType<T, const VAL: T>;` because the type of const parameters must not depend on other generic parameters [\[E770\]]. Consequently, one can't provide a canonical "const type" for any const value.
* The `const_type_instance!` macro currently supports only `bool` type. However, it can be extended to support other types in the future.
* Due to lack of support for [negative trait bounds] and [\[E770\]], it's impossible to implement `OptionallyConst<T>` for all types that implement `Const<T>`. The current implementation only supports `bool` type. However, you can implement both `OptionallyConst<T>` and `Const<T>` for your own types.

## Optional constness for user-defined types

### Enums

#### Using a derive macro

```rust
use optionally_const::{OptionallyConst, FieldlessEnumConstType};

// Clone and Copy derives on the enum are required for the derive macro to work.
#[derive(FieldlessEnumConstType, Debug, Clone, Copy)]
#[const_type(
    // You can use any outer attributes you want here.
    // They will be placed verbatim on the generated type. 
    #[derive(Clone, Copy)]
    ConstTypeName
)]
enum FieldlessEnum {
    A,
    B,
    C,
}

fn print_fieldless_enum<T>(value: T)
where
    T: OptionallyConst<FieldlessEnum>,
{
    if let Some(value) = T::MAYBE_CONST {
        println!("Const value: {:?}", value);
    } else {
        let value: FieldlessEnum = T::into_value(value);
        println!("Non-const value: {:?}", value);
    }
}

fn main() {
    print_fieldless_enum(FieldlessEnum::A);
    print_fieldless_enum(FieldlessEnum::B);
    print_fieldless_enum(FieldlessEnum::C);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::A as usize }>);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::B as usize }>);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::C as usize }>);

    let Ok(_const_type_instance) = FieldlessEnum::A.try_into_const_type_instance::<{FieldlessEnum::A as usize}>() else {
        panic!("The conversion from a variant to a corresponding const type instance failed");
    };
}
```

#### Using a manual implementation

```rust
use optionally_const::{Const, OptionallyConst};

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

fn main() {
    print_my_enum(MyEnum::A);
    print_my_enum(MyEnum::B);
    print_my_enum(MyEnum::C);
    print_my_enum(MyEnumAConstType);
    print_my_enum(MyEnumBConstType);
    print_my_enum(MyEnumCConstType);
}
```

## Use cases

* **More generic const parameters**: at the moment of writing, only a handful of types can be used as [generic const parameters]. However, with this crate, you can accept `T: Const<U>` where `U` is an enum type.
* **Higher control over constant propagation in generic contexts**: you can make it more likely to generate function instantiations that will accept fewer parameters at the ABI level, at the cost of higher complexity of the code and higher likelyhood of generic (aka template) bloat.

## Read more about keyword genericity

* <https://blog.rust-lang.org/inside-rust/2022/07/27/keyword-generics/>

[\[E770\]]: https://doc.rust-lang.org/stable/error_codes/E0770.html
[negative trait bounds]: https://doc.rust-lang.org/beta/unstable-book/language-features/negative-bounds.html
[generic const parameters]: https://doc.rust-lang.org/reference/items/generics.html#const-generics
