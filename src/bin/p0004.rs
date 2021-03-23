use marathon592::math_utils;

fn main() {
    // every palindrome is a multiple of 11
    let mut biggest_palindrome = 0;
    for i in (100..=999).rev().step_by(11) {
       for j in (100..=999).rev() {
           let num = i*j;
           if num > biggest_palindrome && math_utils::is_palindrome(num){
               biggest_palindrome = num;
               break;
           }
           else if biggest_palindrome > num {
                break;
           }
       }
    }

    println!("{}", biggest_palindrome);
}
