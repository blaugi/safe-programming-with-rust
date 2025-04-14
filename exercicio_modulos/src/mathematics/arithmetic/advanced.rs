/// Calcula a potência de um número base elevado a um expoente.
///
/// Utiliza números de ponto flutuante para permitir expoentes não inteiros.
///
/// # Parameters
/// * `base`: O número base.
/// * `exp`: O expoente.
///
/// # Returns
/// `base` elevado à potência `exp`.
pub fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

/// Calcula o fatorial de um número inteiro não negativo.
///
/// O fatorial de 0 é 1.
///
/// # Parameters
/// * `n`: O número para o qual calcular o fatorial (deve ser não negativo).
///
/// # Returns
/// O fatorial de `n`. Pode ocorrer overflow para valores grandes de `n`.
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Verifica se um número inteiro não negativo é primo.
///
/// Um número primo é um número natural maior que 1 que não tem divisores
/// positivos além de 1 e ele mesmo.
///
/// # Parameters
/// * `n`: O número a ser verificado.
///
/// # Returns
/// `true` se `n` for primo, `false` caso contrário.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    // Otimização: verificar divisores apenas até a raiz quadrada de n
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}