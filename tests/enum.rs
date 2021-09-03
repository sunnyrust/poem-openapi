use poem_openapi::{
    types::{DataType, Type},
    Enum,
};
use serde_json::Value;

#[derive(Enum, Debug, Eq, PartialEq)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn parse() {
    assert_eq!(
        MyEnum::parse(Some(Value::String("A".to_string()))).unwrap(),
        MyEnum::A
    );
    assert_eq!(
        MyEnum::parse(Some(Value::String("B".to_string()))).unwrap(),
        MyEnum::B
    );
    assert_eq!(
        MyEnum::parse(Some(Value::String("C".to_string()))).unwrap(),
        MyEnum::C
    );
}

#[test]
fn to_value() {
    assert_eq!(MyEnum::A.to_value(), Value::String("A".to_string()));
    assert_eq!(MyEnum::B.to_value(), Value::String("B".to_string()));
    assert_eq!(MyEnum::C.to_value(), Value::String("C".to_string()));
}

#[test]
fn data_type() {
    assert_eq!(
        MyEnum::DATA_TYPE,
        DataType::Enum {
            items: &["A", "B", "C"]
        }
    );
}
