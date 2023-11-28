use std::{collections::HashMap, path::PathBuf};

use crate::mir_analyze::mirs::mir::{MIR, MOVE_TYPE, REF_TYPE};

pub fn count_mir_entry(mirs: &Vec<MIR>, out_dir: &PathBuf) -> anyhow::Result<()> {
    count_mir(mirs)?;

    Ok(())
}

fn count_mir(mirs: &Vec<MIR>) -> anyhow::Result<HashMap<String, u32>> {
    let mut mir_number = MIR::get_all_types()
        .into_iter()
        .map(|s| (s, 0))
        .collect::<HashMap<String, u32>>();

    mirs.iter().for_each(|mir| {
        *mir_number.get_mut(&mir.get_type()).unwrap() += 1;

        match mir {
            MIR::CALL(c) => c.params.iter().for_each(|param| match param {
                crate::mir_analyze::mirs::mir::Param::MOVE(_) => {
                    *mir_number.get_mut(&MOVE_TYPE.to_string()).unwrap() += 1
                }
                _ => (),
            }),
            MIR::FIELDACCESS(f) => match f {
                crate::mir_analyze::mirs::mir::FIELDACCESS::MOVE(_) => {
                    *mir_number.get_mut(&MOVE_TYPE.to_string()).unwrap() += 1
                }
                crate::mir_analyze::mirs::mir::FIELDACCESS::REF(_)
                | crate::mir_analyze::mirs::mir::FIELDACCESS::REFMUT(_) => {
                    *mir_number.get_mut(&REF_TYPE.to_string()).unwrap() += 1
                }
            },
            _ => (),
        };
    });

    Ok(mir_number)
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use crate::mir_analyze::{
        analyze::reader::read_mir,
        mirs::mir::{ASSIGN_TYPE, CALL_TYPE, FIELD_ACCESS_TYPE, MOVE_TYPE, REF_TYPE},
    };

    use super::count_mir;

    #[test]
    fn test_count_mir() {
        // let mir_file = PathBuf::from("test/mir_analyze/count/condvar-83823257d08c42e7.mir");
        let mir_file = PathBuf::from("test/mir_analyze/reader/condvar-9b2e97b194975c57.mir");

        let numbers = count_mir(&read_mir(mir_file).unwrap()).unwrap();

        let std_numbers = vec![
            (ASSIGN_TYPE.to_string(), 24u32),
            (MOVE_TYPE.to_string(), 37),
            (CALL_TYPE.to_string(), 34),
            (FIELD_ACCESS_TYPE.to_string(), 4),
            (REF_TYPE.to_string(), 18),
        ]
        .into_iter()
        .collect();

        assert_eq!(numbers, std_numbers);
    }
}
