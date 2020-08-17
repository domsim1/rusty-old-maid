mod fiftytwo;

use fiftytwo::{Table};
use fiftytwo::old_maid;

fn main() {
    let mut table = Table::new();
    table.deck.remove(11); // Remove Queen
    table.shuffle_deck();
    table.deal(4, 15);
    table.scrap.push(Vec::new());

    for i in 0..table.hands.len() {
        old_maid::remove_all_pairs(table.hands.get_mut(i).unwrap());
    }

    let mut index = 0;
    let mut round_count = 1;
    loop {
        println!("--- Round {} ---", round_count);
        for i in 0..table.hands.len() {
            println!("p{}: {}", i+1, table.hand_to_string(table.hands.get(i).unwrap()));
        }
        println!("---------------");
        round_count += 1;
        if let Some(maid) = old_maid::check_for_old_maid(&table.hands) {
            println!("The maid is p{}", maid+1);
            break;
        }

        old_maid::take_turn(index, table.hands.as_mut());
        index += 1;
        if index > table.hands.len()-1 {
            index = 0;
        }
    }
}
