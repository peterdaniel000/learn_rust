fn main() {
    println!("Hello, World!");
    another_function(5, 7);
    another_function1();

    let x = five();

    println!("The value of x is: {}", x);

    looping();
    using_for_loop();
    count_down();
}

fn another_function(x: i32, y: u32) {
    println!("another function");
    println!("the value of the parameter is {}, {}", x,y);
}

fn another_function1() {
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn looping() {
    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 6 {
        println!("The value is: {}", a[index]);

        index = index + 1;
    }
}

fn using_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element)
    }
}

fn count_down() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF");
}