use super::central_tendency::media;
/// Calcula o desvio padrão populacional de um slice de números de ponto flutuante.
///
/// O desvio padrão é a raiz quadrada da variância.
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(desvio_padrao)` se o slice não estiver vazio, `None` caso contrário.
pub fn standard_deviation(numeros: &[f64]) -> Option<f64> {
    variance(numeros).map(|v| v.sqrt())
}

/// Calcula a variância populacional de um slice de números de ponto flutuante.
///
/// A variância mede a dispersão dos dados em relação à média.
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(variancia)` se o slice não estiver vazio, `None` caso contrário.
pub fn variance(numeros: &[f64]) -> Option<f64> {
    // Calcula a média dos quadrados das diferenças em relação à média
    media(numeros).map(|m| {
        media(
            &numeros
                .iter()
                .map(|x| (x - m).powi(2))
                .collect::<Vec<f64>>(),
        )
        .unwrap_or(0.0)
    })
}