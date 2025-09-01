use bcrypt;

pub fn run(password: Option<&str>, check: Option<&str>) {
    let password = match password {
        Some(v) => v,
        None => panic!("missing required --password command-line flag"),
    };

    match check {
        Some(target) => {
            match bcrypt::verify(&password, &target) {
                Ok(true) => println!("ok"),
                _ => println!("nope."),
            };
        },
        None => {
            match bcrypt::hash(password, 10) {
                Ok(crypted) => println!("{}", crypted),
                Err(e) => panic!("unable to hash password: {}", e),
            };
        },
    };
}
