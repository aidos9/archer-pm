// used for tests
#[allow(unused_macros)]
macro_rules! hashmap {
    [$($k:expr ; $v:expr),*] => {
        vec![$(($k, $v)),*].into_iter().collect()
    };
}

macro_rules! with_function {
    ($field:ident, $field_method:ident, $field_tp:ty, $v:vis) => {
        paste::paste! {
            $v fn [<with_ $field_method>](mut self, $field: $field_tp) -> Self {
                self.$field = $field;

                return self;
            }

            $v fn [<set_ $field_method>](&mut self, $field: $field_tp) {
                self.$field = $field;
            }
        }
    };

    ($field:ident, $field_tp:ty) => {
        with_function!($field, $field, $field_tp, pub);
    };
}
