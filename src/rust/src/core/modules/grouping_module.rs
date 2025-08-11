use super::module::Module;
use crate::impl_base_module;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum DisplayType {
    List,         // Simple vertical order
    Blocks,       // Block of modules, horizontal list
    Grid,         // Grid of items, fixed-size slots
    Collapsible,  // Same as list but can be hidden
    Tree,         // Hierarchical list, all groups bellow will be ordered like a tree
}

pub struct GroupingModule {
    group_name: String,
    display_type: DisplayType,
    module_order: Vec<Uuid>,
}

impl GroupingModule {
    pub fn new(group_name: String, display_type: Option<DisplayType>) -> Self {
        Self { group_name: group_name, display_type: display_type.unwrap_or(DisplayType::List), module_order: Vec::new() }
    }

    pub fn new_name_only(group_name: String) -> Self {
        Self::new(group_name, None)
    }

    pub fn group_name(&self) -> &str {
        &self.group_name
    }

    pub fn set_group_name(&mut self, group_name: String) {
        self.group_name = group_name;
    }

    pub fn display_type(&self) -> &DisplayType {
        &self.display_type
    }

    pub fn set_display_type(&mut self, display_type: DisplayType) {
        self.display_type = display_type;
    }

    pub fn list(&self) -> &Vec<Uuid> {
        &self.module_order
    }

    pub fn clear_list(&mut self) {
        self.module_order.clear()
    }

    pub fn add_module(&mut self, index: usize) -> Uuid {
        let uuid: Uuid = Uuid::new_v4();

        self.module_order.insert(index, uuid);
        uuid
    }

    pub fn remove_module_with_index(&mut self, index: usize) -> Option<Uuid> {
        if index <= self.module_order.len() {
            Some(self.module_order.remove(index))
        } else {
            None
        }
    }

    pub fn remove_module_with_uuid(&mut self, uuid: Uuid) -> Option<Uuid> {
        if let Some(index) = self.module_order.iter().position(|cur_uuid| *cur_uuid == uuid) {
            self.module_order.remove(index);
            Some(uuid)
        } else {
            None
        }
    }
}

impl_base_module!(GroupingModule,);
impl Default for GroupingModule {
    fn default() -> Self {
        Self {
            group_name: String::from("List of someting"),
            display_type: DisplayType::List,
            module_order: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn new_and_new_name_only() {
        let group1 = GroupingModule::new(String::from("group1"), None);
        assert_eq!(group1.group_name(), "group1");
        assert_eq!(*group1.display_type(), DisplayType::List);
        assert!(group1.list().is_empty());

        let group2 = GroupingModule::new_name_only(String::from("group2"));
        assert_eq!(group2.group_name(), "group2");
        assert_eq!(*group2.display_type(), DisplayType::List);
        assert!(group2.list().is_empty());
    }

    #[test]
    fn add_module() {
        let mut group = GroupingModule::new(String::from("test"), None);

        // Add at index 0 (empty list)
        let uuid1 = group.add_module(0);
        assert_eq!(group.list().len(), 1);
        assert_eq!(group.list()[0], uuid1);

        // Add at index 1 (end)
        let uuid2 = group.add_module(1);
        assert_eq!(group.list().len(), 2);
        assert_eq!(group.list()[1], uuid2);

        // Add at index 1 (middle insert)
        let uuid3 = group.add_module(1);
        assert_eq!(group.list().len(), 3);
        assert_eq!(group.list()[1], uuid3);
        assert_eq!(group.list()[2], uuid2);
    }

    #[test]
    fn remove_module_with_index() {
        let mut group = GroupingModule::new(String::from("test"), None);

        let uuid1 = group.add_module(0);
        let uuid2 = group.add_module(1);

        // Remove valid index
        let removed = group.remove_module_with_index(0);
        assert_eq!(removed, Some(uuid1));
        assert_eq!(group.list().len(), 1);
        assert_eq!(group.list()[0], uuid2);

        // Remove invalid index (out of bounds)
        let removed_none = group.remove_module_with_index(5);
        assert_eq!(removed_none, None);
    }

    #[test]
    fn remove_module_with_uuid() {
        let mut group = GroupingModule::new(String::from("test"), None);

        let uuid1 = group.add_module(0);
        let uuid2 = group.add_module(1);

        // Remove existing uuid
        let removed = group.remove_module_with_uuid(uuid1);
        assert_eq!(removed, Some(uuid1));
        assert_eq!(group.list().len(), 1);
        assert_eq!(group.list()[0], uuid2);

        // Remove non-existing uuid
        let fake_uuid = Uuid::new_v4();
        let removed_none = group.remove_module_with_uuid(fake_uuid);
        assert_eq!(removed_none, None);
        assert_eq!(group.list().len(), 1);
    }

    #[test]
    fn clear_list() {
        let mut group = GroupingModule::new(String::from("test"), None);
        group.add_module(0);
        group.add_module(1);

        assert_eq!(group.list().len(), 2);
        group.clear_list();
        assert!(group.list().is_empty());
    }
}
