/// Adiciona dois inteiros.
///
/// # Parameters
/// * `a`: O primeiro operando.
/// * `b`: O segundo operando.
///
/// # Returns
/// A soma de `a` e `b`.
pub fn adicionar(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtrai o segundo inteiro do primeiro.
///
/// # Parameters
/// * `a`: O minuendo.
/// * `b`: O subtraendo.
///
/// # Returns
/// A diferença entre `a` e `b`.
pub fn subtrair(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplica dois inteiros.
///
/// # Parameters
/// * `a`: O primeiro fator.
/// * `b`: O segundo fator.
///
/// # Returns
/// O produto de `a` e `b`.
pub fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

/// Divide dois números de ponto flutuante.
///
/// Retorna `None` se o divisor `b` for zero.
///
/// # Parameters
/// * `a`: O dividendo.
/// * `b`: O divisor.
///
/// # Returns
/// `Some(resultado)` se a divisão for bem-sucedida, `None` caso contrário.
pub fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

/// Calcula o resto da divisão inteira.
///
/// # Parameters
/// * `a`: O dividendo.
/// * `b`: O divisor.
///
/// # Returns
/// O resto de `a` dividido por `b`.
pub fn modulo(a: i32, b: i32) -> i32 {
    a % b
}
