use std::{fs, io, vec};

use image::GenericImageView;


fn main() {
    let mut img_inputs: Vec<(Vec<f64>, String)> = vec![];
    let mut img_dimensions: (u32, u32) = (0, 0);

    for file in fs::read_dir("test/images/6x6-patterns").unwrap() {
        let file = file.expect("Error al leer el archivo");
        let img = image::open(file.path()).unwrap();

        if img_dimensions == (0, 0) {
            img_dimensions = img.dimensions();
        } else if img_dimensions != img.dimensions() {
            println!("Las imágenes deben de tener la misma dimensión");
            return;
        }

        let pattern: Vec<f64> = fh_image(img.to_luma8().into_vec());
        let name = file.file_name().into_string().unwrap();

        img_inputs.push((pattern, name));
    }

    let w = generate_w(&img_inputs);
    let mut x_pattern = String::new();

    println!("\nEscoger imagen de prueba:");

    for (i, img) in img_inputs.iter().enumerate() {
        println!("{i} - {}", img.1);
    }

    io::stdin().read_line(&mut x_pattern).unwrap();

    let x_pattern: usize = x_pattern.trim().parse().expect("Indice incorrecto");

    verification(w, &img_inputs, &img_inputs[x_pattern]);
}


fn generate_w(img_inputs: &Vec<(Vec<f64>, String)>) -> Vec<Vec<f64>> {
    let n = img_inputs[0].0.len();
    let m = img_inputs.len();

    println!("# Neuronas (N): {n}");
    println!("# Patrones (M): {m}");

    let mut w: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j || w[i][j] != 0.0 {
                continue;
            }

            let mut value: f64 = 0.0;

            for s in 0..m {
                value += img_inputs[s].0[i] * img_inputs[s].0[j];
            }

            value = value * (1.0 / (n as f64));

            w[i][j] = value;
            w[j][i] = value;
        }
    }

    return w;
}


fn verification(
    w: Vec<Vec<f64>>,
    img_inputs: &Vec<(Vec<f64>, String)>,
    img_v: &(Vec<f64>, String),
) {
    let n = img_v.0.len();
    let mut u: Vec<Vec<f64>> = vec![vec![0.0; n]];
    let mut y: Vec<Vec<f64>> = vec![vec![0.0; n]];
    let mut converge = false;
    let mut iteration: u32 = 1;

    u[0] = (&img_v.0).clone();
    y[0] = fh(u[0].clone());

    while !converge && iteration <= 50 {
        let mut actual_u: Vec<f64> = vec![0.0; n];

        for row in 0..n {
            let mut value: f64 = 0.0;

            for p_idx in 0..n {
                value += w[row][p_idx] * y[y.len() - 1][p_idx];
            }

            actual_u[row] = value;
        }

        u.push(actual_u);
        y.push(fh(u[u.len() - 1].clone()));

        for c_idx in 0..img_inputs.len() {
            if compare(&img_inputs[c_idx].0, &y[y.len() - 1]) {
                println!("Y({}) se asocia con {}", y.len() - 1, img_inputs[c_idx].1);
                break;
            }
        }

        if compare(&y[y.len() - 2], &y[y.len() - 1]) {
            println!("Y({}) = Y({}) -> Converge", y.len() - 2, y.len() - 1);
            println!();
            println!("Iteraciones: {}", iteration);

            converge = true;
        } else {
            println!("Y({}) = Y({}) -> No converge", y.len() - 2, y.len() - 1);
        }

        println!();

        iteration += 1;
    }

    if iteration > 50 {
        println!("Entro en bucle");
    }
}


fn fh_image(p: Vec<u8>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; p.len()];

    for idx in 0..p.len() {
        if p[idx] > 0 {
            result[idx] = 1.0;
        } else {
            result[idx] = -1.0;
        }
    }

    return result;
}

fn fh(p: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; p.len()];

    for idx in 0..p.len() {
        if p[idx] >= 0.0 {
            result[idx] = 1.0;
        } else {
            result[idx] = -1.0;
        }
    }

    return result;
}


fn compare(v1: &Vec<f64>, v2: &Vec<f64>) -> bool {
    for index in 0..(v1.len()) {        
        if v1[index] != v2[index] {
            return false;
        }
    }

    return true;
}
