use memory_game::cards::{Card, CardState, FLIP_DURATION};

#[test]
fn test_card_state_updates() {
    let mut card = Card::new("some_card");
    card.trigger_flip();

    // Starts from the back of the card
    assert!(matches!(card.state, CardState::Back));

    // 1/2 of the way -- still showing its back
    card.update(FLIP_DURATION / 2.0);
    assert!(matches!(card.state, CardState::Back));

    // all the way done with the duration + a bit more: shows its front
    card.update(FLIP_DURATION / 2.0 + 0.1);
    assert!(matches!(card.state, CardState::Front));
}
