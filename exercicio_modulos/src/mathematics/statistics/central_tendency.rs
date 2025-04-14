use std::collections::HashMap;
/// Calcula a média aritmética de um slice de números de ponto flutuante.
///
/// Retorna `None` se o slice estiver vazio.
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(media)` se o slice não estiver vazio, `None` caso contrário.
pub fn media(numeros: &[f64]) -> Option<f64> {
    if numeros.is_empty() {
        None
    } else {
        Some(numeros.iter().sum::<f64>() / numeros.len() as f64)
    }
}

/// Calcula a mediana de um slice de números de ponto flutuante.
///
/// A mediana é o valor que separa a metade superior da metade inferior de um conjunto de dados.
/// Retorna `None` se o slice estiver vazio.
/// **Importante:** Esta função ordena o slice de entrada `numeros` no local.
///
/// # Parameters
/// * `numeros`: Um slice mutável contendo os números. Será ordenado por esta função.
///
/// # Returns
/// `Some(mediana)` se o slice não estiver vazio, `None` caso contrário.
pub fn mediana(numeros: &mut [f64]) -> Option<f64> {
    if numeros.is_empty() {
        return None;
    }
    // Ordena o slice no local para encontrar a mediana
    numeros.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numeros.len() / 2;
    if numeros.len() % 2 == 0 {
        // Média dos dois elementos centrais para tamanho par
        media(&[numeros[mid - 1], numeros[mid]])
    } else {
        // Elemento central para tamanho ímpar
        Some(numeros[mid])
    }
}

/// Encontra a moda (ou modas) de um slice de números de ponto flutuante.
///
/// A moda é o valor que aparece com mais frequência no conjunto de dados.
/// Pode haver mais de uma moda (conjunto multimodal).
/// Retorna `None` se o slice estiver vazio ou se não houver uma moda clara (todos os elementos são únicos ou têm a mesma frequência baixa).
///
/// # Parameters
/// * `numeros`: Um slice contendo os números.
///
/// # Returns
/// `Some(Vec<f64>)` contendo a(s) moda(s) se existir(em), `None` caso contrário.
pub fn mode(numeros: &[f64]) -> Option<Vec<f64>> {
    if numeros.is_empty() {
        return None;
    }
    let mut counts = HashMap::new();
    // Conta a frequência de cada número.
    // Usar representação de string para chaves f64 é uma solução alternativa,
    // pois f64 não implementa Eq/Hash diretamente devido a NaN.
    numeros
        .iter()
        .for_each(|&n| *counts.entry(n.to_string()).or_insert(0) += 1);

    // Encontra a frequência máxima
    let max_count = counts.values().max().copied().unwrap_or(0);

    // Se a contagem máxima for 1 (e houver mais de um elemento), não há moda clara
    if max_count <= 1 && numeros.len() > 1 {
        return None;
    }

    // Coleta todos os números que têm a frequência máxima
    Some(
        counts
            .into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(val_str, _)| {
                val_str
                    .parse::<f64>()
                    .expect("Falha ao converter string de volta para f64")
            }) // Usar expect aqui, pois a string veio de um f64
            .collect(),
    )
}