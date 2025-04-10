mod math;

use math::algebra::{add, subtract};
use math::algebra::linear_algebra::Vector;

fn main() {
    let r1 = add(3,4);
    println!("Resultado soma = {}", r1);
    let r1 = subtract(5, 3);
    println!("Resultado subtração = {}", r1);

    let v = Vector{
        components: vec![10.0, 20.0, 30.0],
    };

    let d = Vector::dimension(&v);
    let m = Vector::magnitude(&v);

    println!("Dimension = {}", d);
    println!("Manitude = {}", m);
}
