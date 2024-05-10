use std::{
    fmt::{Debug, Display},
    vec::IntoIter,
};

use collector::mir_analyze::data::table_data::{into_vec, sort, TableDatas};
use nalgebra::{DMatrix, SymmetricEigen};

/// `PcaRawData`
pub trait PcaRawData {
    // `check` will verify whether the raw data can be transformed into a matrix or not.
    fn check(&self) -> Result<(), String> {
        Ok(())
    }

    /// `get_row_numbers` will return the number of rows
    fn get_row_numbers(&self) -> usize;

    /// `get_row_labels` will return the label of each row.
    /// The order of labels will be the same with the order
    /// of rows in raw data.
    fn iter_with_row_labels(&self) -> IntoIter<(Vec<f64>, String)>;

    /// `get_feature_numbers` will return the number of features
    fn get_feature_numbers(&self) -> usize;

    /// `into_arrays` will transform raw data into an matrix with data grouped by different features.
    /// The order of features shall remain the same with that of the raw data.
    /// Each column in the matrix represents a group of data with one specific feature.
    fn into_matrix(&self) -> DMatrix<f64>;
}

/// `pca_analysis` calculates the principal components of a matrix.
///
/// Step1. Transfer raw data into a matrix
///
/// Step2. Normalize the matrix
///
/// Step3. Calculate the convariance of the matrix
///
/// Step4. Calculate the eigenvalue and eigenvectors of the matrix
///
/// Step5. Sort the eigenvectors by the corresponding eigenvalue.
pub fn get_principle_components(raw_data: &dyn PcaRawData, pc_num: u32) -> Vec<Vec<f64>> {
    let data = raw_data.into_matrix();

    // normalize
    let normalized_matrix = normalize(&data);

    // covariance
    let cov_matrix =
        (normalized_matrix.transpose() * &normalized_matrix) / ((data.nrows() - 1) as f64);

    // eigenvalue eecomposition
    let eigen = SymmetricEigen::new(cov_matrix);
    let eigen_values = eigen.eigenvalues;
    let eigen_vectors = eigen.eigenvectors;

    // sort eigenvectors
    let mut iter_eigen_vectors = eigen_vectors.column_iter();

    let mut eigen_value_vectors: Vec<(f64, Vec<f64>)> = eigen_values
        .iter()
        .map(|eigen_value| {
            (
                *eigen_value,
                iter_eigen_vectors
                    .next()
                    .unwrap()
                    .iter()
                    .map(|x| *x)
                    .collect(),
            )
        })
        .collect();
    eigen_value_vectors.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());

    if eigen_value_vectors.len() > pc_num as usize {
        eigen_value_vectors[0..pc_num as usize]
            .into_iter()
            .map(|(_, v)| v.clone())
            .collect()
    } else {
        eigen_value_vectors.into_iter().map(|(_, v)| v).collect()
    }
}

/// normalize a matrix
fn normalize(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let mean = matrix.row_mean();

    let mut mean_iter = mean.iter();
    let deviations = matrix
        .column_iter()
        .map(|c| {
            let mut d: f64 = 0.0;
            let m = mean_iter.next().unwrap();
            c.iter().for_each(|x| {
                d += (x - m).powf(2.0);
            });
            d /= matrix.nrows() as f64;
            d.sqrt()
        })
        .collect::<Vec<f64>>();

    let mut matrix = matrix.clone();

    let mut mean_iter = mean.iter();
    let mut deviation_iter = deviations.iter();

    matrix.column_iter_mut().for_each(|mut c| {
        let m = *mean_iter.next().unwrap();
        c.iter_mut().for_each(|x| *x -= m);
        c /= *deviation_iter.next().unwrap();
    });

    matrix
}

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

#[cfg(test)]
mod cfg_test {
    use std::collections::HashMap;

    use nalgebra::DMatrix;

    use collector::mir_analyze::data::table_data::TableDatas;

    use super::{get_principle_components, normalize};

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

    /// A test for `pca`.
    ///
    /// Step1. Create a matrix.
    ///
    /// Step2. Calculate PCA.
    ///
    /// Step3. Verify the matrix.
    #[test]
    fn test_pca() {
        assert_eq!(
            get_principle_components(&generate_table_data(), 100),
            vec![
                vec![0.8164965809277261, -0.4082482904638633, -0.408248290463863],
                vec![0.0, -0.7071067811865475, 0.7071067811865477],
                vec![
                    -0.5773502691896258,
                    -0.5773502691896262,
                    -0.5773502691896258
                ],
            ]
        );
    }

    /// A test for `matrix normalize`.
    ///
    /// Step1. Create a matrix.
    ///
    /// Step2. Normalize the matrix.
    ///
    /// Step3. Verify the result.
    #[test]
    fn test_normalize() {
        let matrix = DMatrix::from_vec(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let result = DMatrix::from_vec(
            3,
            2,
            vec![
                -1.224744871391589,
                0.0,
                1.224744871391589,
                -1.224744871391589,
                0.0,
                1.224744871391589,
            ],
        );

        assert_eq!(normalize(&matrix), result);
    }
}
