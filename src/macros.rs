// used for tests
#[allow(unused_macros)]
macro_rules! hashmap {
    [$($k:expr ; $v:expr),*] => {
        vec![$(($k, $v)),*].into_iter().collect()
    };
}

macro_rules! with_function {
    ($field:ident, $field_method:ident, $field_tp:ty, $val:expr, $v:vis) => {
        paste::paste! {
            $v fn [<with_ $field_method>](mut self) -> Self {
                self.$field = $val;

                return self;
            }

            $v fn [<set_ $field_method>](&mut self, $field: $field_tp) {
                self.$field = $field;
            }
        }
    };

    ($field:ident, $field_tp:ty, $val:expr) => {
        with_function!($field, $field, $field_tp, $val, pub);
    };
}

macro_rules! define_package_string_enum {
    ($visibility:vis $name:ident {$($variant:ident $(=$override:literal)?),*}, other) => {
        #[derive(Debug, PartialEq, Hash, Clone)]
        $visibility enum $name {
            $($variant),*,
            Other(String)
        }

        paste::paste! {
            struct [< $name Visitor >];
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                return match self {
                    $(
                        Self::$variant => __variant_to_string!($variant $($override)?),
                    )*
                    Self::Other(s) => s.as_str(),
                };
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                return serializer.serialize_str(self.as_str());
            }
        }

        impl<'a> From<&'a str> for $name {
            fn from(s: &'a str) -> Self {
                return match s {
                    $(
                        __variant_to_string!($variant $($override)?) => Self::$variant,
                    )*
                    _ => Self::Other(s.to_string()),
                };
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                return Self::from(s.as_str());
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return write!(f, "{}", self.as_str());
            }
        }

        paste::paste! {
            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    return deserializer.deserialize_str([< $name Visitor >]);
                }
            }

            impl<'de> serde::de::Visitor<'de> for [< $name Visitor >] {
                type Value = $name;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    return formatter.write_str("a string");
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    return Ok($name::from(v));
                }
            }
        }
    };
}

macro_rules! __variant_to_string {
    ($variant:ident) => {
        stringify!($variant)
    };

    ($variant:ident $override:literal) => {
        $override
    };
}
