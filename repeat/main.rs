fn main(){
	println!(
		"{}",
		repeat("Hello 😴 ",4) //Hello 😴 Hello 😴 Hello 😴 Hello 😴
	)
}

//This will return the first first argument repeated as many times you pass in the second argument
fn repeat(text: &str, times: usize) -> String	{
	text.repeat( times )
}