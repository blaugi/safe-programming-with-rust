/// Representa um ponto no espaço 2D com coordenadas de ponto flutuante.
#[derive(Debug, Clone, Copy)]
pub struct Ponto {
    /// A coordenada no eixo X.
    pub x: f64,
    /// A coordenada no eixo Y.
    pub y: f64,
}

/// Calcula a distância euclidiana entre dois pontos no espaço 2D.
///
/// # Parameters
/// * `p1`: O primeiro ponto.
/// * `p2`: O segundo ponto.
///
/// # Returns
/// A distância entre `p1` e `p2`.
pub fn distancia_entre_pontos(p1: &Ponto, p2: &Ponto) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}



/// Translada um ponto por um vetor de deslocamento (dx, dy).
///
/// # Parameters
/// * `p`: O ponto original.
/// * `dx`: O deslocamento no eixo X.
/// * `dy`: O deslocamento no eixo Y.
///
/// # Returns
/// Um novo `Ponto` resultante da translação.
pub fn translate_point(p: &Ponto, dx: f64, dy: f64) -> Ponto {
    Ponto {
        x: p.x + dx,
        y: p.y + dy,
    }
}