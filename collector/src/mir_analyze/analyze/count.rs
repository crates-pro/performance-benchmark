use std::collections::HashMap;

use crate::mir_analyze::mirs::mir::{MIR, MOVE_TYPE, REF_TYPE};

type MirCount = HashMap<String, u32>;

pub fn count_mir(mirs: &Vec<MIR>) -> anyhow::Result<MirCount> {
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
    use std::path::PathBuf;

    use crate::mir_analyze::{
        analyze::reader::read_mir,
        mirs::mir::{ASSIGN_TYPE, CALL_TYPE, FIELD_ACCESS_TYPE, MOVE_TYPE, REF_TYPE},
    };

    use super::count_mir;

    #[test]
    fn test_count_mir() {
        let mir_file = PathBuf::from("test/mir_analyze/count/condvar-83823257d08c42e7.mir");

        let numbers = count_mir(&read_mir(mir_file).unwrap()).unwrap();

        let std_numbers = vec![
            (ASSIGN_TYPE.to_string(), 23u32),
            (MOVE_TYPE.to_string(), 43),
            (CALL_TYPE.to_string(), 24),
            (FIELD_ACCESS_TYPE.to_string(), 3),
            (REF_TYPE.to_string(), 18),
        ]
        .into_iter()
        .collect();

        assert_eq!(numbers, std_numbers);
    }
}
