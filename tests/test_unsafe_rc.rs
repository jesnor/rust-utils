extern crate utils;

use utils::{UnsafeWeakSet, UnsafeRc, UnsafeWeak};
use std::collections::HashSet;

struct Player {
	game: GameWRef,
	health: u32,
	friends: UnsafeWeakSet<Player>
}

struct Game {
	players: HashSet<PlayerRef>
}

type GameRef = UnsafeRc<Game>;
type GameWRef = UnsafeWeak<Game>;
type PlayerRef = UnsafeRc<Player>;
type PlayerWRef = UnsafeWeak<Player>;

fn game() -> GameRef {
	GameRef::new(Game {
		players: HashSet::new()
	})
}

fn add_player(game: &mut GameRef, h: u32) -> PlayerRef {
	let p = PlayerRef::new(Player {
		game: game.downgrade(),
		health: h,
		friends: UnsafeWeakSet::new()
	});
	
	game.players.insert(p.clone());
	p
}

fn remove_player(p: &PlayerRef) {
	if let Some(mut g) = p.game.upgrade() {
		g.players.remove(p);
	}
}

#[test]
fn game_test() {
	let mut g = game();
	let mut p1 = add_player(&mut g, 1);
	let mut p4 = add_player(&mut g, 4);
	
	{
		let mut p2 = add_player(&mut g, 2);
		let p3 = add_player(&mut g, 3);
		
		p1.health = 7;
		p2.health = 8;
		p4.health = 11;
		
		p1.friends.insert(&p2);
		p1.friends.insert(&p3);
		
		remove_player(&p2);
	}
	
	p1.friends.remove(&p4);
	
	for p in &p1.friends {
		println!("{}", p.health)
	}
	
	println!("{}", p1.health)
}
