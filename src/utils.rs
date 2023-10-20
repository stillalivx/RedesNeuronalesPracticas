pub fn ones_standard_input(matrix: (Vec<i8>, String)) -> (Vec<f64>, String) {
    let n = matrix.0.len();
    let mut transformed_matrix: (Vec<f64>, String) = (vec![0.0; n], matrix.1);

    for j in 0..n {
        transformed_matrix.0[j] = if matrix.0[j] > 0 { 1.0 } else { -1.0 };
    }

    transformed_matrix
}

// Transforma una matriz de acuerdo a la función de transformación
pub fn ones_matrix_transformation(matrix: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let mut transformed_matrix: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        transformed_matrix[i] = if matrix[i] >= 0.0 { 1.0 } else { -1.0 };
    }

    transformed_matrix
}