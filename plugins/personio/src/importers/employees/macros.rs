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
macro_rules! get_value {
    ($attr:expr, $field:ident) => {
        if let Some(value) = macros::unpack_attribute!($attr, $field, value) {
            Some(value.clone())
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! add_field {
    ($fields:expr, $attr:expr, $field:ident) => {
        if let Some(value) = macros::get_value!($attr, $field) {
            model::field::add_field(
                $fields,
                stringify!($field),
                model::value::Value::from(value),
            );
        }
    };
}

macro_rules! add_field_none {
    ($record:expr, $attributes:expr, $field:ident) => {
        let inner = &$attributes.$field;
        $record.fields_as_mut().push(model::field::Field::new_value(
            stringify!($field),
            model::value::Value::from(inner.clone()),
        ));
    };
}

macro_rules! add_field_direct {
    ($record:expr, $attributes:expr, $field:ident) => {
        if let Some(inner) = &$attributes.$field {
            $record.fields_as_mut().push(model::field::Field::new_value(
                stringify!($field),
                model::value::Value::from(inner.clone()),
            ));
        }
    };
}

macro_rules! add_field_option {
    ($record:expr, $attributes:expr, $field:ident) => {
        if let Some(outer) = &$attributes.$field {
            if let Some(inner) = outer {
                $record.fields_as_mut().push(model::field::Field::new_value(
                    stringify!($field),
                    model::value::Value::from(inner.clone()),
                ));
            }
        }
    };
}

macro_rules! add_field_boxed {
    ($record:expr, $attributes:expr, $field:ident) => {
        if let Some(boxed) = &$attributes.$field {
            if let Some(ref outer) = boxed.value {
                if let Some(inner) = outer {
                    $record.fields_as_mut().push(model::field::Field::new_value(
                        stringify!($field),
                        model::value::Value::from(inner.clone()),
                    ));
                }
            }
        }
    };
}

pub(crate) use add_field;
pub(crate) use get_value;
pub(crate) use unpack_attribute;

pub(crate) use add_field_none;
pub(crate) use add_field_boxed;
pub(crate) use add_field_direct;
pub(crate) use add_field_option;
