use crate::{linked_list::ListNode, solution::Solution};

mod linked_list;
mod solution;

#[cfg(test)]
mod tests;

fn main() {
    let result = Solution::get_decimal_value(ListNode::build_linked_list(Vec::from([1, 0, 1])));
    println!("{}", result);
}
