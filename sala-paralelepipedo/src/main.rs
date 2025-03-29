fn main() {
}

struct Sala{
    profundidade: f64,
    largura : f64,
    altura : f64
}

impl Sala{
    fn area_parede_frontal(&self) -> f64{
        self.altura * self.largura
    }

    fn area_parede_lateral(&self) -> f64{
        self.profundidade * self.altura
    }

    fn area_piso(&self) -> f64{
        self.profundidade * self.largura
    }

    fn volume(&self) -> f64{
        self.profundidade * self.largura * self.altura
    }

    fn soma_volume(&self, sala : &Sala) -> f64{
        self.volume() + sala.volume()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_piso() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_piso(), 20.0);
    }

    #[test]
    fn test_area_parede_frontal() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_parede_frontal(), 15.0);
    }

    #[test]
    fn test_area_parede_lateral() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_parede_lateral(), 12.0);
    }

    #[test]
    fn test_volume() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.volume(), 60.0);
    }

    #[test]
    fn test_zero_dimensions() {
        let sala = Sala { largura: 0.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_piso(), 0.0);
        assert_eq!(sala.area_parede_frontal(), 0.0);
        assert_eq!(sala.area_parede_lateral(), 12.0);
        assert_eq!(sala.volume(), 0.0);
    }

    #[test]
    fn test_large_dimensions() {
        let sala = Sala { largura: 1e6, profundidade: 1e6, altura: 1e6 };
        assert_eq!(sala.area_piso(), 1e12);
        assert_eq!(sala.area_parede_frontal(), 1e12);
        assert_eq!(sala.area_parede_lateral(), 1e12);
        assert_eq!(sala.volume(), 1e18);
    }


    #[test]
    fn teste_volume_addition(){
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        let sala2 = Sala { largura: 4.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.soma_volume(&sala2), 108.0 )
    }
}