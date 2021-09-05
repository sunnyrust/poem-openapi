use poem_openapi::{
    registry::{MetaSchema, Registry},
    types::{DataType, Type},
    Schema,
};
use serde_json::json;

fn get_meta<T: Schema>() -> MetaSchema {
    let mut registry = Registry::new();
    T::register(&mut registry);
    registry.schemas.remove(T::NAME).unwrap()
}

#[test]
fn rename() {
    #[derive(Schema)]
    #[oai(name = "Abc")]
    struct Obj {
        a: i32,
    }
    assert_eq!(Obj::NAME, "Abc");
}

#[test]
fn rename_fields() {
    #[derive(Schema)]
    #[oai(rename_fields = "camelCase")]
    struct Obj {
        create_user: i32,
        delete_user: i32,
    }

    let meta = get_meta::<Obj>();
    assert_eq!(meta.properties.get_index(0).unwrap().0, &"createUser");
    assert_eq!(meta.properties.get_index(1).unwrap().0, &"deleteUser");
}

#[test]
fn concretes() {
    #[derive(Schema)]
    #[oai(
        concrete(name = "Obj_i32_i64", params(i32, i64)),
        concrete(name = "Obj_f32_f64", params(f32, f64))
    )]
    struct Obj<T1: Type, T2: Type> {
        create_user: T1,
        delete_user: T2,
    }

    assert_eq!(
        <Obj<i32, i64>>::DATA_TYPE,
        DataType::SchemaReference("Obj_i32_i64")
    );
    let meta = get_meta::<Obj<i32, i64>>();
    assert_eq!(
        meta.properties.get_index(0).unwrap().1.data_type,
        DataType::Normal {
            ty: "integer",
            format: Some("int32")
        }
    );
    assert_eq!(
        meta.properties.get_index(1).unwrap().1.data_type,
        DataType::Normal {
            ty: "integer",
            format: Some("int64")
        }
    );

    assert_eq!(
        <Obj<f32, f64>>::DATA_TYPE,
        DataType::SchemaReference("Obj_f32_f64")
    );
    let meta = get_meta::<Obj<f32, f64>>();
    assert_eq!(
        meta.properties.get_index(0).unwrap().1.data_type,
        DataType::Normal {
            ty: "number",
            format: Some("float32")
        }
    );
    assert_eq!(
        meta.properties.get_index(1).unwrap().1.data_type,
        DataType::Normal {
            ty: "number",
            format: Some("float64")
        }
    );
}

#[test]
fn deprecated() {
    #[derive(Schema)]
    struct Obj {
        a: i32,
    }

    let meta = get_meta::<Obj>();
    assert!(!meta.deprecated);

    #[derive(Schema)]
    #[oai(deprecated)]
    struct ObjDeprecated {
        a: i32,
    }

    let meta = get_meta::<ObjDeprecated>();
    assert!(meta.deprecated);
}

#[test]
fn field_skip() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct Obj {
        a: i32,
        #[oai(skip)]
        b: i32,
    }

    let meta = get_meta::<Obj>();
    assert_eq!(meta.properties.len(), 1);

    assert_eq!(
        Obj::parse(json!({
            "a": 10,
        }))
        .unwrap(),
        Obj { a: 10, b: 0 }
    );

    assert_eq!(
        Obj { a: 10, b: 0 }.to_value(),
        json!({
            "a": 10,
        })
    );
}

#[test]
fn field_name() {
    #[derive(Schema)]
    struct Obj {
        #[oai(name = "b")]
        a: i32,
    }

    let meta = get_meta::<Obj>();
    assert_eq!(meta.properties.get_index(0).unwrap().0, &"b");
}

#[test]
fn register() {
    #[derive(Schema)]
    struct A {
        a: i32,
        b: B,
    }

    #[derive(Schema)]
    struct B {
        c: i64,
    }

    let mut registry = Registry::default();
    A::register(&mut registry);

    let meta_a = registry.schemas.remove("A").unwrap();
    let meta_b = registry.schemas.remove("B").unwrap();

    assert_eq!(meta_a.properties.get_index(0).unwrap().0, &"a");
    assert_eq!(
        meta_a.properties.get_index(0).unwrap().1.data_type,
        DataType::Normal {
            ty: "integer",
            format: Some("int32"),
        }
    );
    assert_eq!(
        meta_a.properties.get_index(1).unwrap().1.data_type,
        DataType::SchemaReference("B"),
    );

    assert_eq!(meta_b.properties.get_index(0).unwrap().0, &"c");
    assert_eq!(
        meta_b.properties.get_index(0).unwrap().1.data_type,
        DataType::Normal {
            ty: "integer",
            format: Some("int64"),
        }
    );
}

#[test]
fn description() {
    /// A
    ///
    /// AB
    /// CDE
    #[derive(Schema)]
    struct Obj {
        a: i32,
    }

    let meta = get_meta::<Obj>();
    assert_eq!(meta.summary, Some("A"));
    assert_eq!(meta.description, Some("AB\nCDE"));
}

#[test]
fn field_description() {
    #[derive(Schema)]
    struct Obj {
        /// A
        ///
        /// AB
        /// CDE
        a: i32,
    }

    let meta = get_meta::<Obj>();
    let field_meta = meta.properties.get_index(0).unwrap().1;
    assert_eq!(field_meta.title, Some("A"));
    assert_eq!(field_meta.description, Some("AB\nCDE"));
}
