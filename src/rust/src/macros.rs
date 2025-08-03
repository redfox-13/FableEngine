#[macro_export]
macro_rules! impl_base_module {
    ($type:ty, $($impl_functions:item)*) => {
        impl Module for $type {
            fn module_name(&self) -> &'static str {
                stringify!($type)
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            $($impl_functions)*
        }
    };
}

