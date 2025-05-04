use std::ops::{Add, Mul};
use num_traits::FromPrimitive;

pub trait Calculavel<T> {
    fn area(&self) -> T;
    fn perimetro(&self) -> T;
}

struct Circulo<T> {
    raio: T,
}

struct Retangulo<T> {
    largura: T,
    altura: T,
}

impl<T> Calculavel<T> for Retangulo<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialOrd + std::fmt::Display,
{
    fn area(&self) -> T {
        self.largura * self.altura
    }
    fn perimetro(&self) -> T {
        let dois = T::from_f64(2.0).expect("Falha ao converter 2.0 para T");
        dois * (self.largura + self.altura)
    }
}

impl<T> Calculavel<T> for Circulo<T>
where
    T: Copy + Mul<Output = T> + FromPrimitive + PartialOrd + std::fmt::Display,
{
    fn area(&self) -> T {
        let pi = T::from_f64(std::f64::consts::PI).expect("Falha ao converter PI para T");
        self.raio * self.raio * pi
    }
    fn perimetro(&self) -> T {
        let dois = T::from_f64(2.0).expect("Falha ao converter 2.0 para T");
        let pi = T::from_f64(std::f64::consts::PI).expect("Falha ao converter PI para T");
        dois * pi * self.raio
    } 
}

// Métodos construtores com validação
impl<T> Circulo<T> {
    fn new(raio: T) -> Option<Self>
    where
        T: PartialOrd + FromPrimitive,
    {
        let zero = T::from_f64(0.0).expect("Falha ao converter 0.0 para T");
        if raio < zero {
            None
        } else {
            Some(Circulo { raio })
        }
    }
}

impl<T> Retangulo<T> {
    fn new(largura: T, altura: T) -> Option<Self>
    where
        T: PartialOrd + FromPrimitive,
    {
        let zero = T::from_f64(0.0).expect("Falha ao converter 0.0 para T");
        if largura < zero || altura < zero {
            None
        } else {
            Some(Retangulo { largura, altura })
        }
    }
}

fn main() {
    // Testes com f64
    let circulo_f64 = Circulo::<f64>::new(5.0).expect("Dimensão inválida");
    println!("Círculo de raio {:.2}:", 5.0);
    println!("Área: {:.2}", circulo_f64.area());
    println!("Perímetro: {:.2}", circulo_f64.perimetro());

    let retangulo_f64 = Retangulo::<f64>::new(4.0, 6.0).expect("Dimensão inválida");
    println!("\nRetângulo de largura {:.2} e altura {:.2}:", 4.0, 6.0);
    println!("Área: {:.2}", retangulo_f64.area());
    println!("Perímetro: {:.2}", retangulo_f64.perimetro());

    // Testes com f32
    let circulo_f32 = Circulo::<f32>::new(5.0).expect("Dimensão inválida");
    println!("\nCírculo de raio {:.2} (f32):", 5.0);
    println!("Área: {:.2}", circulo_f32.area());
    println!("Perímetro: {:.2}", circulo_f32.perimetro());

    let retangulo_f32 = Retangulo::<f32>::new(4.0, 6.0).expect("Dimensão inválida");
    println!("\nRetângulo de largura {:.2} e altura {:.2} (f32):", 4.0, 6.0);
    println!("Área: {:.2}", retangulo_f32.area());
    println!("Perímetro: {:.2}", retangulo_f32.perimetro());
}
