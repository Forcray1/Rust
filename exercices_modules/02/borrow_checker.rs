struct Inventory {
    items: Vec<String>,
}

fn add_item(inv: &mut Inventory, item: String) {
    inv.items.push(item);
}

fn describe(inv: &Inventory) -> String {
    format!("{} items", inv.items.len())
}

fn main() {
    let mut inv = Inventory { items: vec![] };
    add_item(&mut inv, String::from("sword"));
    add_item(&mut inv, String::from("shield"));
    println!("{}", describe(&inv));
    println!("{}", describe(&inv));   // expected: "2 items" twice
}