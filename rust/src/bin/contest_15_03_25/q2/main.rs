mod spreadsheet;

use spreadsheet::Spreadsheet;

fn main() {
    let mut obj = Spreadsheet::new(3);

    let result = obj.get_value("=5+7".to_owned());
    println!("{}", result);

    obj.set_cell("A1".to_owned(), 10);

    let result = obj.get_value("=A1+6".to_owned());
    println!("{}", result);

    obj.set_cell("B2".to_owned(), 15);

    let result = obj.get_value("=A1+B2".to_owned());
    println!("{}", result);

    obj.reset_cell("A1".to_owned());

    let result = obj.get_value("=A1+B2".to_owned());
    println!("{}", result);
}
