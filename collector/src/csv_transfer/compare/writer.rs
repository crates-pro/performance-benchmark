use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use super::stat::Stat;

pub fn write_csv(stat: &Stat, dir: PathBuf) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(
        dir.join(stat.profile.clone() + "_" + stat.metric.as_str() + ".csv"),
    )?);

    let mut data_sorted = stat
        .data
        .iter()
        .map(|(n, v)| (n.clone(), *v))
        .collect::<Vec<(String, f64)>>();
    data_sorted.sort_by(|a, b| a.0.cmp(&b.0));

    writer.write_all(
        data_sorted
            .iter()
            .map(|(n, _)| n.clone())
            .collect::<Vec<String>>()
            .join(",")
            .as_bytes(),
    )?;
    writer.write_all("\n".as_bytes())?;
    writer.write_all(
        data_sorted
            .iter()
            .map(|(_, v)| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .as_bytes(),
    )?;
    writer.write_all("\n".as_bytes())?;

    Ok(())
}

pub fn write_tex(stat: &Stat, dir: PathBuf) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(
        dir.join(stat.profile.clone() + "_" + stat.metric.as_str() + ".tex"),
    )?);

    let mut data_sorted = stat
        .data
        .iter()
        .map(|(n, v)| (n.clone(), *v))
        .collect::<Vec<(String, f64)>>();
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
        "l".repeat(data_sorted.len()).as_str(),
        "@{}}\n",
        "\\toprule\n",
    ] {
        writer.write_all(s.as_bytes())?;
    }

    writer.write_all(
        (data_sorted
            .iter()
            .map(|(n, _)| n.clone())
            .collect::<Vec<String>>()
            .join(" & ")
            + " \\\\\\midrule\n")
            .as_bytes(),
    )?;
    writer.write_all(
        (data_sorted
            .iter()
            .map(|(_, v)| {
                if *v < 0.0 {
                    format!("\\color{{green}}{:.2}\\%", v)
                } else {
                    format!("\\color{{gray}}{:.2}\\%", v)
                }
            })
            .collect::<Vec<String>>()
            .join(" & ")
            + " \\\\\\bottomrule\n")
            .as_bytes(),
    )?;

    for s in vec![
        "\\end{tabular}%\n",
        "}\n",
        "\\caption{Changing rate of",
        format!(" {} ({} profile)", stat.metric, stat.profile).as_str(),
        "}\n",
        "\\label{tab:",
        format!(
            "Changing rate of {} ({} profile)",
            stat.metric, stat.profile
        )
        .as_str(),
        "}\n",
        "\\end{table}\n",
    ] {
        writer.write_all(s.as_bytes())?;
    }

    Ok(())
}
