extern crate utils;

use utils::{WeakCellSet, RcCell, WeakCell};
use std::collections::HashSet;

struct Player {
	game: GameWRef,
	health: u32,
	friends: WeakCellSet<Player>
}

struct Game {
	players: HashSet<PlayerRef>
}

type GameRef = RcCell<Game>;
type GameWRef = WeakCell<Game>;
type PlayerRef = RcCell<Player>;
type PlayerWRef = WeakCell<Player>;

fn game() -> GameRef {
	GameRef::new(Game {
		players: HashSet::new()
	})
}

fn add_player(game: &GameRef, h: u32) -> PlayerRef {
	let p = PlayerRef::new(Player {
		game: game.downgrade(),
		health: h,
		friends: WeakCellSet::new()
	});
	
	game.get().players.insert(p.clone());
	p
}

fn remove_player(p: &PlayerRef) {
	if let Some(g) = p.get().game.upgrade() {
		g.get().players.remove(p);
	}
}

#[test]
fn game_test() {
	let g = game();
	let p1 = add_player(&g, 1);
	let p4 = add_player(&g, 4);
	
	{
		let p2 = add_player(&g, 2);
		let p3 = add_player(&g, 3);
		
		p1.get().health = 7;
		p2.get().health = 8;
		p4.get().health = 11;
		
		p1.get().friends.insert(&p2);
		p1.get().friends.insert(&p3);
		
		remove_player(&p2);
	}
	
	p1.get().friends.remove(&p4);
	
	for p in &p1.get().friends {
		println!("{}", p.get().health)
	}
	
  {let p = p1.get(); println!("{}", p.health) }
}
