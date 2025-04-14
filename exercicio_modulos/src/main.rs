mod mathematics;

use mathematics::arithmetic::basic::{
    adicionar,
    subtrair,
    multiplicar,
    dividir,
    modulo
}; 
use mathematics::arithmetic::advanced::{
    power,
    factorial,
    is_prime
};
use mathematics::geometry::coordinates::{
    Ponto,
    translate_point,
    distancia_entre_pontos
};
use mathematics::geometry::shapes_2d::{
    area_circulo,
    area_retangulo,
    perimeter_circle,
    perimeter_rectangle
};
use mathematics::statistics::standard_deviation::{
    variance,
    standard_deviation
};
use mathematics::statistics::central_tendency::{
    media,
    mediana,
    mode
};


/// Ponto de entrada principal da aplicação.
///
/// Demonstra o uso das funções matemáticas definidas neste arquivo.
fn main() {
    println!("--- Demonstração Inicial Expandida (Todas as Funções Usadas) ---");

    // --- Seção de Demonstração: Aritmética ---
    println!("\n** Aritmética **");
    let a_int = 15;
    let b_int = 4;
    let a_float = 15.0;
    let b_float = 4.0;
    let base_pow = 2.0;
    let exp_pow = 10.0;

    println!(
        "Adição: {} + {} = {}",
        a_int,
        b_int,
        adicionar(a_int, b_int)
    );
    println!(
        "Subtração: {} - {} = {}",
        a_int,
        b_int,
        subtrair(a_int, b_int)
    );
    println!(
        "Multiplicação: {} * {} = {}",
        a_int,
        b_int,
        multiplicar(a_int, b_int)
    );
    match dividir(a_float, b_float) {
        Some(res) => println!("Divisão: {} / {} = {}", a_float, b_float, res),
        None => println!("Divisão por zero!"),
    }
    match dividir(a_float, 0.0) {
        // Teste de divisão por zero
        Some(res) => println!("Divisão: {} / {} = {}", a_float, 0.0, res), // Não deve acontecer
        None => println!("Tentativa de Divisão por zero!"),
    }
    println!("Módulo: {} % {} = {}", a_int, b_int, modulo(a_int, b_int));
    println!(
        "Potência: {}^{} = {}",
        base_pow,
        exp_pow,
        power(base_pow, exp_pow)
    );
    println!("Fatorial de 5: {}", factorial(5));
    println!("Verificação Primo (17): {}", is_prime(17));
    println!("Verificação Primo (18): {}", is_prime(18));

    // --- Seção de Demonstração: Geometria ---
    println!("\n** Geometria **");
    let ponto_a = Ponto { x: 1.0, y: 1.0 };
    let ponto_b = Ponto { x: 4.0, y: 5.0 };
    let ponto_c = translate_point(&ponto_a, 3.0, 4.0); // Ponto A transladado
    let largura_rect = 5.0;
    let altura_rect = 3.0;
    let raio_circ = 2.5;

    println!("Ponto A: {:?}", ponto_a);
    println!("Ponto B: {:?}", ponto_b);
    println!("Ponto C (A transladado): {:?}", ponto_c);
    println!(
        "Distância (A -> B): {:.2}",
        distancia_entre_pontos(&ponto_a, &ponto_b)
    );
    println!(
        "Área Retângulo ({:.1}x{:.1}): {:.2}",
        largura_rect,
        altura_rect,
        area_retangulo(largura_rect, altura_rect)
    );
    println!(
        "Perímetro Retângulo ({:.1}x{:.1}): {:.2}",
        largura_rect,
        altura_rect,
        perimeter_rectangle(largura_rect, altura_rect)
    );
    println!(
        "Área Círculo (raio {:.1}): {:.2}",
        raio_circ,
        area_circulo(raio_circ)
    );
    println!(
        "Perímetro Círculo (raio {:.1}): {:.2}",
        raio_circ,
        perimeter_circle(raio_circ)
    );

    // --- Seção de Demonstração: Estatística ---
    println!("\n** Estatística **");
    // 'dados_stats' precisa ser mutável para a função 'mediana' que ordena no local.
    let mut dados_stats = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0, 1.0];
    println!("Dados originais: {:?}", dados_stats);

    if let Some(m) = media(&dados_stats) {
        println!("Média: {:.2}", m);
    }
    // A chamada a 'mediana' modifica 'dados_stats' ordenando-o.
    if let Some(med) = mediana(&mut dados_stats) {
        println!("Mediana: {:.2}", med);
    }
    println!(
        "Dados após cálculo da mediana (ordenados): {:?}",
        dados_stats
    );
    if let Some(var) = variance(&dados_stats) {
        // Usa os dados já ordenados
        println!("Variância: {:.2}", var);
    }
    if let Some(dev_std) = standard_deviation(&dados_stats) {
        // Usa os dados já ordenados
        println!("Desvio Padrão: {:.2}", dev_std);
    }
    if let Some(moda_vals) = mode(&dados_stats) {
        // Usa os dados já ordenados
        println!("Moda: {:?}", moda_vals);
    }

    println!("\n--- Fim da Demonstração ---");
}
