use optionally_const::FieldlessEnumConstType;

#[derive(FieldlessEnumConstType, Debug, Clone, Copy, PartialEq)]
#[const_type(
    #[derive(Clone, Copy, PartialEq)]
    ConstTypeName
)]
pub enum FieldlessEnum {
    A,
    B,
    C,
}

fn main() {
    println!("Hello, world!");
}
