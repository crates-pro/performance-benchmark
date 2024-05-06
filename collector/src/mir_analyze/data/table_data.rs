use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    vec::IntoIter,
};

use nalgebra::DMatrix;

use crate::pca_analysis::pca_data::PcaRawData;

use crate::mir_analyze::mir::function_pattern::*;
use crate::mir_analyze::mir::io_function::*;
use crate::mir_analyze::mir::oop_pattern::*;
use crate::mir_analyze::mir::parallelism::*;
use crate::mir_analyze::mir::reader::*;

/// `TableDatas` represents a 2D table with column labels of type X,
/// row labels of type Y and data of type T.
pub type TableDatas<X, Y, T> = HashMap<X, HashMap<Y, T>>;

pub fn generate_benchmark_data() -> TableDatas<String, String, i32> {
    let mut table_data = HashMap::new();

    vec![
        "io_call",
        "parallelism_call",
        "parallelism_struct",
        "oop_lof",
        "oop_dfc",
        "oop_pbf",
        "oop_wms",
        "oop_rfs",
        "oop_noc",
        "pure_function",
        "closure",
        "higher_function",
    ]
    .iter()
    .for_each(|s| {
        table_data.insert(s.to_string(), HashMap::new());
    });

    for test_file in TEST_FILES.iter() {
        let column_name = test_file.name.to_string();
        let file_path = test_file.path.to_string();
        let mir = &parse_mir(File::open(file_path.clone()).unwrap()).unwrap();

        //let test_file = File::open(file_path).unwrap();
        vec![
            ("io_call".to_string(), count_io_metrics(mir)),
            (
                "parallelism_call".to_string(),
                count_parallelism_metrics(mir),
            ),
            (
                "parallelism_struct".to_string(),
                count_parallelism_strcut(mir),
            ),
            ("oop_lof".to_string(), lof(mir)),
            ("oop_dfc".to_string(), dfc(mir)),
            ("oop_pbf".to_string(), pbf(mir)),
            ("oop_wms".to_string(), wms_noc_rfs(mir)[0]),
            ("oop_rfs".to_string(), wms_noc_rfs(mir)[1]),
            ("oop_noc".to_string(), wms_noc_rfs(mir)[2]),
            ("pure_function".to_string(), count_pure_function(mir)),
            ("closure".to_string(), count_closure(mir)),
            ("higher_function".to_string(), higher_function(mir)),
        ]
        .into_iter()
        .for_each(|(k, v)| {
            table_data
                .get_mut(&k)
                .unwrap()
                .insert(column_name.clone(), v);
        });
    }

    table_data
}

/// Transform 2-D data into a 2-D tex table.
pub fn write_tex_table<
    X: Display + Ord + Clone,
    Y: Display + Ord + Clone,
    T: Display + Default + Clone,
>(
    data: &TableDatas<X, Y, T>,
    dir: &Path,
    file_name: String,
    caption: String,
) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(dir.join(file_name))?);

    // Sort by axis X and Y
    let data_sorted = sort(data);

    for s in vec![
        "% Please add the following required packages to your document preamble:\n",
        "% \\usepackage{booktabs}\n",
        "% \\usepackage{graphicx}\n",
        "% \\usepackage{xcolor}\n",
        "\\begin{table}[]\n",
        "\\centering\n",
        "\\resizebox{\\textwidth}{!}{%\n",
        "\\begin{tabular}{@{}",
        "l".repeat(data_sorted.len() + 1).as_str(),
        "@{}}\n",
        "\\toprule\n",
    ] {
        writer.write_all(s.as_bytes())?;
    }

    // write first column
    writer.write_all(" & ".as_bytes())?;
    writer.write_all(
        (data_sorted
            .iter()
            .map(|(n, _)| n.to_string().replace('_', "\\_"))
            .collect::<Vec<String>>()
            .join(" & ")
            + " \\\\\\midrule\n")
            .as_bytes(),
    )?;

    // Write each row of data under X_i
    for i in 0..data_sorted.first().unwrap().1.len() {
        writer.write_all(
            (data_sorted
                .first()
                .unwrap()
                .1
                .get(i)
                .unwrap()
                .0
                .to_string()
                .replace('_', "\\_")
                + " & "
                + data_sorted
                    .iter()
                    .map(|(_, y)| y.get(i).unwrap().1.to_string().replace('_', "\\_"))
                    .collect::<Vec<String>>()
                    .join(" & ")
                    .as_str()
                + "\\\\\n")
                .as_bytes(),
        )?;
    }

    for s in vec![
        "\\bottomrule\n",
        "\\end{tabular}%\n",
        "}\n",
        "\\caption{",
        caption.replace('_', "\\_").as_str(),
        "}\n",
        "\\label{tab:",
        caption.as_str(),
        "}\n",
        "\\end{table}\n",
    ] {
        writer.write_all(s.as_bytes())?;
    }

    Ok(())
}

