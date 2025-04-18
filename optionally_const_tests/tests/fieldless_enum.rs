use optionally_const::{FieldlessEnumConstType, OptionallyConst};

#[derive(FieldlessEnumConstType, Debug, Clone, Copy, PartialEq)]
#[const_type(
    #[derive(Clone, Copy, PartialEq)]
    ConstTypeName
)]
enum FieldlessEnum {
    A,
    B,
    C,
}

impl<const DISCRIMINANT: usize> std::fmt::Debug for ConstTypeName<DISCRIMINANT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstTypeName<{DISCRIMINANT}>")
    }
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

fn test_print_fieldless_enum() {
    print_fieldless_enum(FieldlessEnum::A);
    print_fieldless_enum(FieldlessEnum::B);
    print_fieldless_enum(FieldlessEnum::C);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::A as usize }>);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::B as usize }>);
    print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::C as usize }>);
}

fn test_try_into_const_type_instance() {
    let Ok(_const_type_instance) =
        FieldlessEnum::A.try_into_const_type_instance::<{ FieldlessEnum::A as usize }>()
    else {
        panic!("The conversion from a variant to a corresponding const type instance failed");
    };
    let Ok(_const_type_instance) =
        FieldlessEnum::B.try_into_const_type_instance::<{ FieldlessEnum::B as usize }>()
    else {
        panic!("The conversion from a variant to a corresponding const type instance failed");
    };
    let Ok(_const_type_instance) =
        FieldlessEnum::C.try_into_const_type_instance::<{ FieldlessEnum::C as usize }>()
    else {
        panic!("The conversion from a variant to a corresponding const type instance failed");
    };
}

fn test_maybe_const() {
    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::A as usize }>::MAYBE_CONST,
        Some(FieldlessEnum::A),
    );
    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::B as usize }>::MAYBE_CONST,
        Some(FieldlessEnum::B),
    );
    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::C as usize }>::MAYBE_CONST,
        Some(FieldlessEnum::C),
    );
}

fn test_try_from_value() {
    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::A as usize }>::try_from_value(FieldlessEnum::A),
        Ok(ConstTypeName::<{ FieldlessEnum::A as usize }>)
    );

    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::A as usize }>::try_from_value(FieldlessEnum::B),
        Err(FieldlessEnum::B)
    );

    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::B as usize }>::try_from_value(FieldlessEnum::B),
        Ok(ConstTypeName::<{ FieldlessEnum::B as usize }>)
    );

    assert_eq!(
        ConstTypeName::<{ FieldlessEnum::C as usize }>::try_from_value(FieldlessEnum::C),
        Ok(ConstTypeName::<{ FieldlessEnum::C as usize }>)
    );
}

fn test_try_from_another() {
    assert_eq!(
        <ConstTypeName::<{ FieldlessEnum::A as usize }> as OptionallyConst<FieldlessEnum>>::try_from_another(
            ConstTypeName::<{ FieldlessEnum::A as usize }>,
        ),
        Ok(ConstTypeName::<{ FieldlessEnum::A as usize }>)
    );
    assert_eq!(
        <ConstTypeName::<{ FieldlessEnum::A as usize }> as OptionallyConst<FieldlessEnum>>::try_from_another(
            ConstTypeName::<{ FieldlessEnum::B as usize }>,
        ),
        Err(ConstTypeName::<{ FieldlessEnum::B as usize }>)
    );
    assert_eq!(
        <ConstTypeName::<{ FieldlessEnum::A as usize }> as OptionallyConst<FieldlessEnum>>::try_from_another(
            FieldlessEnum::A,
        ),
        Ok(ConstTypeName::<{ FieldlessEnum::A as usize }>)
    );
    assert_eq!(
        <ConstTypeName::<{ FieldlessEnum::A as usize }> as OptionallyConst<FieldlessEnum>>::try_from_another(
            FieldlessEnum::B,
        ),
        Err(FieldlessEnum::B)
    );
    assert_eq!(
        <FieldlessEnum as OptionallyConst<FieldlessEnum>>::try_from_another(
            ConstTypeName::<{ FieldlessEnum::A as usize }>,
        ),
        Ok(FieldlessEnum::A)
    );
    assert_eq!(
        <FieldlessEnum as OptionallyConst<FieldlessEnum>>::try_from_another(
            ConstTypeName::<{ FieldlessEnum::B as usize }>,
        ),
        Err(ConstTypeName::<{ FieldlessEnum::B as usize }>)
    );
    assert_eq!(
        <FieldlessEnum as OptionallyConst<FieldlessEnum>>::try_from_another(FieldlessEnum::A,),
        Ok(FieldlessEnum::A)
    );
}

fn main() {
    test_print_fieldless_enum();
    test_try_into_const_type_instance();
    test_maybe_const();
    test_try_from_value();
    test_try_from_another();
}
