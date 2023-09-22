use std::{io, vec};

fn main() {
  let mut n: String = String::new();
  let mut m: String = String::new();

  println!("# Neuronas (N):");
  
  io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

  let n: usize = n.trim().parse().expect("You must provide a integer value");

  println!("# Patrones (M):");

  io::stdin()
    .read_line(&mut m)
    .expect("Failed to read line");

  let m: usize = m.trim().parse().expect("You must provide a integer value");

  println!();

  let mut c: Vec<Vec<f32>> = vec![vec![0.0; n]; m];

  'm_loop: for index_m in 0..m {
    for index_n in 0..n {
      let mut value = String::new();

      println!("Para C{}-{index_n}:", index_m + 1);

      io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

      let value: f32 = value.trim().parse().expect("You must provide a integer value");

      if value != -1.0 && value != 1.0 {
        break 'm_loop;
      }

      c[index_m][index_n] = value;
    }
  }

  let mut result_matrix: Vec<Vec<f32>> = vec![vec![0.0; n]; n];

  for i in 0..n {
    for j in 0..n {
      if i == j || result_matrix[i][j] != 0.0 {
        continue;
      }

      let mut value: f32 = 0.0;

      for s in 0..m {
        value += c[s][i] * c[s][j];
      }

      value = value * (1.0/(n as f32)) ;

      result_matrix[i][j] = value;
      result_matrix[j][i] = value;
    }
  }

  println!();
  println!("{:?}", result_matrix);
  println!();

  let mut confirm_v = String::new();
  let mut x: Vec<f32> = vec![0.0; n];
  let mut u: Vec<Vec<f32>> = vec![vec![0.0; n]];
  let mut y: Vec<Vec<f32>> = vec![vec![0.0; n]];
  let mut converge = false;
  let mut iteration: u8 = 1;

  println!("¿Definir patrón de comprobación? (s/n):");

  io::stdin()
    .read_line(&mut confirm_v)
    .expect("Failed to read line");

  println!();

  if confirm_v.trim() == "s" {
    for index_p in 0..n {
      let mut value = String::new();

      println!("P{index_p}:");

      io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

      let value: f32 = value.trim().parse().expect("You must provide a integer value");

      x[index_p] = value;
    }
  } else {
    x = c[0].clone();
  }

  println!();

  u[0] = x;
  y[0] = fh(u[0].clone());

  while !converge || iteration > 50 {
    let mut actual_u: Vec<f32> = vec![0.0; n];

    for row in 0..n {
      let mut value: f32 = 0.0;
      
      for p_idx in 0..n {
        value += result_matrix[row][p_idx] * y[y.len() - 1][p_idx];
      }
      
      actual_u[row] = value;
    }
    
    u.push(actual_u);
    y.push(fh(u[u.len() - 1].clone()));

    for c_idx in 0..c.len() {
      if compare(&c[c_idx], &y[y.len() - 1]) {
        println!("{:?} = {:?} -> Y({}) se asocia con C{}", c[c_idx], y[y.len() - 1], y.len() - 1, c_idx);
        break;
      }
    }
        
    if compare(&y[y.len() - 2], &y[y.len() - 1]) {
      println!("Y({}){:?} = Y({}){:?} -> Converge", y.len() - 2, y[y.len() - 2], y.len() - 1, y[y.len() - 1]);
      println!();
      println!("Iteraciones: {}", iteration);

      converge = true;
    } else {
      println!("Y({}){:?} = Y({}){:?} -> No converge", y.len() - 2, y[y.len() - 2], y.len() - 1, y[y.len() - 1]);
    }
    
    println!();
    
    iteration += 1;
  }

  if iteration > 50 {
    println!("Entro en bucle");
  }
}

fn fh(p: Vec<f32>) -> Vec<f32> {
  let mut result: Vec<f32> = vec![0.0; p.len()];

  for idx in 0..p.len() {
    if p[idx] >= 0.0 {
      result[idx] = 1.0;
    } else {
      result[idx] = -1.0;
    }
  }

  return result;
}

fn compare(v1: &Vec<f32>, v2: &Vec<f32>) -> bool {
  for index in 0..(v1.len()) {
    if v1[index] != v2[index] {
      return false;
    }
  }

  return true;
}