use std::collections::HashMap;

pub struct Spreadsheet {
    cells: HashMap<(char, i32), i32>,
    rows: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Spreadsheet {
            cells: HashMap::with_capacity(26 * rows as usize),
            rows,
        }
    }

    fn _cell_key_from_ref(cell: String) -> (char, i32) {
        let col = cell.chars().nth(0).unwrap();
        let row = cell[1..].parse().unwrap();
        (col, row)
    }

    fn _from_formula_slice(&self, slice: String) -> i32 {
        if slice.chars().nth(0).unwrap().is_alphabetic() {
            let key = Self::_cell_key_from_ref(slice);
            return *self.cells.get(&key).unwrap_or(&0);
        }
        slice.parse().unwrap()
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let key = Self::_cell_key_from_ref(cell);
        if key.1 <= self.rows {
            self.cells.insert(key, value);
        }
    }

    pub fn reset_cell(&mut self, cell: String) {
        let key = Self::_cell_key_from_ref(cell);
        if key.1 <= self.rows {
            self.cells.remove(&key);
        }
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let mut formula = formula[1..].split('+');
        let first = self._from_formula_slice(formula.next().unwrap().to_owned());
        let second = self._from_formula_slice(formula.next().unwrap().to_owned());
        first + second
    }
}
