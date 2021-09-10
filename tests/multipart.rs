use std::io::Write;

use poem::{Request, RequestBody};
use poem_openapi::{
    payload::{Binary, Field, Payload, PlainText},
    registry::MetaSchema,
    Multipart, ParseRequestError,
};

fn create_multipart_payload(parts: &[(&str, Option<&str>, &[u8])]) -> Vec<u8> {
    let mut data = Vec::new();

    for part in parts {
        data.write_all(b"--X-BOUNDARY\r\n").unwrap();
        match part.1 {
            Some(filename) => data
                .write_all(
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n\r\n",
                        part.0, filename
                    )
                    .as_bytes(),
                )
                .unwrap(),
            None => data
                .write_all(
                    format!(
                        "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                        part.0
                    )
                    .as_bytes(),
                )
                .unwrap(),
        }

        data.write(part.2).unwrap();
        data.write_all(b"\r\n").unwrap();
    }

    data.write_all(b"--X-BOUNDARY--\r\n").unwrap();
    data
}

#[tokio::test]
async fn rename_fields() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    #[oai(rename_fields = "UPPERCASE")]
    struct A {
        name: PlainText,
        file: Binary,
    }

    let data = create_multipart_payload(&[("NAME", None, b"abc"), ("FILE", None, &[1, 2, 3])]);
    let a = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap();
    assert_eq!(
        a,
        A {
            name: PlainText("abc".to_string()),
            file: Binary(vec![1, 2, 3])
        }
    )
}

#[tokio::test]
async fn required_fields() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    struct A {
        name: PlainText,
        file: Binary,
    }

    let schema_ref = A::schema_ref();
    let schema: &MetaSchema = schema_ref.unwrap_inline();
    assert_eq!(schema.ty, "object");
    assert_eq!(schema.properties.len(), 2);

    assert_eq!(schema.properties[0].0, "name");
    assert_eq!(schema.properties[0].1.unwrap_inline().ty, "string");

    assert_eq!(schema.properties[1].0, "file");
    assert_eq!(schema.properties[1].1.unwrap_inline().ty, "binary");

    assert_eq!(schema.required, &["name", "file"]);

    let data = create_multipart_payload(&[("name", None, b"abc")]);
    let err = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap_err();
    assert_eq!(
        err,
        ParseRequestError::ParseRequestBody {
            reason: "field `file` is required".to_string()
        }
    );
}

#[tokio::test]
async fn optional_fields() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    struct A {
        name: Option<PlainText>,
        file: Binary,
    }

    let schema_ref = A::schema_ref();
    let schema: &MetaSchema = schema_ref.unwrap_inline();
    assert_eq!(schema.ty, "object");
    assert_eq!(schema.properties.len(), 2);

    assert_eq!(schema.properties[0].0, "name");
    assert_eq!(schema.properties[0].1.unwrap_inline().ty, "string");

    assert_eq!(schema.properties[1].0, "file");
    assert_eq!(schema.properties[1].1.unwrap_inline().ty, "binary");

    assert_eq!(schema.required, &["file"]);

    let data = create_multipart_payload(&[("file", None, &[1, 2, 3])]);
    let a = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap();
    assert_eq!(
        a,
        A {
            name: None,
            file: Binary(vec![1, 2, 3])
        }
    )
}

#[tokio::test]
async fn rename_field() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    struct A {
        #[oai(name = "Name")]
        name: PlainText,
        file: Binary,
    }

    let data = create_multipart_payload(&[("Name", None, b"abc"), ("file", None, &[1, 2, 3])]);
    let a = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap();
    assert_eq!(
        a,
        A {
            name: PlainText("abc".to_string()),
            file: Binary(vec![1, 2, 3])
        }
    )
}

#[tokio::test]
async fn skip() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    struct A {
        name: PlainText,
        file: Binary,
        #[oai(skip)]
        value1: i32,
        #[oai(skip)]
        value2: i32,
    }

    let data = create_multipart_payload(&[("name", None, b"abc"), ("file", None, &[1, 2, 3])]);
    let a = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap();
    assert_eq!(
        a,
        A {
            name: PlainText("abc".to_string()),
            file: Binary(vec![1, 2, 3]),
            value1: 0,
            value2: 0,
        }
    )
}

#[tokio::test]
async fn field_info() {
    #[derive(Multipart, Debug, Eq, PartialEq)]
    struct A {
        name: PlainText,
        file: Field<Binary>,
    }

    let data =
        create_multipart_payload(&[("name", None, b"abc"), ("file", Some("1.txt"), &[1, 2, 3])]);
    let a = A::from_request(
        &Request::builder()
            .header("content-type", "multipart/form-data; boundary=X-BOUNDARY")
            .finish(),
        &mut RequestBody::new(data.into()),
    )
    .await
    .unwrap();
    assert_eq!(a.name, PlainText("abc".to_string()));
    assert_eq!(a.file.file_name(), Some("1.txt"));
    assert_eq!(a.file.content_type(), None);
    assert_eq!(&*a.file, &Binary(vec![1, 2, 3]));
}
