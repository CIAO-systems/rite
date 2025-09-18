#[derive(Debug)]
struct Inner {
    value: String,
}

#[derive(Debug)]
struct Field {
    value: Option<Option<Inner>>,
}

#[derive(Debug)]
struct Attr {
    field: Option<Field>,
}

fn create_struct() -> Attr {
    Attr {
        field: Some(Field {
            value: Some(Some(Inner {
                value: "hello".into(),
            })),
        }),
    }
}

#[test]
fn test_macro_unpack_attribute() {
    let attr = create_struct();

    let result = unpack_attribute!(attr, field, value);
    assert_eq!(result.unwrap().value, "hello");
}

#[test]
fn test_macro_get_value() {
    let attr = create_struct();

    let result = get_value!(attr, field);
    assert_eq!(result.unwrap().value, "hello");
}
