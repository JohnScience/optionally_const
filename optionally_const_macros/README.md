# Proc-macro crate for `optionally_const`

This crate exposes a `FieldlessEnumConstType` derive macro that generates a "const type" for a fieldless enum.

This allows you to parameterize the const type with the discriminants of the enum variants to represent them as const values.

```rust
use optionally_const::OptionallyConst;
use optionally_const_macros::FieldlessEnumConstType;

#[derive(FieldlessEnumConstType, Debug)]
#[const_type(ConstTypeName)]
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
}
```
