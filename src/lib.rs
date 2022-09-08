pub mod desk {
    enum Pieces {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }
    struct Player {
        name: String,
    }

    pub fn print_desk() {
        let mut desk = standard_desk();
        for i in desk {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
    }

    fn start_game() {
        let mut white_team = [""];
    }

    fn standard_desk() -> Vec<Vec<String>> {
        let mut desk: Vec<Vec<String>> = Vec::with_capacity(10);
        let mut letters = vec![" ", "|", "a", "b", "c", "d", "e", "f", "g", "h"];
        let mut border = vec!["-", "|", "-", "-", "-", "-", "-", "-", "-", "-"];
        let mut white_color = false;

        for i in 0..8 {
            let mut desk_row: Vec<String> = Vec::with_capacity(10);
            desk_row.push(String::from((8 - i).to_string()));
            desk_row.push(String::from("|"));
            white_color = !white_color;
            for j in 1..9 {
                match white_color {
                    true => {
                        desk_row.push(String::from("0"));
                        white_color = false;
                    }
                    _ => {
                        desk_row.push(String::from("*"));
                        white_color = true;
                    }
                }
            }
            desk.push(desk_row);
        }
        desk.push(border.iter().map(|&a| String::from(a)).collect());
        desk.push(letters.iter().map(|&a| String::from(a)).collect());

        desk
    }
}
