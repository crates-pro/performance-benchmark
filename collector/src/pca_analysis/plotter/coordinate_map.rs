use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use nalgebra::DVector;

use crate::mir_analyze::data::table_data::*;
use crate::pca_analysis::pca_data::*;

#[test]
fn test_allfiles() {
    //println!("{:?}", generate_benchmark_data());
    fs::create_dir("test/pca_map").unwrap();
    let tmp_dir = PathBuf::from("test/pca_map");
    draw_coordinate_map_2d(
        &get_principle_components(&generate_benchmark_data()),
        &generate_benchmark_data(),
        &tmp_dir,
    );
    fs::remove_dir_all("test/pca_map").unwrap();
}

/// `draw_coordinate_map_2d` plots the dataset,
/// shows the relationship between specific data
/// and principle_components.
///
/// parameter `principle_components` is a group of eigenvectors.
pub fn draw_coordinate_map_2d(
    principle_components: &Vec<Vec<f64>>,
    data_set: &dyn PcaRawData,
    out_dir: &PathBuf,
) -> anyhow::Result<()> {
    // Step1. For each principle_component pair (u, v), do:
    //
    // Step2. Calculate the projection of each data on u and v,
    //        the projection value is the coordinate of the data.
    //
    // Step3. Generate coordinate map according to the coordinates.
    let feature_vectors: Vec<DVector<f64>> = principle_components
        .iter()
        .map(|u| DVector::from_vec(u.clone()))
        .collect();

    let mut iter_feature_vectors = feature_vectors.iter();
    let mut pc_u = 1;
    let mut pc_v = 2;

    feature_vectors.iter().for_each(|u| {
        let mut iter_v = iter_feature_vectors.clone();
        iter_feature_vectors.next();

        while let Some(v) = iter_v.next() {
            let coordinates: Vec<(Coordinate, String)> = data_set
                .iter_with_row_labels()
                .map(|(data, label)| (get_coordinate_2d(&DVector::from_vec(data), u, v), label))
                .collect();

            draw(coordinates, pc_u, pc_v, out_dir).unwrap();
            pc_v += 1;
        }

        pc_u += 1;
        pc_v = pc_u + 1;
    });

    Ok(())
}

pub type Coordinate = (f64, f64);

fn get_coordinate_2d(
    data: &DVector<f64>,
    feature_x: &DVector<f64>,
    feature_y: &DVector<f64>,
) -> Coordinate {
    (data.dot(feature_x), data.dot(feature_y))
}

fn draw(
    coordinates: Vec<(Coordinate, String)>,
    pc_x: u32,
    pc_y: u32,
    out_dir: &PathBuf,
) -> anyhow::Result<()> {
    let mut scatter = Command::new("python");
    scatter
        .arg("src/pca_analysis/plotter/scatter.py")
        .arg(
            coordinates
                .into_iter()
                .map(|((x, y), label)| format!("{},{},{};", x.to_string(), y.to_string(), label))
                .collect::<String>(),
        )
        .arg(pc_x.to_string())
        .arg(pc_y.to_string())
        .arg(out_dir.join(format!("PC{}vsPC{}.png", pc_x, pc_y)));

    scatter.stdout(Stdio::inherit());
    scatter.stderr(Stdio::inherit());

    scatter.spawn()?.wait()?;

    Ok(())
}

#[cfg(test)]
mod coordinate_map_test {
    use std::{collections::HashMap, fs, path::PathBuf};

    use nalgebra::DVector;

    use crate::mir_analyze::data::table_data::TableDatas;

    use super::{draw_coordinate_map_2d, get_coordinate_2d};

    fn generate_table_data() -> TableDatas<String, String, i32> {
        vec![
            (
                "feature_a".to_string(),
                vec![
                    ("row_a".to_string(), 10),
                    ("row_b".to_string(), 1),
                    ("row_c".to_string(), 1),
                    ("row_d".to_string(), 10),
                    ("row_e".to_string(), 1),
                    ("row_f".to_string(), 1),
                ],
            ),
            (
                "feature_b".to_string(),
                vec![
                    ("row_a".to_string(), 1),
                    ("row_b".to_string(), 100),
                    ("row_c".to_string(), 1),
                    ("row_d".to_string(), 1),
                    ("row_e".to_string(), 100),
                    ("row_f".to_string(), 1),
                ],
            ),
            (
                "feature_c".to_string(),
                vec![
                    ("row_a".to_string(), 1),
                    ("row_b".to_string(), 1),
                    ("row_c".to_string(), 900),
                    ("row_d".to_string(), 1),
                    ("row_e".to_string(), 1),
                    ("row_f".to_string(), 900),
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

    /// A test for `get_coordinate_2d`.
    ///
    /// Step1. Create datas and features.
    ///
    /// Step2. Calculate coordinate of each data.
    ///
    /// Step3. Verify the result.
    #[test]
    fn test_get_coordinate_2d() {
        let datas = vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 9.0],
        ];
        let feature_x = DVector::from_vec(vec![
            0.8164965809277261,
            -0.4082482904638633,
            -0.408248290463863,
        ]);
        let feature_y = DVector::from_vec(vec![0.0, -0.7071067811865475, 0.7071067811865477]);
        let mut results = vec![
            (-3.674234614174768, 2.1213203435596437),
            (-3.674234614174768, 2.121320343559644),
            (-3.6742346141747686, 2.1213203435596446),
        ]
        .into_iter();

        datas.into_iter().for_each(|d| {
            assert_eq!(
                get_coordinate_2d(&DVector::from(d), &feature_x, &feature_y),
                results.next().unwrap()
            );
        });
    }

    /// A test for `draw_coordinate_map_2d`.
    ///
    /// Step1. Draw PCA charts.
    ///
    /// Step3. Remove the chart files.

    #[test]
    fn test_draw_coordinate_map_2d() {
        fs::create_dir("test/draw_coordinate_map_2d").unwrap();
        draw_coordinate_map_2d(
            &vec![
                vec![0.8164965809277261, -0.4082482904638633, -0.408248290463863],
                vec![0.0, -0.7071067811865475, 0.7071067811865477],
                vec![
                    -0.5773502691896258,
                    -0.5773502691896262,
                    -0.5773502691896258,
                ],
            ],
            &generate_table_data(),
            &PathBuf::from("test/draw_coordinate_map_2d"),
        )
        .unwrap();

        fs::remove_dir_all("test/draw_coordinate_map_2d").unwrap();
    }
}
