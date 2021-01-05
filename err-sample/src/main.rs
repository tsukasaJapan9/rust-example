fn main() {
    // result_test();
    // option_test();
    //unwrap_test();
    odd_test();
}

fn odd_test() {
    let odd = |n| -> Result<i32, String> {
        if n % 2 == 1 {
            Ok(n)
        } else {
            Err(format!("{} is not odd", n))
        }
    };

    let double = |n| -> Result<i32, String> {
        let n = odd(n)?;
        Ok(n * 2)
    };

    for n in 0 .. 4 {
        println!("number: {}", n);
        println!("double result: {:?}", double(n))
    }
}


/*
fn unwrap_test() {
    let result: Result<i32, String> = Ok(10);
    println!("{:?}", result.unwrap());

    // panic
    // let result: Result<i32, String> = Err("Oops".to_string());
    // println!("{:?}", result.unwrap());

    let result: Result<i32, String> = Err("Oops".to_string());
    println!("{:?}", result.unwrap_or(111));

    let result: Result<i32, String> = Err("Oops".to_string());
    println!("{:?}", result.unwrap_or_else(|_| 222));

    // let option: Option<i32> = Some(2);
    // println!("{:?}", option.unwrap());

    // panic
    // let option: Option<i32> = None;
    // println!("{:?}", option.unwrap());
}
*/

/*
fn result_test() {
    let result = [Ok(100), Err("Oops!!")];
    for r in result.iter() {
        // println!("Result: {:?}", r);

        let d = match r {
            Ok(v) => v * 2,
            Err(_) => 0,
        };
        println!("result: {}", d);

        if let Ok(v) = r {
            println!("OK: {}", v)
        }
    }
}

fn option_test() {
    let option = [Some(-1), None];

    for o in option.iter() {
        let d = match o {
            Some(v) => v * 2,
            None => 0,
        };
        println!("result: {}", d);

        if let Some(v) = o {
            println!("OK: {}", v);
        }
    }
}
*/
