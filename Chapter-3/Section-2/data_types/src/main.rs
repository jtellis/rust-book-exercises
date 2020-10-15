fn main() {
    let x = 2030u32;
    let y = 0b111111_00100;
    let z: u8  = 255;
    let a = 100.0;
    let b: bool = true;
    let c: (u32, u32, u8, f64, bool) = (x, y, z, a, b);
    println!("c = {:?}", c);
}
