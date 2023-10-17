pub mod hopfield {

    pub fn create_weight_matrix(inputs: &Vec<(Vec<f64>, String)>) -> Vec<Vec<f64>> {
        let n: usize = inputs[0].0.len(); // # neuronas
        let m: usize = inputs.len(); // # patrones

        // Se inicializa una matriz cuadratica con valores de 0
        let mut weight_matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i == j || weight_matrix[i][j] != 0.0 {
                    continue;
                }

                let mut value: f64 = 0.0;

                // Se hace la sumatoria de la multiplicación de las neuronas
                // del patron indicado con la variable s
                for s in 0..m {
                    value += inputs[s].0[i] * inputs[s].0[j];
                }

                // Se multiplica la sumatoria por 1/n
                value = (1.0 / (n as f64)) * value;

                // Guardamos el valor en la matriz con su valor espejo
                weight_matrix[i][j] = value;
                weight_matrix[j][i] = value;
            }
        }

        weight_matrix
    }

    // Haciendo uso de la matriz de peso generada con la función
    // create_weight_matrix, genera la matriz que permitirá
    // evaluar si es que converge o asocia con algún dato de entrada
    pub fn evaluation(weigth_matrix: &Vec<Vec<f64>>, x_matrix: &Vec<f64>) -> Vec<f64> {
        let n = x_matrix.len(); // # neuronas
        
        // Toma la matriz de prueba (x_matrix) como U0 y la pasa por la
        // función de transferencia para generar Y0
        let y_matrix = transformation(&x_matrix);

        // Valor resultante de la evaluación
        let mut u_matrix_result: Vec<f64> = vec![0.0; n];

        for i in 0..n {
            let mut value: f64 = 0.0;

            for j in 0..n {
                value += weigth_matrix[i][j] * y_matrix[j];
            }

            u_matrix_result[i] = value;
        }

        // Pasa el resultante de la multiplicación de la matriz peso
        // por la matriz de prueba y lo transforma
        transformation(&u_matrix_result)
    }

    fn transformation(matrix: &Vec<f64>) -> Vec<f64> {
        let n = matrix.len();
        let mut transformed_matrix: Vec<f64> = vec![0.0; n];

        for i in 0..n {
            if matrix[i] >= 0.0 {
                transformed_matrix[i] = 1.0;
            } else {
                transformed_matrix[i] = -1.0;
            }
        }

        transformed_matrix
    }
}
