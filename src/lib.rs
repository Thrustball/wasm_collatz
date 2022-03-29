use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

fn next_collatz(n: u64) -> u64 {
    if n%2 == 0 {
        n / 2
    } else {
        n * 3 + 1
    }
}

#[wasm_bindgen]
pub fn greet(n: u32) -> Vec<u64> {
    // alert(&format!("Hello, {}!", name));
    
    let mut n_c = next_collatz(n.into());
    // let mut count = 0;
    let mut numbers: Vec<u64> = Vec::new();
    numbers.push(n.into());

    'outer: loop {
        if n_c == 1 {
            break 'outer
        }
        numbers.push(n_c);
        n_c = next_collatz(n_c);
        // count += 1;
    }

    numbers.push(1);

    numbers
}

#[wasm_bindgen]
pub fn get_vector() -> Vec<usize> {
    let x = vec![1,2,3,4,5];
    x
}
