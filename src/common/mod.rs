pub mod macros {
    macro_rules! _load_input {
        () => {
            use lazy_static::lazy_static;
            use std::{fs::read_to_string, path};
            lazy_static! {
                static ref INPUT: String = {
                    let file = file!();
                    let len = file.len();
                    let day = &file[len - 5..len - 3];
                    let path = path::Path::new(file)
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("inputs")
                        .join(format!("day{}.txt", day));

                    read_to_string(path).expect("input file not found")
                };
            }
        };
    }
    pub(crate) use _load_input as load_input;
}
