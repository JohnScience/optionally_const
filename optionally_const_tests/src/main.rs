use optionally_const::FieldlessEnumConstType;

#[derive(FieldlessEnumConstType, Debug, Clone, Copy, PartialEq)]
#[const_type(ConstTypeName)]
pub enum FieldlessEnum {
    A,
    B,
    C,
}

fn main() {
    println!("Hello, world!");
}
