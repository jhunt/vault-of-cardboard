// parser: parse vif

pub struct Line {
    pub quantity: u32,
    pub set: String,
    pub oracle: String,
    pub gvars: Vec<String>,
    pub lvars: Vec<(String, String)>,
}

impl Line {
    pub fn id(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!("{} {}", self.set, self.oracle));
        if self.gvars.len() + self.lvars.len() > 0 {
            s.push_str(" |");
            for gvar in &self.gvars {
                s.push_str(&format!(" {}", gvar));
            }
            for (k, v) in &self.lvars {
                s.push_str(&format!(" ({}: {})", k, v));
            }
        }
        s
    }
    pub fn as_cdif_string(&self) -> String {
        String::from(&format!("{}x {}", self.quantity, self.id()))
    }
}

pub fn parse_line(line: &str) -> Option<Line> {
    let mut state = 0;
    let mut data = Line {
        quantity: 0,
        set: String::new(),
        oracle: String::new(),
        lvars: Vec::new(),
        gvars: Vec::new(),
    };
    let mut gvar = String::new();
    let mut lkey = String::new();
    let mut lval = String::new();

    for c in line.chars() {
        match (state, c) {
            (0, ' ') => (),
            (0, '0'..='9') => {
                state = 1;
                data.quantity = c.to_digit(10)?;
            }
            (0, '#') => state = 14, // 0 -> 14 [label="'#']

            (1, '0'..='9') => {
                data.quantity = data.quantity * 10 + c.to_digit(10)?;
            }
            (1, 'x') => state = 2, // 1 -> 2 [label="'x'"]
            (1, ' ') => state = 3, // 1 -> 3 [label="WS"]

            (2, ' ') => state = 3, // 2 -> 3 [label="WS"]

            (3, ' ') => (), // 3 -> 3 [label="WS"]
            (3, 'a'..='z') | (3, 'A'..='Z') | (3, '0'..='9') => {
                data.set.push(c);
                state = 4;
            }

            (4, 'a'..='z') | (4, 'A'..='Z') | (4, '0'..='9') => {
                data.set.push(c);
            }
            (4, ' ') => state = 5, // 4 -> 5 [label="WS"]

            (5, ' ') => (), // 5 -> 5 [label="WS"]
            (5, 'a'..='z') | (5, 'A'..='Z') | (5, '0'..='9') => {
                data.oracle.push(c);
                state = 6;
            } // 5 -> 6 [label="LETTER"]

            (6, '#') => state = 14,        // 6 -> 14 [label="'#'"]
            (6, '|') => state = 7,         // 6 -> 7 [label="'|'"]
            (6, _) => data.oracle.push(c), // 6 -> 6 [label="*"]

            (7, ' ') => (), // 7 -> 7 [label="WS"]
            (7, 'a'..='z') | (7, 'A'..='Z') | (7, '0'..='9') => {
                gvar.push(c);
                state = 8;
            } // 7 -> 8 [label="LETTER"]
            (7, '(') => state = 9, // 7 -> 9 [label="'('"]
            (7, '#') => state = 14, // 7 -> 14 [label="'#'"]

            (8, ' ') => {
                data.gvars.push(gvar);
                gvar = String::new();
                state = 7;
            } // 8 -> 7 [label="WS"]
            (8, 'a'..='z') | (8, 'A'..='Z') | (8, '0'..='9') => {
                gvar.push(c);
            } // 8 -> 8 [label="LETTER"]

            (9, 'a'..='z') | (9, 'A'..='Z') | (9, '0'..='9') => {
                lkey = String::new();
                lkey.push(c);
                state = 10;
            } // 9 -> 10 [label="LETTER"]

            (10, 'a'..='z') | (10, 'A'..='Z') | (10, '0'..='9') => {
                lkey.push(c);
            } // 10 -> 10 [label="LETTER"]
            (10, ':') => {
                state = 11;
            } // 10 -> 11 [label="':'"]

            (11, ' ') => (), // 11 -> 11 [label="WS"]
            (11, 'a'..='z') | (11, 'A'..='Z') | (11, '0'..='9') => {
                lval = String::new();
                lval.push(c);
                state = 12;
            } // 11 -> 12 [label="LETTER"]

            (12, ')') => {
                data.lvars.push((lkey.to_string(), lval.to_string()));
                state = 13;
            } // 12 -> 13 [label="')'"]
            (12, _) => {
                lval.push(c);
            }

            (13, ' ') => state = 7,  // 13 -> 7 [label="WS"]
            (13, '#') => state = 14, // 13 -> 14 [label="'#'"]

            (14, _) => (), // 14 -> 14 [label="*"]

            (_, _) => return None, // syntax error
        }
    }
    data.oracle = data.oracle.trim_end().to_string();
    match state {
        8 => {
            data.gvars.push(gvar);
            Some(data)
        }
        0 | 6 | 7 | 13 | 14 => Some(data),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_should_format_line_ids_properly() {
        let mut line = Line {
            quantity: 2,
            set: "MIR".to_string(),
            oracle: "Swamp".to_string(),
            gvars: vec![],
            lvars: vec![],
        };
        assert_eq!("MIR Swamp", line.id());
        assert_eq!("2x MIR Swamp", line.as_cdif_string());

        line.oracle = "Femeref Scouts".to_string();
        assert_eq!("MIR Femeref Scouts", line.id());
        assert_eq!("2x MIR Femeref Scouts", line.as_cdif_string());

        line.gvars.push("NM".to_string());
        line.gvars.push("promo".to_string());
        assert_eq!("MIR Femeref Scouts | NM promo", line.id());
        assert_eq!("2x MIR Femeref Scouts | NM promo", line.as_cdif_string());

        line.quantity = 1;
        line.lvars
            .push(("signed".to_string(), "SDCC 2019".to_string()));
        assert_eq!(
            "MIR Femeref Scouts | NM promo (signed: SDCC 2019)",
            line.id()
        );
        assert_eq!(
            "1x MIR Femeref Scouts | NM promo (signed: SDCC 2019)",
            line.as_cdif_string()
        );

        line.gvars = vec![];
        assert_eq!("MIR Femeref Scouts | (signed: SDCC 2019)", line.id());
        assert_eq!(
            "1x MIR Femeref Scouts | (signed: SDCC 2019)",
            line.as_cdif_string()
        );

        line.gvars.push("NM".to_string());
        line.lvars
            .push(("misprint".to_string(), "double-flip".to_string()));
        assert_eq!(
            "MIR Femeref Scouts | NM (signed: SDCC 2019) (misprint: double-flip)",
            line.id()
        );
        assert_eq!(
            "1x MIR Femeref Scouts | NM (signed: SDCC 2019) (misprint: double-flip)",
            line.as_cdif_string()
        );
    }

    macro_rules! assert_parses {
        ($line:expr, $qty:expr, $set:expr, $ora:expr) => {
            let c = parse_line($line);
            assert!(c.is_some(), format!("The line '{}' should parse", $line));

            let c = c.unwrap();
            assert_eq!(
                $qty, c.quantity,
                "Quantity of '{}' should be {}",
                $line, $qty
            );
            assert_eq!($set, c.set, "Set of '{}' should be '{}'", $line, $set);
            assert_eq!(
                $ora, c.oracle,
                "Oracle card of '{}' should be '{}'",
                $line, $ora
            );
        };
    }

    #[cfg(test)]
    macro_rules! assert_no_local_variants {
        ($line:expr) => {
            let c = parse_line($line);
            assert!(c.is_some(), format!("The line '{}' should parse", $line));

            let c = c.unwrap();
            assert_eq!(
                0,
                c.lvars.len(),
                "The line '{}' should recognize zero (0) local variants",
                $line
            );
        };
    }

    #[cfg(test)]
    macro_rules! assert_local_variants {
        ($line:expr, $want:expr) => {
            let want = $want;
            let c = parse_line($line);
            assert!(c.is_some(), format!("The line '{}' should parse", $line));

            let c = c.unwrap();
            assert_eq!(
                want.len(),
                c.lvars.len(),
                "The line '{}' should recognize all local variants",
                $line
            );
            for (i, wanted) in want.iter().enumerate() {
                let (want_k, want_v) = wanted;
                let (have_k, have_v) = &c.lvars[i];
                assert_eq!(
                    want_k, have_k,
                    "The {}-th local variant should have the expected key ('{}')",
                    i, want_k
                );
                assert_eq!(
                    want_v, have_v,
                    "The {}-th local variant ({}) should have the expected value ('{}')",
                    i, want_k, want_v
                );
            }
        };
    }

    #[cfg(test)]
    macro_rules! assert_global_variants {
        ($line:expr, $want:expr) => {
            let want = $want;
            let c = parse_line($line);
            assert!(c.is_some(), format!("The line '{}' should parse", $line));

            let c = c.unwrap();
            assert_eq!(
                want.len(),
                c.gvars.len(),
                "The line '{}' should recognize all global variants",
                $line
            );
            for (i, wanted) in want.iter().enumerate() {
                assert_eq!(
                    wanted, &c.gvars[i],
                    "The {}-th global variant should be as expected",
                    i
                );
            }
        };
    }

    #[cfg(test)]
    macro_rules! assert_no_global_variants {
        ($line:expr) => {
            let c = parse_line($line);
            assert!(c.is_some(), format!("The line '{}' should parse", $line));

            let c = c.unwrap();
            assert_eq!(
                0,
                c.gvars.len(),
                "The line '{}' should recognize zero (0) global variants",
                $line
            );
        };
    }

    #[test]
    fn should_be_able_to_parse_a_blank_line() {
        assert!(parse_line("").is_some());
        assert!(parse_line("                   ").is_some());
    }

    #[test]
    fn should_be_able_to_parse_a_full_line_comment() {
        assert!(parse_line("# this is a test comment").is_some());
    }

    #[test]
    fn should_be_able_to_parse_simple_lines() {
        assert_parses!("1 DOM Opt", 1, "DOM", "Opt");
        assert_parses!("1x DOM Opt", 1, "DOM", "Opt");
        assert_parses!("1 WAR Return to Nature", 1, "WAR", "Return to Nature");
        assert_parses!("1 M19 Ajani's Last Stand", 1, "M19", "Ajani's Last Stand");
    }

    #[test]
    fn should_be_able_to_parse_multidigit_quantities() {
        assert_parses!("23 DOM Opt", 23, "DOM", "Opt");
        assert_parses!("23x DOM Opt", 23, "DOM", "Opt");
        assert_parses!("1029384756 DOM Opt", 1029384756, "DOM", "Opt");
        assert_parses!("055 DOM Opt", 55, "DOM", "Opt");
    }

    #[test]
    fn should_be_able_to_handle_several_variations_on_set_code() {
        assert_parses!("1 DOM Opt", 1, "DOM", "Opt");
        assert_parses!("1 M19 Opt", 1, "M19", "Opt");
        assert_parses!("1 m19 Opt", 1, "m19", "Opt");
        assert_parses!("1 3ED Clone", 1, "3ED", "Clone");
        assert_parses!("1 3ed Clone", 1, "3ed", "Clone");
    }

    #[test]
    fn should_be_able_to_parse_an_inline_comment_after_oracle_card_name() {
        assert_parses!("1 DOM Opt # gotta love scry!", 1, "DOM", "Opt");
    }

    #[test]
    fn should_be_able_to_handle_extra_whitespace() {
        assert_parses!("    1 DOM Opt", 1, "DOM", "Opt");
        assert_parses!("1     DOM Opt", 1, "DOM", "Opt");
        assert_parses!("1 DOM     Opt", 1, "DOM", "Opt");
        assert_parses!("  1  DOM  Opt", 1, "DOM", "Opt");
        assert_parses!(" 1 DOM Opt   ", 1, "DOM", "Opt");
    }

    #[test]
    fn should_be_able_to_handle_variants() {
        let test = "1 DOM Opt | ";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_local_variants!(test);
        assert_no_global_variants!(test);

        let test = "1 DOM Opt | NM";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_local_variants!(test);
        assert_global_variants!(test, vec!["NM"]);

        let test = "1 DOM Opt | VG foil";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_local_variants!(test);
        assert_global_variants!(test, vec!["VG", "foil"]);

        let test = "1 DOM Opt | (signed: SDCC 2020)";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_global_variants!(test);
        assert_local_variants!(test, vec![("signed", "SDCC 2020")]);

        let test = "1 DOM Opt | EX foil (signed: artist)";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_global_variants!(test, vec!["EX", "foil"]);
        assert_local_variants!(test, vec![("signed", "artist")]);

        let test = "1 DOM Opt | (signed: SDCC 2020) (misprint: Otp not Opt)";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_global_variants!(test);
        assert_local_variants!(
            test,
            vec![("signed", "SDCC 2020"), ("misprint", "Otp not Opt")]
        );
    }

    #[test]
    fn should_be_able_to_handle_inline_comments_in_variants() {
        let test = "1 DOM Opt |#";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_local_variants!(test);
        assert_no_global_variants!(test);

        let test = "1 DOM Opt | NM # worth some $$$";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_local_variants!(test);
        assert_global_variants!(test, vec!["NM"]);

        let test = "1 DOM Opt | (misprint: double print)# worth SERIOUS $$$";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_global_variants!(test);
        assert_local_variants!(test, vec![("misprint", "double print")]);

        let test = "1 DOM Opt | (misprint: double print) # worth SERIOUS $$$";
        assert_parses!(test, 1, "DOM", "Opt");
        assert_no_global_variants!(test);
        assert_local_variants!(test, vec![("misprint", "double print")]);
    }

    #[test]
    fn should_be_unwilling_to_accept_eol_in_the_middle_of_a_local_variant() {
        assert!(!parse_line("1 DOM Opt | (").is_some());
        assert!(!parse_line("1 DOM Opt | (test").is_some());
        assert!(!parse_line("1 DOM Opt | (test:").is_some());
        assert!(!parse_line("1 DOM Opt | (test: ").is_some());
        assert!(!parse_line("1 DOM Opt | (test: foo").is_some());
    }

    #[test]
    fn should_handle_bad_fsm_transitions_as_syntax_errors() {
        assert!(!parse_line("1xx DOM Opt").is_some());
        assert!(!parse_line("1x DOM Opt ||").is_some());
        assert!(!parse_line("1x DOM Opt | NM ((test:signed))").is_some());
        assert!(!parse_line("1x DOM Opt | NM (test:signed))").is_some());
    }
}
