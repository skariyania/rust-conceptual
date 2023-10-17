fn divide_ex(ex: i32, divider: i32) -> i32 {
    return ex / divider;
}

fn compute(ex: i32, history: &mut [i32]) -> i32 {
    mut result: i32 = ex;
    let counter = 0;
    let ex_mod = ex % 2;
    if ex_mod == 0 {
        result = divide_ex(ex, 2);
        // history.push(result);
        compute(ex, history);
    } 
    else {
        result = (ex * 3) + 1;
        // history.push(result);
        compute(ex, history);
        
    }
    counter+=1;
    if counter > 50 {
        return result;
    } else {
        println!("empty else");
        return 0;
    }
    
        
}

fn main() {
    // compute(4, &[])
    println!(",w");
}