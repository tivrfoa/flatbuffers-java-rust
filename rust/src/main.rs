extern crate flatbuffers;

mod my_game;

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("../flatbuffer.data").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("file reading failed");

    let monster = my_game::sample::get_root_as_monster(&buf[..]);
    println!("{}", monster.hp()); // `80`
    println!("{}", monster.mana()); // default value of `150`
    println!("{:?}", monster.name()); // Some("MyMonster")
    // println!("{:?}", monster.weapons());
    // println!("{:?}", monster.equipped_as_weapon());
}
