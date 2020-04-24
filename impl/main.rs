
struct ACoolStruct {
	something: u8
}

impl ACoolStruct {
	fn get_something(&self)  -> u8 {
		self.something
	}
	fn set_something(&mut self, new_value: u8){
		self.something = new_value;
	}
}

fn main(){
	let mut my_struct = ACoolStruct {
		something: 10
	};
	println!("Initial value: {}", my_struct.get_something()); //Initial value: 10
	my_struct.set_something(99); //Changes my_struct.something to 99
	println!("New value: {}", my_struct.get_something()); //Initial value: 99
	
	/*
	* Although I am using getSomething to get the `something` value, it can also 
	* be accessed via dot notation: my_struct.something
	* like in basic_struct example
	*/
}

