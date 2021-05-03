
fn create_option() -> Option<u32> {
    return Some(1);
}

fn main(){
    // If it returned anything
    if create_option().is_some() {
        println!("Got something!");
    }

    // Do X based on what it returns
    match create_option().unwrap() {
        1 => println!("Got 1!"),
        2 => println!("Got 2"),
        _ => println!("uhmmm ?")
    };
}