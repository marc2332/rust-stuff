
//Import fruits module which contains the banana
mod fruits;

//Import others module which contains the kiwi
mod others;

fn main(){
	fruits::print_banana();
	others::kiwi::print_kiwi();
}
