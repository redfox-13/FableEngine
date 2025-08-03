use std::any::Any;

pub trait Module: Any {
    fn module_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}

impl dyn Module {
    pub fn downcast_module<T: Module + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
