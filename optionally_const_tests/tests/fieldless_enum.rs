use optionally_const::{FieldlessEnumConstType, OptionallyConst};

#[derive(FieldlessEnumConstType, Debug, Clone, Copy)]
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

    let Ok(_const_type_instance) =
        FieldlessEnum::A.try_into_const_type_instance::<{ FieldlessEnum::A as usize }>()
    else {
        panic!("The conversion from a variant to a corresponding const type instance failed");
    };
}
