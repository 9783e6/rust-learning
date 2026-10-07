use pretty_table;

mod pretty_table;


fn main() {
    let table = pretty_table::get_pretty_table(
        Some("Test"),
        &vec![String::from("Header1"), String::from("Header2")],
        &vec![
            vec![String::from("1st item!!"), String::from("This is 2nd item")],
            vec![String::from("2nd row!!"), String::from("LAST ITEMMMM!!!")]
        ]);
    println!("{table}");
}
