pub fn ones_standard_input(matrix: (Vec<i8>, String), is_image_input: bool) -> (Vec<f64>, String) {
    let n = matrix.0.len();
    let mut transformed_matrix: (Vec<f64>, String) = (vec![0.0; n], matrix.1);
    let mut min_value = 0.0;
    let max_value = 1.0;

    if !matrix.0.contains(&0) {
        min_value = -1.0;
    }

    for i in 0..n {
        if is_image_input {
            transformed_matrix.0[i] = if matrix.0[i] < 0 {
                max_value
            } else {
                min_value
            };
        } else {
            transformed_matrix.0[i] = if matrix.0[i] > 0 {
                max_value
            } else {
                min_value
            };
        }
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

// Transforma una matriz de acuerdo a la función de transformación
// teniendo valores 1.0 o 0.0 dependiendo de qué condición cumple
pub fn zeros_matrix_transformation(matrix: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let mut transformed_matrix: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        transformed_matrix[i] = if matrix[i] >= 0.0 { 1.0 } else { 0.0 };
    }

    transformed_matrix
}
