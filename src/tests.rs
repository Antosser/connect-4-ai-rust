#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn player_eq() {
        assert!(Cell::Player == Cell::Player);
        assert!(Cell::AI == Cell::AI);
        assert!(Cell::None == Cell::None);
    }

    #[test]
    fn horizontal_winner() {
        for player in [Cell::Player, Cell::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][4] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[5][4] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[6][4] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[7][4] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn vertical_winner() {
        for player in [Cell::Player, Cell::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][4] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][5] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][6] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][7] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn no_winner() {
        assert_eq!(Board::new().get_winner(), Cell::None);
    }

    #[test]
    fn right_up_diagonal_winner() {
        for player in [Cell::Player, Cell::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Cell::None);
            board.data[2][8] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[3][7] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][6] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[5][5] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn left_up_diagonal_winner() {
        for player in [Cell::Player, Cell::AI] {
            let mut board = Board::new();

            assert_eq!(board.get_winner(), Cell::None);
            board.data[2][3] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[3][4] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[4][5] = player;
            assert_eq!(board.get_winner(), Cell::None);
            board.data[5][6] = player;
            assert_eq!(board.get_winner(), player);
        }
    }

    #[test]
    fn ai_detect_horizontal_win() {
        for player in [Player::Player, Player::AI] {
            for _ in 0..2 {
                let mut board = Board::new();

                board.place(player, 5);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 3);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 4);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 2);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                assert_ne!(board.get_winner(), player.to_cell());
            }
        }
    }

    #[test]
    fn ai_detect_vertical_win() {
        for player in [Player::Player, Player::AI] {
            for _ in 0..2 {
                let mut board = Board::new();

                board.place(player, 5);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 3);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 4);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                board.place(player, 2);
                board.place(
                    player.to_opposite(),
                    board.calculate_best_move(Player::AI, 3) as usize,
                );
                assert_ne!(board.get_winner(), player.to_cell());
            }
        }
    }
}
