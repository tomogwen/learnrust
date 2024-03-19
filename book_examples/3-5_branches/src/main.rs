// branches rustbook

fn main() {
    let mut num = 3;
    while num != 0 { 
        println!("{num}");
        num -= 1;
    }
    println!("rockets brev\n");

    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("num1 {}", a[idx]);
        idx += 1;
    }

    for element in a {
        println!("num2 {}", element);
    }

    for index in (0..5).rev(){
        println!("{index}: {}", a[index]);
    }

}


fn branching () {
    let number = 12;

    if number % 4 == 0 { 
        println!{"mod 4"};
    } else if number % 3 == 0 {
        println!("mod 3");
    } else {
        println!("condition false");
    }

    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");
}

fn loops () {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("AGAIN");
        if count < 5 {
            count += 1;
            continue;
        } else {
            'count_inside: loop {
                count += 1;
                if count == 10 {
                    break 'counting_up;
                }
            }
        }
    }

    let mut count2: i32 = 0;

    let result = loop {
        count2 += 1;
        if count2 == 10 {
            break count2*23;
        }
    };
    println!("{result}")
}
