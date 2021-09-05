use poem_openapi::{
    types::{DataType, Type},
    Enum,
};
use serde_json::Value;

#[test]
fn data_type() {
    #[derive(Enum, Debug, Eq, PartialEq)]
    enum MyEnum {
        CreateUser,
        DeleteUser,
    }

    assert_eq!(
        MyEnum::DATA_TYPE,
        DataType::Enum {
            items: &["CREATE_USER", "DELETE_USER"]
        }
    );
}

#[test]
fn rename_items() {
    #[derive(Enum, Debug, Eq, PartialEq)]
    #[oai(rename_items = "camelCase")]
    enum MyEnum {
        CreateUser,
        DeleteUser,
    }

    assert_eq!(
        MyEnum::parse(Value::String("createUser".to_string())).unwrap(),
        MyEnum::CreateUser
    );

    assert_eq!(
        MyEnum::parse(Value::String("deleteUser".to_string())).unwrap(),
        MyEnum::DeleteUser
    );

    assert_eq!(
        MyEnum::CreateUser.to_value(),
        Value::String("createUser".to_string())
    );
    assert_eq!(
        MyEnum::DeleteUser.to_value(),
        Value::String("deleteUser".to_string())
    );
}

#[test]
fn rename_item() {
    #[derive(Enum, Debug, Eq, PartialEq)]
    enum MyEnum {
        CreateUser,
        #[oai(name = "delete_user")]
        DeleteUser,
    }

    assert_eq!(
        MyEnum::parse(Value::String("CREATE_USER".to_string())).unwrap(),
        MyEnum::CreateUser
    );

    assert_eq!(
        MyEnum::parse(Value::String("delete_user".to_string())).unwrap(),
        MyEnum::DeleteUser
    );

    assert_eq!(
        MyEnum::CreateUser.to_value(),
        Value::String("CREATE_USER".to_string())
    );
    assert_eq!(
        MyEnum::DeleteUser.to_value(),
        Value::String("delete_user".to_string())
    );
}
