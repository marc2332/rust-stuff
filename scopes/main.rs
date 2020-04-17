
//You can create a scope when assigning a value to a variable and it must return something
//See https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main(){
	let my_value:i8 = {
		let x:i8 = 3;
		let y:i8 = 7;
		x + y
	};
	println!("{}",my_value) //10
}