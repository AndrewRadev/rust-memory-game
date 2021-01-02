use memory_game::board::Board;
use memory_game::cards::Card;

#[test]
fn test_finding_a_card_by_coordinates() {
    let mut board = Board::new(2, 2, 1024.0, 768.0);

    board.set_card(0, 0, Card::new("card_0_0"));
    board.set_card(1, 0, Card::new("card_1_0"));
    board.set_card(0, 1, Card::new("card_0_1"));
    board.set_card(1, 1, Card::new("card_1_1"));

    assert_eq!(board.interact_with_card(500.0, 300.0).unwrap().identifier, "card_0_0");
    assert_eq!(board.interact_with_card(600.0, 300.0).unwrap().identifier, "card_0_1");
    assert_eq!(board.interact_with_card(500.0, 400.0).unwrap().identifier, "card_1_0");
    assert_eq!(board.interact_with_card(600.0, 400.0).unwrap().identifier, "card_1_1");
}
