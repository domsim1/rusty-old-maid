use super::Card;
use rand::Rng;

pub fn remove_all_pairs(hand: &mut Vec<Card>) {
    hand.clone_from(&remove_pairs(0, hand.to_vec()));
}

fn remove_pairs(index: usize, hand: Vec<Card>) -> Vec<Card> {
    let mut new_hand = hand.clone();
    if new_hand.len() > index {
        let selected_card = new_hand.remove(index);
        for (i, x) in new_hand.clone().into_iter().enumerate() {
            if x.value == selected_card.value {
                new_hand.remove(i);
                return remove_pairs(index, new_hand);
            }
        }
        remove_pairs(index+1, hand)
    } else {
        new_hand
    }
}

pub fn add_card_to_hand(card: Card, hand: &mut Vec<Card>) {
    for (i, x) in hand.into_iter().enumerate() {
        if x.value == card.value {
            hand.remove(i);
            return;
        }
    }
    hand.push(card);
}

pub fn check_for_old_maid(hands: &Vec<Vec<Card>>) -> Option<usize> {
    let mut old_maid_index: Option<usize> = None;
    for (i, x) in hands.into_iter().enumerate() {
        if x.len() == 1 {
            if x[0].value == 12 {
                old_maid_index = Some(i);
            } else {
                return None;
            }
        } else if x.len() > 1 {
            return None;
        }
    }
    old_maid_index
}

pub fn take_turn(player_index: usize, hands: &mut Vec<Vec<Card>>) {
    if hands.get(player_index).unwrap().len() == 0 {
        return;
    }
    let mut rng = rand::thread_rng();
    let player_count = hands.len()-1;
    let mut index: usize = player_index + 1;
    loop {
        if index > player_count {
            index = 0;
        }
        if index == player_index {
            return;
        }
        if let Some(target) = hands.get_mut(index) {
            if target.len() == 0 {
                index += 1;
                continue;
            } else {
                let card = target.remove(rng.gen_range(0, target.len()));
                add_card_to_hand(card, hands.get_mut(player_index).unwrap());
                return;
            }
        }
    }

}
