#[derive(Debug)]
struct Something {
	name: String
}

fn main(){
	let mut my_thing = Something {
		name: String::from("Marc")
	};
	my_thing.name = String::from("Víctor"); //Change "Marc" to "Víctor"
	println!("{:?}",my_thing) //Something { nombre: "Víctor" }
}

