// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";
    /*
        这里因为 ? 的行为是遇到 Err 直接返回 Err，众所周知，返回的行为必须在函数上
        ，所以使用 ? 时，相关的代码必须在一个函数/方法里，而函数也应该返回 
        Result 类型，如果在 main() 中直接使用则同样需要把 main() 
        改为 main() -> Result.. 否则编译时会给你这个：

     */
    //let cost = total_cost(pretend_user_input)?;
    let cost = total_cost(pretend_user_input);
    match cost {
        Ok(cost) => { 
            if cost > tokens {
            println!("You can't afford that many!");
        } else {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }},
        Err(_) => {println!("NO!!!!");}
    };
   
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
