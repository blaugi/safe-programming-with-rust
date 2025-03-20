fn main() {
    let mut z = 0.2;
    muda_valor(&mut z);
    z += 1.0;
    println!("Valor de z: {}", z);
}

fn muda_valor(w: &mut f32) {
    *w = 0.5;
}