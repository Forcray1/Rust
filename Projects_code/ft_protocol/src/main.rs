use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct World {
	room: String,
	items: Vec<String>,
}

struct Player {
	name: String,
	inventory: HashMap<String, i32>,
	hp: i32,
	status: String,
	group: String,
}

#[allow(dead_code)]
enum Command {
	Connect,
	Look,
	Move,
	Chat,
	Take,
	Drop,
	Inventory,
	Talk,
	Attack,
	Status,
	Quest,
	Quests,
	Who,
	Group,
	Quit,
}

fn execute(action: &str, args: &str, world: &World, player: &Player) {
	match action {
		"CONNECT" => println!("S: {args} connected"),
		"LOOK" => println!("S: room={:?} items={:?}", world.room, world.items),
		"MOVE" => println!("S: You moved to {args}"),
		"CHAT" => println!("S: CHAT: {args}"),
		"TAKE" => println!("S: {args} has been added to your inventory"),
		"DROP" => println!("S: You dropped {args}"),
		"INVENTORY" => println!("S: Inventory: {:?}", player.inventory),
		"TALK" => println!("S: You're talking to {args}"),
		"ATTACK" => println!("S: You attacked {args}"),
		"STATUS" => println!("S: {}: {}hp, {}", player.name, player.hp, player.status),
		"QUEST" => println!("S: Your current quest is: [...]"),
		"QUESTS" => println!("S: Quests you are registered to: [...]"),
		"WHO" => println!("S: People in your room: [...]"),
		"GROUP" => println!("S: Your group: {}", player.group),
		"QUIT" => println!("S: {} DISCONNECTED", player.name),
		_ => println!("S: ERR 400 unknown command"),
	}
}

fn parser(s: &str, player: &Player, world: &World) {
	let (instruction, args) = match s.split_once(' ') {
		Some((i, a)) => (i.trim(), a.trim()),
		None => (s.trim(), ""),
	};
	execute(instruction, args, world, player);
}

fn main() {
	let player = Player {
		name: String::from("test"),
		inventory: HashMap::new(),
		hp: 100,
		status: String::from("AFK"),
		group: String::from("None"),
	};
	let world = World {
		room: String::from("Hall"),
		items: Vec::new(),
	};
	loop{
		println!("C: ");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		parser(&input, &player, &world);
	}
}