/// row label is of type Y
impl<X: Debug + Clone + Ord, Y: Debug + Clone + Ord + Display, T: Debug + Copy + Into<f64>>
    PcaRawData for TableDatas<X, Y, T>
{
    fn check(&self) -> Result<(), String> {
        let mut feature_number = 0;
        let mut valid = true;
        self.values().for_each(|v| {
            if feature_number == 0 {
                feature_number = v.keys().len();
            } else if feature_number != v.keys().len() {
                valid = false;
            }
        });

        if valid {
            Ok(())
        } else {
            Err(format!(
                "raw data \n {:?} \n failed to be formatted into a matrix",
                self,
            ))
        }
    }

    fn get_row_numbers(&self) -> usize {
        let mut feature_number = 0;
        self.values().for_each(|v| {
            if feature_number == 0 {
                feature_number = v.keys().len();
            }
        });
        feature_number
    }

    fn get_feature_numbers(&self) -> usize {
        self.len()
    }

    fn into_matrix(&self) -> DMatrix<f64> {
        self.check().unwrap();

        let matrix = DMatrix::from_vec(
            self.get_row_numbers(),
            self.get_feature_numbers() as usize,
            into_vec(self),
        );

        matrix
    }

    fn iter_with_row_labels(&self) -> IntoIter<(Vec<f64>, String)> {
        let sorted_table = sort(self);

        let mut row_vecs: Vec<(Vec<f64>, String)> = Vec::new();
        sorted_table
            .first()
            .unwrap()
            .1
            .iter()
            .for_each(|s| row_vecs.push((vec![], s.0.to_string())));

        sorted_table.into_iter().for_each(|(_, y_t)| {
            let mut iter_row_vecs = row_vecs.iter_mut();
            y_t.into_iter()
                .for_each(|(_, t)| iter_row_vecs.next().unwrap().0.push(t.into()));
        });

        row_vecs.into_iter()
    }
}

pub fn into_vec<X: Clone + Ord, Y: Clone + Ord, T: Copy + Into<f64>>(
    table_data: &TableDatas<X, Y, T>,
) -> Vec<f64> {
    let mut data = vec![];

    sort(table_data)
        .iter()
        .for_each(|(_, y_t)| data.append(&mut y_t.iter().map(|(_, t)| (*t).into()).collect()));

    data
}

/// Sort TableData by axis X and Y.
pub fn sort<X: Ord + Clone, Y: Ord + Clone, T: Clone>(
    table_data: &TableDatas<X, Y, T>,
) -> Vec<(X, Vec<(Y, T)>)> {
    let mut data_sorted = table_data
        .into_iter()
        .map(|(x, y_t)| {
            let mut data_sorted = y_t
                .into_iter()
                .map(|(y, t)| (y.clone(), t.clone()))
                .collect::<Vec<(Y, T)>>();
            data_sorted.sort_by(|a, b| a.0.cmp(&b.0));
            (x.clone(), data_sorted)
        })
        .collect::<Vec<(X, Vec<(Y, T)>)>>();
    data_sorted.sort_by(|a, b| a.0.cmp(&b.0));
    data_sorted
}

#[cfg(test)]
mod test_table_data {
    use std::{collections::HashMap, path::PathBuf};

