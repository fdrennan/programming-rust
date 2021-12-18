use std::fs::ReadDir;
// use ansi_term::Style;
use std::path::Path;
use std::ffi::OsStr;

#[allow(dead_code)]
use crate::misc::print_type_of;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).file_stem()
        // .extension()
        .and_then(OsStr::to_str)
}

pub fn print_paths(paths: &mut ReadDir, args: &mut Vec<String>) -> Vec<String> {

    let base_dir = &args[0].to_owned();
    let b = paths.into_iter().map(|x| {
        let x = x.unwrap().path().display().to_string();
        let original_path = x.as_str();

        let new_path = x
            .as_str()
            .replace(".rs", "")
            .replace("src", &base_dir);
        println!("Copying {} to history/{}/", original_path, new_path);
        x
    }).collect::<Vec<String>>();

    println!("{:?}", args);
    // println!("Args are - {:?}", args);
    b
    // b = b.coll
    // println!("{:?}", b);

    // for path in paths {
    //     let path = path.unwrap().path().display().to_string();
    //     let file_name = get_extension_from_filename(&path.as_str());
    //     match file_name {
    //         Some(val) => println!("{:?}", val),
    //         None => println!("Parse Failed")
    //     }
    // }

    // let file_name = match file_name {
    //     Some(a) => a,
    //     None => println!("No String")
    // };
    // println!("{:?}", file_name);
    // println!("{:?}", args);

    // println!("{:#?}", args[0]);
    //
    // let message = Style::new()
    //     .bold()
    //     .paint(
    //         "Copying paths: "
    // );
    // println!("{}", message);
    //
    // for path in paths {
    //     // let file_name = path.unwrap().path().display();
    //
    //     println!(
    //         " - {} ->",
    //         Style::new()
    //             .italic()
    //             .paint(
    //                 file_name
    //             )
    //     )
    // }

}