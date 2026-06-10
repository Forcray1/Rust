use std::collections::HashMap;
use std::io;

struct Player {
	name: String,
	inventory: Inventory,
	hp: i32,
}

struct Inventory {
	items: HashMap<Item, i32>,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Item {
	name: String,
	category: String,
	value: i32,
}

#[derive(Debug)]
struct World {
	items: HashMap<String, Item>,
}

fn take(inventory: &mut Inventory, item: Item) {
	// picking up an item adds one (creates the entry at 0 if absent, then +1)
	*inventory.items.entry(item).or_insert(0) += 1;
}

fn remove(inventory: &mut Inventory, item: &Item) {
	if let Some(x) = inventory.items.get_mut(item) {
		if *x <= 1 {
			inventory.items.remove(item);
		} else {
			*x -= 1;
		}
	}
}

fn use_item(player: &mut Player, item: Item) {
	if item.category == "heal" {
		if player.inventory.items.contains_key(&item) {
			remove(&mut player.inventory, &item); // borrow, no clone needed
			heal(player, item.value);
		} else {
			println!("You don't have enough {}", item.name);
		}
	} else {
		println!("You can't use {}", item.name);
	}
}

fn heal(player: &mut Player, i: i32) {
	if player.hp == 100 {
		println!("You already are at max hp");
	} else if player.hp + i > 100 {
		player.hp = 100;
		println!("You're now at {}/100hp", player.hp);
	} else {
		player.hp += i;
		println!("You're now at {}/100hp", player.hp);
	}
}

fn action(line: String, player: &mut Player, world: &mut World) {
	let mut splited = line.split_whitespace();
	let verb = splited.next().unwrap_or("");
	let item_name = splited.next().unwrap_or("");

	match verb {
		"take" => match world.items.get(item_name) {
			Some(item) => {
				let item = item.clone();
				take(&mut player.inventory, item);
				world.items.remove(item_name); // it left the world, it's in the bag now
			}
			None => println!("Unknown item: {}", item_name),
		},
		"use" => {
			// a used item must come from the PLAYER'S inventory, not the world
			let found = player
				.inventory
				.items
				.keys()
				.find(|it| it.name == item_name)
				.cloned();
			match found {
				Some(item) => use_item(player, item),
				None => println!("You don't have a {}", item_name),
			}
		}
		"" => {} // empty line: do nothing
		_ => println!("Unknown action: {}", verb),
	}
}

fn main() {
	let mut world = World { items: HashMap::new() };

	println!("Enter your name:");
	let mut name_player = String::new();
	io::stdin()
		.read_line(&mut name_player)
		.expect("Failed to read line");

	let mut player = Player {
		name: name_player.trim().to_string(), // strip the trailing '\n'
		inventory: Inventory { items: HashMap::new() },
		hp: 100,
	};

	let potion = Item {
		name: String::from("potion"),
		category: String::from("heal"),
		value: 20,
	};
	let sword = Item {
		name: String::from("sword"),
		category: String::from("weapon"),
		value: 0,
	};
	world.items.insert(potion.name.clone(), potion);
	world.items.insert(sword.name.clone(), sword);

	loop {
		print!("\x1B[2J\x1B[1;1H");
		println!("Player: {} | HP: {}/100", player.name, player.hp);
		println!("World: {:?}", world.items.keys().collect::<Vec<_>>());
		println!("Inventory: {:?}", player.inventory.items.keys().collect::<Vec<_>>());
		println!("Type: take <item> | use <item> | exit");

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if input.trim() == "exit" {
			return;
		}
		action(input, &mut player, &mut world);

		println!("Press enter");
		let mut _a = String::new();
		let _ = io::stdin().read_line(&mut _a); // ignore the Result on purpose
	}
}
