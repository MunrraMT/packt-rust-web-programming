fn main() {
    usando_macros();
}

fn usando_macros() {
    macro_rules! capitalize {
        ($a: expr) => {
            let mut v: Vec<char> = $a.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            $a = v.into_iter().collect();
        };
    }

    let mut x = String::from("test");
    capitalize!(x);
    println!("{}", x);

    macro_rules! dobra {
        ($a: expr) => {
            $a + $a
        };
    }

    dbg!(dobra!(4));
}
