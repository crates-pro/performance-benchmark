use std::collections::HashMap;

use super::tex_writer::TableDatas;

pub struct Data {
    mir_count: TableDatas<String, String, u32>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            mir_count: HashMap::default(),
        }
    }
}
