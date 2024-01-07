pub fn str_add(a: &String, b: &String) -> String {
    str_add_subroutine(a, b, false)
}

fn str_add_subroutine(a: &String, b: &String, carry: bool) -> String {
    if a.chars().count() <= 32 && b.chars().count() <= 32 {
        (a.parse::<u128>().unwrap() + b.parse::<u128>().unwrap() + (carry as u128)).to_string()
    }
    else {
        let a1 = if a.chars().count() > 32 {(&a[0..a.chars().count()-32]).to_string()} else {String::from("0")};
        let a2 = if a.chars().count() > 32 {(&a[a.chars().count()-32..]).to_string()} else {a.to_string()};
        let b1 = if b.chars().count() > 32 {(&b[0..b.chars().count()-32]).to_string()} else {String::from("0")};
        let b2 = if b.chars().count() > 32 {(&b[b.chars().count()-32..]).to_string()} else {b.to_string()};

        let mut new_b: String = str_add_subroutine(&a2, &b2, carry);

        // 58598744820488384738229308546321
        // 65381954416493075065395941912219
        
        // carry treatment
        let mut new_carry = false;
        if new_b.chars().count() > 32 {
            new_carry = true;
            new_b = (&new_b[1..]).to_string();
        }

        format!("{}{:0>32}", str_add_subroutine(&a1, &b1, new_carry), new_b)
    }
}

pub fn str_geq(a: &String, b: &String) -> bool {
    let len_a = a.trim_start_matches("0").chars().count();
    let len_b = b.trim_start_matches("0").chars().count();
    if len_a > len_b {
        true
    }
    else if len_b > len_a {
        false
    }
    else {
        // a and b has equal length after stripping zeros from the left
        a.trim_start_matches("0") >= b.trim_start_matches("0")
    }
}

pub fn str_subtract(a: &String, b: &String) -> String {
    if str_geq(a, b) {
        let res = str_subtract_subroutine(a, b, false).trim_start_matches("0").to_string();
        if res.chars().count() > 0 {res} else {String::from("0")}
    }
    else {
        format!("-{}", str_subtract_subroutine(b, a, false))
    }
}

fn str_subtract_subroutine(a: &String, b: &String, carry: bool) -> String {
    if a.chars().count() <= 32 && b.chars().count() <= 32 {
        if str_geq(a, b) {
            (a.parse::<u128>().unwrap() - b.parse::<u128>().unwrap() - (carry as u128)).to_string()
        }
        else {
            format!("*{}", 
                (u128::pow(10, a.chars().count().try_into().unwrap()) + a.parse::<u128>().unwrap() - b.parse::<u128>().unwrap() 
                    - (carry as u128)).to_string()
            )
        }
    }
    else {
        let a1 = if a.chars().count() > 32 {(&a[0..a.chars().count()-32]).to_string()} else {String::from("0")};
        let a2 = if a.chars().count() > 32 {(&a[a.chars().count()-32..]).to_string()} else {a.to_string()};
        let b1 = if b.chars().count() > 32 {(&b[0..b.chars().count()-32]).to_string()} else {String::from("0")};
        let b2 = if b.chars().count() > 32 {(&b[b.chars().count()-32..]).to_string()} else {b.to_string()};

        let mut new_b: String = str_subtract_subroutine(&a2, &b2, carry);

        // 04233108251307480031023559119268
        // 40386439922305675146246007976965
        
        // carry treatment
        let mut new_carry = false;
        if &new_b[0..1] == "*" {
            new_carry = true;
            new_b = (&new_b[1..]).to_string();
        }
        format!("{}{:0>32}", str_subtract_subroutine(&a1, &b1, new_carry), new_b)
    }
}

// implement Karatsuba multiplication
pub fn str_multiply(a: &String, b: &String) -> String {
    
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a <= 1 && len_b <= 1 {
        (a.parse::<u128>().unwrap() * b.parse::<u128>().unwrap()).to_string()
    }
    else {
        let mut n = std::cmp::max(len_a, len_b);
        if n % 2 == 1 {n += 1};
        let new_a = format!("{:0>n$}", a, n = n);
        let new_b = format!("{:0>n$}", b, n = n);

        let a1 = (&new_a[0..(n / 2)]).to_string();
        let a2 = (&new_a[(n / 2)..]).to_string();
        let b1 = (&new_b[0..(n / 2)]).to_string();
        let b2 = (&new_b[(n / 2)..]).to_string();

        let ac = str_multiply(&a1, &b1);
        let bd = str_multiply(&a2, &b2);
        let mut acbd = str_multiply(&str_add(&a1, &a2), &str_add(&b1, &b2));
        acbd = str_subtract(&str_subtract(&acbd, &ac), &bd);

        str_add(
            &str_add(
                &format!("{}{}", &ac, "0".repeat(n)), &format!("{}{}", &acbd, "0".repeat(n / 2))
            ), &format!("{}", &bd)
        )
    }
}
