fn main() {
    let q: i8 = 1;
    let w: i16 = 2;
    let e: i32 = 3;
    let r: i64 = 4;
    let t: i128 = 5;
    let y: u8 = 6;
    let u: u16 = 7;
    let i: u32= 8;
    let o: u64= 9;
    let p: u128= 10;

    println!("valor={}",q);
    println!("valor={}",w);
    println!("valor={}",e);
    println!("valor={}",r);
    println!("valor={}",t);
    println!("valor={}",y);
    println!("valor={}",u);
    println!("valor={}",i);
    println!("valor={}",o);
    println!("valor={}",p);

    println!("{}", format!("{:#x}", q));
    println!("{}", format!("{:#x}", w));
    println!("{}", format!("{:#x}", e));
    println!("{}", format!("{:#x}", r));
    println!("{}", format!("{:#x}", t));
    println!("{}", format!("{:#x}", y));
    println!("{}", format!("{:#x}", u));
    println!("{}", format!("{:#x}", i));
    println!("{}", format!("{:#x}", o));
    println!("{}", format!("{:#x}", p));

    println!("{}", format!("{:#b}", q));
    println!("{}", format!("{:#b}", w));
    println!("{}", format!("{:#b}", e));
    println!("{}", format!("{:#b}", r));
    println!("{}", format!("{:#b}", t));
    println!("{}", format!("{:#b}", y));
    println!("{}", format!("{:#b}", u));
    println!("{}", format!("{:#b}", i));
    println!("{}", format!("{:#b}", o));
    println!("{}", format!("{:#b}", p));

    let min_i8 = std::i8::MIN;
    let max_i8 = std::i8::MAX;
    let min_i16 = std::i16::MIN;
    let max_i16 = std::i16::MAX;
    let min_i32 = std::i32::MIN;
    let max_i32 = std::i32::MAX;
    let min_i64 = std::i64::MIN;
    let max_i64 = std::i64::MAX;
    let min_i128 = std::i128::MIN;
    let max_i128 = std::i128::MAX;

    let min_u8 = std::u8::MIN;
    let max_u8 = std::u8::MAX;
    let min_u16 = std::u16::MIN;
    let max_u16 = std::u16::MAX;
    let min_u32 = std::u32::MIN;
    let max_u32 = std::u32::MAX;
    let min_u64 = std::u64::MIN;
    let max_u64 = std::u64::MAX;
    let min_u128 = std::u128::MIN;
    let max_u128 = std::u128::MAX;

    println!("i8: min = {}, max = {}", min_i8, max_i8);
    println!("i16: min = {}, max = {}", min_i16, max_i16);
    println!("i32: min = {}, max = {}", min_i32, max_i32);
    println!("i64: min = {}, max = {}", min_i64, max_i64);
    println!("i128: min = {}, max = {}", min_i128, max_i128);

    println!("u8: min = {}, max = {}", min_u8, max_u8);
    println!("u16: min = {}, max = {}", min_u16, max_u16);
    println!("u32: min = {}, max = {}", min_u32, max_u32);
    println!("u64: min = {}, max = {}", min_u64, max_u64);
    println!("u128: min = {}, max = {}", min_u128, max_u128);

   // let overflow_i16: i16 = 32768;
    let a: i32 = 10;
    let b: i32 = 5;
    let c: u32 = 15;
    let d: u32 = 3;

    let addition_i32 = a + b;
    let subtraction_i32 = a - b;
    let multiplication_i32 = a * b;
    let division_i32 = a / b;

    let addition_u32 = c + d;
    let subtraction_u32 = c - d;
    let multiplication_u32 = c * d;
    let division_u32 = c / d;

    // Step 11: Display the results of these operations in the console
    println!("i32 addition: {} + {} = {}", a, b, addition_i32);
    println!("i32 subtraction: {} - {} = {}", a, b, subtraction_i32);
    println!("i32 multiplication: {} * {} = {}", a, b, multiplication_i32);
    println!("i32 division: {} / {} = {}", a, b, division_i32);

    println!("u32 addition: {} + {} = {}", c, d, addition_u32);
    println!("u32 subtraction: {} - {} = {}", c, d, subtraction_u32);
    println!("u32 multiplication: {} * {} = {}", c, d, multiplication_u32);
    println!("u32 division: {} / {} = {}", c, d, division_u32);


    // em rust no modo release ocorre wrapping underflow/overflow, ou seja, 
    // o valor ultrapassa o limite do espaço em memória
    // ex: i8 : 11111111 + 1 =  o 1 está fora do espaço alocado e é descartado->1_00000000 alterando seu valor
    // no modo debug ocorre um erro em runtime  
}
 