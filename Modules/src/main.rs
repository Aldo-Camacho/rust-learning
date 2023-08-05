mod restaurant;
mod math;
use crate::restaurant::order_food;
use crate::math::add;
fn main(){
    order_food();
    println!("{}", add(5, 10));
}