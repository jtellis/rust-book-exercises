fn main() {
    let ten = if true { 10 } else { 9 };

    if ten == 10 {
        println!("ten is 10 👍");
    } else if ten == 9 {
        println!("something ain't right...");
    } else {
        println!("ten isn't 10 and ten isn't 9...");
    }

    let dinos = ['🦖', '🦕'];
    let mut i = 0;
    loop {
        if i == 2 {
            break;
        }
        println!("{} -\"roar!\"", dinos[i]);
        i += 1;
    }

    i = 0;
    while i != 2 {
        println!("{} -\"roar!\"", dinos[i]);
        i += 1;
    }

    for dino in dinos.iter() {
        println!("{} -\"roar!\"", dino);
    }

    for idx in 0..2 {
        println!("{} -\"roar!\"", dinos[idx]);
    }

    for idx in (0..2).rev() {
        println!("{} -\"roar!\"", dinos[idx]);
    }
}
