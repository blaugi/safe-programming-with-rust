use std::f64::consts::PI; 
/// Calcula a área de um retângulo.
///
/// # Parameters
/// * `largura`: A largura do retângulo.
/// * `altura`: A altura do retângulo.
///
/// # Returns
/// A área do retângulo.
pub fn area_retangulo(largura: f64, altura: f64) -> f64 {
    largura * altura
}

/// Calcula a área de um círculo.
///
/// # Parameters
/// * `raio`: O raio do círculo.
///
/// # Returns
/// A área do círculo.
pub fn area_circulo(raio: f64) -> f64 {
    PI * raio.powi(2)
}

/// Calcula o perímetro de um retângulo.
///
/// # Parameters
/// * `width`: A largura do retângulo.
/// * `height`: A altura do retângulo.
///
/// # Returns
/// O perímetro do retângulo.
pub fn perimeter_rectangle(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

/// Calcula o perímetro (circunferência) de um círculo.
///
/// # Parameters
/// * `radius`: O raio do círculo.
///
/// # Returns
/// A circunferência do círculo.
pub fn perimeter_circle(radius: f64) -> f64 {
    2.0 * PI * radius
}