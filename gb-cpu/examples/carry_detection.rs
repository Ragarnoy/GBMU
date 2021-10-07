fn borrow() {
    println!("borrow:");
    println!("|  x |  y |  r | ba | bb | bc | b |");
    for x in 0..4 {
        for y in 0..4 {
            let r = (x - y) & 0b11;
            let x2 = x >> 1;
            let y2 = y >> 1;
            let r2 = r >> 1;
            let ba = !x2 & y2;
            let bb = y2 & r2;
            let bc = r2 & !x2;
            let b = ba | bb | bc;

            println!(
                "| {:02b} | {:02b} | {:02b} |  {:b} |  {:b} |  {:b} | {:b} |",
                x, y, r, ba, bb, bc, b
            );
        }
    }
}

fn carry() {
    println!("carry:");
    println!("|  x |  y |  r | ca | cb | c |");
    for x in 0..4 {
        for y in 0..4 {
            let r = (x + y) & 0b11;
            let x2 = x >> 1;
            let y2 = y >> 1;
            let r2 = r >> 1;
            let ca = x2 & y2;
            let cb = (x2 | y2) & !r2;
            let c = ca | cb;

            println!(
                "| {:02b} | {:02b} | {:02b} |  {:b} |  {:b} | {:b} |",
                x, y, r, ca, cb, c
            );
        }
    }
}
fn main() {
    borrow();
    carry();
}
