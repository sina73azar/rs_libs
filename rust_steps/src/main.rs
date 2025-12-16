fn main() {
    println!("Hello, world! changed 2");
        let x = 10;        // immutable
        let mut y = 10;    // mutable
        y = y + 5;

        let z = {
            let a = 2;
            a * 3           // no semicolon => expression value
        };

        println!("x={x}, y={y}, z={z}");
}
