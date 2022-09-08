use std::cmp;

fn Or(val1: f64, val2: f64) -> f64 { 
    return cmp::max(val1, val2);
}

fn And(val1: f64, val2: f64) -> f64 { 
    return cmp::min(val1, val2);
}

fn Not(val: f64) -> f64 { 
    let not_val = 1 - val;
    return not_val;
}