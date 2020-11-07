
fn main(){
	
	// You can use match to do different things depending on the Result's value
	
	match do_magic(50){
		Ok(_res) => {
			println!("{}", "All is ok!");
		},
		Err(_err) => {
			println!("{}", "Oh no!");
		}
	}
	
	// Or just get the value if you are sure it will won't return Err, because if it does, the program will panic
	
	println!("{}", do_magic(5).unwrap());
}

fn do_magic(value: i32) -> Result<i32,&'static str>{
	if value < 10 {
		Ok(value)
	}else{
		Err("Error!")
	}
}
