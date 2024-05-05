use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use super::table_data::TableDatas;

/// Transform 2-D data into a 2-D tex table.
pub fn write_tex_table<X: Display + Ord, Y: Display + Ord, T: Display + Default>(
    data: TableDatas<X, Y, T>,
    dir: &Path,
    file_name: String,
    caption: String,
) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(dir.join(file_name))?);

    // Sort by axis X and Y
    let mut data_sorted = data
        .into_iter()
        .map(|(x, y_t)| {
            let mut data_sorted = y_t
                .into_iter()
                .map(|(y, t)| (y, t))
                .collect::<Vec<(Y, T)>>();
            data_sorted.sort_by(|a, b| a.0.cmp(&b.0));
            (x, data_sorted)
        })
        .collect::<Vec<(X, Vec<(Y, T)>)>>();
    data_sorted.sort_by(|a, b| a.0.cmp(&b.0));

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

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use super::write_tex_table;

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

        let datas = vec![
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
        .collect::<HashMap<String, HashMap<String, i32>>>();

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
}
