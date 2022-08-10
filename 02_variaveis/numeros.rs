fn main() {
    let salario: i64 = 8328798279837298;

    let signed_8: i8 = -128;
    let unsigned_8: u8 = 255;

    let signed_16: i16 = -32768;
    let unsigned_16: u16 = 65535;

    let signed_32: i32 = -2147483648;
    let unsigned_32: u32 = 4294967295;

    let signed_64: i64 = -922342036854775708;
    let unsigned_64: u64 = 18446744073709551615;

    let signed_128: i128 = -922342036854775708;
    let unsigned_128: u128 = 18446744073709551615;

    println!("{}", salario);
    println!("{}", signed_8);
    println!("{}", unsigned_8);

    println!("{}", signed_16);
    println!("{}", unsigned_16);

    println!("{}", signed_32);
    println!("{}", unsigned_32);

    println!("{}", signed_64);
    println!("{}", unsigned_64);

    println!("{}", signed_128);
    println!("{}", unsigned_128);
}
