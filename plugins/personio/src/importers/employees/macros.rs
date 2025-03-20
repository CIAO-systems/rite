#[macro_export]
macro_rules! unpack_attribute {
    ($attr:expr, $field:ident, $sub_field:ident) => {
        $attr
            .$field
            .as_ref()
            .and_then(|field| field.$sub_field.as_ref())
            .and_then(|inner| inner.as_ref())
    };
}

#[macro_export]
macro_rules! get_label_and_value {
    ($attr:expr, $field:ident) => {
        if let Some(label) = macros::unpack_attribute!($attr, $field, label) {
            if let Some(value) = macros::unpack_attribute!($attr, $field, value) {
                if let Some(label) = label.as_str() {
                    Some((label, value.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! add_field {
    ($fields:expr, $attr:expr, $field:ident) => {
        if let Some((_label, value)) = macros::get_label_and_value!($attr, $field) {
            model::field::add_field(
                $fields,
                stringify!($field),
                model::value::Value::from(value),
            );
            // add_field($fields, label, Value::from(value));
        }
    };
}

pub(crate) use add_field;
pub(crate) use get_label_and_value;
pub(crate) use unpack_attribute;
