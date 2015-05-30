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
	assert!(p1.get().health == 1);
	assert!(p1.strong_count() == 2);
	assert!(p1.weak_count() == 0);
	
	let p4 = add_player(&g, 4);
	assert!(p4.get().health == 4);
	assert!(p4.strong_count() == 2);
	assert!(p4.weak_count() == 0);
	
	{
		let p2 = add_player(&g, 2);
		let p3 = add_player(&g, 3);
		
		p1.get().health = 7;
		p2.get().health = 8;
		p4.get().health = 11;
		assert!(p1.get().health == 7);
		assert!(p2.get().health == 8);
		assert!(p4.get().health == 11);
		
		p1.get().friends.insert(&p2);
		p1.get().friends.insert(&p3);
		p1.get().friends.insert(&p4);
		assert!(p2.strong_count() == 2);
		assert!(p2.weak_count() == 1);
		assert!(p3.strong_count() == 2);
		assert!(p3.weak_count() == 1);
		assert!(p4.strong_count() == 2);
		assert!(p4.weak_count() == 1);
		
		remove_player(&p2);
		assert!(p2.strong_count() == 1);
		assert!(p2.weak_count() == 1);
	}
	
	p1.get().friends.remove(&p4);
	assert!(p4.strong_count() == 2);
	assert!(p4.weak_count() == 0);
	
	let mut size = 0;
	
	for p in &p1.get().friends {
  	assert!(p.get().health == 3);
  	assert!(p.strong_count() == 2);
  	assert!(p.weak_count() == 1);
		size += 1
	}
	
	assert!(size == 1);
}
