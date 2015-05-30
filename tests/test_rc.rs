extern crate utils;

use utils::{WeakSet, RcCell, WeakRcCell};
use std::collections::HashSet;

struct Player {
  game: GameWRef,
  health: u32,
  friends: WeakSet<Player>
}

struct Game {
  players: HashSet<PlayerRef>
}

type GameRef = RcCell<Game>;
type GameWRef = WeakRcCell<Game>;
type PlayerRef = RcCell<Player>;
type PlayerWRef = WeakRcCell<Player>;

fn game() -> GameRef {
  GameRef::new(Game {
    players: HashSet::new()
  })
}

fn add_player(game: &mut GameRef, h: u32) -> PlayerRef {
  let p = PlayerRef::new(Player {
    game: game.weak(),
    health: h,
    friends: WeakSet::new()
  });
  
  game.players.insert(p.clone());
  p
}

fn remove_player(p: &PlayerRef) {
  if let Some(mut g) = p.game.strong() {
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

