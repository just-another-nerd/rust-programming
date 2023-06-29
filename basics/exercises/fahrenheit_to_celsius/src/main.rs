fn main() {
    let mut fahr: f32 = 0.0;
    let mut cel: f32;

    while fahr <= 300.0 {
        cel = (5.0/9.0) * (fahr-32.0);
        println!("{}\t{}", fahr, cel);
        fahr += 20.0;
    }
}
