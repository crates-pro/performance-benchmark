use nalgebra::{DMatrix, SymmetricEigen};

pub trait PcaRawData {
    // `check` will verify whether the raw data can be transformed into a matrix or not.
    fn check(&self) -> Result<(), String> {
        Ok(())
    }

    /// `get_row_numbers` will return the number of rows
    fn get_row_numbers(&self) -> usize;

    /// `get_feature_numbers` will return the number of features
    fn get_feature_numbers(&self) -> usize;

    /// `into_arrays` will transform raw data into an matrix with data grouped by different features.
    /// The order of features shall remain the same with raw data.
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
pub fn get_principal_comonents(raw_data: &dyn PcaRawData) -> Vec<Vec<f64>> {
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

    eigen_value_vectors.into_iter().map(|(_, v)| v).collect()
}

/// normalize a matrix
fn normalize(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let mean = matrix.row_mean();

    let mut mean_iter = mean.iter();
    let deviations = matrix
        .column_iter()
        .map(|c| {
            let mut d = 0.0;
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

#[cfg(test)]
mod cfg_test {
    use std::collections::HashMap;

    use nalgebra::DMatrix;

    use crate::mir_analyze::data::table_data::TableDatas;

    use super::{normalize, get_principal_comonents};

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
            get_principal_comonents(&generate_table_data()),
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