    use crate::{mir_analyze::data::table_data::sort, pca_analysis::pca_data::PcaRawData};

    use super::{generate_benchmark_data, write_tex_table, TableDatas};

    fn assert_file_eq(path: &PathBuf, std_path: &PathBuf) {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open(path).unwrap();
        let std_f = File::open(std_path).unwrap();

        let mut f_reader = BufReader::new(f);
        let mut std_reader = BufReader::new(std_f);

        let mut buf_f = String::new();
        let mut buf_std = String::new();

        while std_reader.read_line(&mut buf_std).unwrap() > 0 {
            f_reader.read_line(&mut buf_f).unwrap();

            assert_eq!(buf_f, buf_std);

            buf_f.clear();
            buf_std.clear();
        }
    }

    fn generate_table_data() -> TableDatas<String, String, i32> {
        vec![
            (
                "column_a".to_string(),
                vec![
                    ("row_a".to_string(), 1),
                    ("row_b".to_string(), 2),
                    ("row_c".to_string(), 3),
                ],
            ),
            (
                "column_b".to_string(),
                vec![
                    ("row_a".to_string(), 4),
                    ("row_b".to_string(), 5),
                    ("row_c".to_string(), 6),
                ],
            ),
        ]
        .into_iter()
        .map(|(x, y)| {
            (
                x,
                y.into_iter()
                    .map(|(a, b)| (a, b))
                    .collect::<HashMap<String, i32>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, i32>>>()
    }

    /// A test for `write_tex_table`.
    ///
    /// Step1. Write a sample table to a tex file.
    ///
    /// Step2. Check the tex file.
    ///
    /// Step3. Clean up.
    #[test]
    fn test_write_tex_table() {
        let tmp_dir = PathBuf::from("test/mir_analyze/writer/test_tex_writer");

        let datas = &generate_table_data();

        write_tex_table(
            datas,
            tmp_dir.as_path(),
            "test_tex_writer.tex".to_string(),
            "test_tex_writer".to_string(),
        )
        .unwrap();

        assert_file_eq(
            &tmp_dir.join("test_tex_writer.tex"),
            &tmp_dir.join("test_tex_writer_std.tex"),
        )
    }

    /// A test for `pca_raw_data_process`.
    ///
    /// Step1. Transfrom TableDatas to a metrix.
    ///
    /// Step2. Verify the metrix.
    #[test]
    fn test_pca_raw_data_process() {
        let data = generate_table_data();

        let sorted_data = sort(&data);

        let matrix = data.into_matrix();

        let mut iter_data = sorted_data.iter();
        let mut iter_matrix = matrix.column_iter();

        while let Some(v) = iter_data.next() {
            if let Some(u) = iter_matrix.next() {
                let mut i = 0;
                assert_eq!(v.1.len(), u.len());
                for (_, x) in &v.1 {
                    assert_eq!(*x as f64, *u.get(i).unwrap());
                    i += 1;
                }
            }
        }
        assert_eq!(iter_matrix.next(), None);
    }

    /// A test for `iter_with_row_labels` implemented by TableData
    ///
    /// Step1. Get the iterator of a table data
    ///
    /// Step2. Verify the iterator.
    #[test]
    fn test_iter_with_row_labels_table_data() {
        assert_eq!(
            generate_table_data()
                .iter_with_row_labels()
                .map(|x| x)
                .collect::<Vec<(Vec<f64>, String)>>(),
            vec![
                (vec![1.0, 4.0], String::from("row_a")),
                (vec![2.0, 5.0], String::from("row_b")),
                (vec![3.0, 6.0], String::from("row_c"))
            ]
        );
    }

    #[test]
    #[ignore]
    fn test_allfiles() {
        let tmp_dir = PathBuf::from("test/mir_analyze/writer/test_tex_writer");
        write_tex_table(
            &generate_benchmark_data(),
            tmp_dir.as_path(),
            "benchmark.tex".to_string(),
            "benchmark".to_string(),
        )
        .unwrap();
    }
}
