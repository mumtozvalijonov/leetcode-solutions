// 1352. Product of the Last K Numbers


struct ProductOfNumbers {
    product_vec: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers { product_vec: Vec::new() }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.product_vec = Vec::new();
            return;
        }
        
        let cur_idx = self.product_vec.len();
        if cur_idx == 0 {
            self.product_vec.push(num);
        } else {
            self.product_vec.push(num * self.product_vec[cur_idx - 1]);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let cur_idx = self.product_vec.len();
        if k > cur_idx {
            return 0;
        } else if k == cur_idx {
            return *self.product_vec.last().unwrap();
        }
        self.product_vec[cur_idx - 1] / self.product_vec[cur_idx - 1 - k]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

fn main() {
    let mut obj = ProductOfNumbers::new();
    obj.add(1);
    obj.add(2);
    obj.add(5);
    obj.add(3);
    let ret_2: i32 = obj.get_product(3);
    println!("{}", ret_2);
}

