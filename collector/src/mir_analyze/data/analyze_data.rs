use std::{collections::HashMap, path::Path};

use crate::mir_analyze::{analyze::count::MirCount, mirs::mir::MIR};

use super::table_data::{write_tex_table, TableDatas};

pub struct Data {
    mir_count: TableDatas<String, String, u32>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            mir_count: MIR::get_all_types()
                .into_iter()
                .map(|mir_item| (mir_item, HashMap::default()))
                .collect(),
        }
    }

    pub fn add_mir_count(&mut self, benchmark: &String, mir_count: MirCount) {
        self.mir_count.iter_mut().for_each(|(mir_item, v)| {
            v.insert(benchmark.clone(), *mir_count.get(mir_item).unwrap());
        })
    }

    pub fn write_all(self, out_dir: &Path) -> anyhow::Result<()> {
        write_tex_table(
            &self.mir_count,
            out_dir,
            "MIR Count".to_string(),
            "MIR Count".to_string(),
        )?;

        Ok(())
    }
}
