fn main() {
    let x = 10;
    let y = -5;
    let subtracted = subtract(x, y);

    let plus_one = {
        let x = subtracted;
        x + 1 /* note no semicolon at end of expression */
    };
}

fn subtract(x: i32, y: i32) -> i32 {
    add(x, -y)
}


fn add(x: i32, y: i32) -> i32 {
    x + y
}
