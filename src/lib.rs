
use wasm_bindgen::prelude::wasm_bindgen;

fn dividend_finder(dividend: i64) -> String {
    // declare variables
    let mut divisor: i64 = 1;
    let mut multi: i64 = 1;
    let mut result = String::new();

    while dividend != divisor {
        // check divisor and multi; if values
        // are the same break to prevent reprinting
        if divisor != 1 && multi != 1 && divisor == multi {
            break;
        }
        if dividend%divisor == 0 {
            multi = dividend/divisor;
            //println!("{} x {} = {}",divisor, multi, dividend)
            let temp: String = format!("{} x {} = {}\n",divisor, multi, dividend).to_owned();
            let temp_slice: &str = &temp[..];
            result.push_str(temp_slice);
        }
        divisor += 1;
    }
    return result;
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn rustInterface(input: &str) -> String {
    if input.parse::<i64>().is_ok() {
        let number: i64 = input.parse::<i64>().unwrap();
        if number != 0 {
            return dividend_finder(number);
        }
        else{
            return format!("0 x 0 = 0");
        }
    }
    else {
        return String::from("Bad input.");
    }
}