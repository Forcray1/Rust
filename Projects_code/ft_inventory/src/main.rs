use std::collections::HashMap;
use std::io;

struct Player {
	name: String,
	inventory: Inventory,
	hp: i32,
}

struct Inventory {
	items: HashMap::<Item, i32>,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Item {
	name: String,
	category: String,
	value: i32,
}

#[derive(Debug)]
struct World {
	items: HashMap::<String, Item>,
}

fn take(inventory: &mut Inventory, item: Item) {
	if inventory.items.contains_key(&item){
		remove(inventory, item);
	} else {
		inventory.items.insert(item, 1);
	}
}

fn remove(inventory: &mut Inventory, item: Item) {
	if let Some(x) = inventory.items.get_mut(&item){
		if *x == 1 {
			inventory.items.remove(&item);
		} else {
			*x -= 1;
		}
	}
}

fn use_item(player: &mut Player, item: Item){
	if item.category == "heal" {
		if player.inventory.items.contains_key(&item){
				remove(&mut player.inventory, item.clone());
				heal(player, item.value);
		} else {
				println!("You don't have enough {}", item.name);
			}
	} else {
		println!("You can't use {}", item.name)
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
	let action = splited.next().unwrap_or("");
	let item_name = splited.next().unwrap_or("");
	let item = match world.items.get(item_name) {
        Some(item) => item.clone(),
        None => {
            println!("Unknown item: {}", item_name);
            return;
        }
    };
	if action == "use"{
		use_item(player, item);
	} else if action == "take" {
		take(&mut player.inventory, item);
		world.items.remove(item_name);
	}
}

fn main() {
    let mut world = World { items: HashMap::new() };
	println!("Enter your name:");
	let mut name_player = String::new();
	io::stdin()
        .read_line(&mut name_player)
        .expect("Failed to read line");
	let inv = HashMap::new();
	let mut player = Player {
		name: name_player,
		inventory: Inventory {items: inv},
		hp: 100
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
	let mut flag: i32 = 0;
	loop {
		print!("\x1B[2J\x1B[1;1H");
		println!("World: {:?}", world);
		println!("You can take an object in the world, or use one you took");
		println!("Or exit by typing 'exit'");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		if input.trim() == "exit" {
			return;
		} else {
			action(input, &mut player, &mut world)
		}
		println!("Press enter");
		let mut _a = String::new();
		io::stdin().read_line(&mut _a);
	}
}
