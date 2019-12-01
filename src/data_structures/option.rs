pub fn option() {
    let x = 3.0;
    let y = 2.0;

    //Option -> some(v) | none
    let result =
        if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(sum) => println!("{}/{} = {}", x, y, sum),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {
        println!("Result = {}", z);
    }

    while let Some(_z) = result {
        println!("hey!, who turned out the lights!");
    }

}
