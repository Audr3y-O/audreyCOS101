fn main() {
    let p:f64 = 520_000_000.0;
    let r:f64 = 10.0;
    let _n:f64 = 5.0;

    //amount
    let a = p*(1.0 + (r / 100.0)).powf(5.0);

    //compound interest
    let ci = a - p;
    println!("Compound Interest is {}", ci);
}
