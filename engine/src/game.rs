use std::fmt::{Display, Formatter};

pub struct Game {}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line1 = " -------------------------------------";
        let line2 = "/                                      \\";
        let line3 = "|                                      |";
        let line4 = "|                                      |";
        let line5 = "\\                                     /";
        let line6 = " -------------------------------------";

        write!(f, "{}\n{}\n{}\n{}\n{}\n{}", line1, line2, line3, line4, line5, line6)
    }
}


