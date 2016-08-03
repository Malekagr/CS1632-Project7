use std::io;
extern crate rand;
use rand::Rng;
use std::io::Error;
use std::process;
//use std::string;

#[derive(Clone, Copy)]
	struct Stats{
		game_count: i32,
		win_count: i32,
		loss_count: i32,
		tie_count: i32,
		rock_count: i32,
		scissors_count: i32,
		paper_count: i32,

	}

#[derive(Debug)]
enum AIChoice {
	Rock, 
	Paper,
	Scissors,
}

// get a string from the user
fn get_a_string() -> Result<String, Error> {
    println!("Enter choice (r,p,s) or q to quit >");
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).expect("FAIL");
    Ok(to_return)
}
fn get_rand() -> i32{
	let x: i32 = rand::thread_rng().gen_range(0,99);
	return x;
}


fn convert_int_to_game_value (r: i32, s: i32) ->  String{
	// this method will take in three ints and convert them to characters
    let random = get_rand();
    if random < r {
    	let r_string = String::from("r");
    	return r_string
    }
    else if (random > r) & (random < 99 - s) {
    	let p_string = String::from("p");
    	return p_string;
    }
    else {
    	let s_string = String::from("s");
    	return s_string;
    }

}

fn play_game (player_guess: String, ai_guess: String, game_data: &mut Stats) {
	game_data.game_count +=1;
	//Player chooses rock
	if player_guess.trim() == "r"{
		game_data.rock_count+=1;
		if ai_guess == "p"{
			println!("You lose!");
			game_data.loss_count+=1;
		}
		if ai_guess == "r"{
			println!("It's a tie!");
			game_data.tie_count+=1;
		}
		if ai_guess == "s"{
			println!("You win!");
			game_data.win_count+=1;

		}
	}
	//Player chooses paper
	else if player_guess.trim() == "p"{
		game_data.paper_count+=1;
		if ai_guess == "p"{
			println!("It's a tie!");
			game_data.tie_count+=1;
		}
		if ai_guess == "r"{
			println!("You win!");
			game_data.win_count+=1;
		}
		if ai_guess == "s"{
			println!("You lose!");
			game_data.loss_count+=1;
		}
	}
	//Player chooses scissors
	else if player_guess.trim() == "s"{
		game_data.scissors_count+=1;
		if ai_guess == "p"{
			println!("You win!");
			game_data.win_count+=1;
		}
		if ai_guess == "r"{
			println!("You lose!");
			game_data.loss_count+=1;
		}
		if ai_guess == "s"{
			println!("It's a tie");
			game_data.tie_count+=1;
		}
	}

}
fn end_game(game_data: Stats){
	let mut win_percent: f32 = (game_data.win_count as f32)/(game_data.game_count as f32);
	let win_percent_value = win_percent;

	let mut loss_percent: f32 = (game_data.loss_count as f32)/(game_data.game_count as f32);
	let loss_percent_value = loss_percent;

	let mut tie_percent: f32 = (game_data.tie_count as f32)/(game_data.game_count as f32);
	let tie_percent_value = tie_percent;

	if game_data.game_count == 0{
		win_percent = 0.0;
		loss_percent = 0.0;
		tie_percent = 0.0;
	}
	
	else {
		for _ in 0..100 { 
				win_percent = win_percent + win_percent_value;
				loss_percent = loss_percent + loss_percent_value;
				tie_percent = tie_percent + tie_percent_value;
		}
	}

	println!("Player Stats");
	println!("Wins: {} ({:.2}%)",game_data.win_count, win_percent);
	println!("Ties: {} ({:.2}%)",game_data.tie_count, tie_percent);
	println!("Losses: {} ({:.2}%)",game_data.loss_count, loss_percent);
	println!("Rocks: {}",game_data.rock_count);
	println!("Papers: {}", game_data.paper_count);
	println!("Scissors: {}", game_data.scissors_count);
	println!("-----------------");
	process::exit(0);
}

fn main() {
    let mut user_input;
    let playing = true;
    let mut rock_chance: i32 = 33;
    let mut scissors_chance: i32 = 33;
    let mut game_data= Stats{ game_count: 0, win_count: 0, loss_count: 0, tie_count: 0,	rock_count: 0, scissors_count: 0, paper_count: 0};
	while playing{
	    match get_a_string() {
	        Ok(n) => user_input = n,
	        Err(_) => {
	              println!("ERROR COULD NOT READ!");
	              user_input = String::from("ERROR");
	           }
	    }

	    let user_string = user_input.to_string();
	    let ai = convert_int_to_game_value(rock_chance, scissors_chance);
	    match user_string.trim() {
	    	"r" => {
	    		println!("Player chose rock");
	    		scissors_chance-= 4;
	    		rock_chance-=2;
	    	}
	    	"p" => {
	    		println!("Player chose paper");
	    		scissors_chance+= 6;
	    		rock_chance-=4;
	    	}
	    	"s" => {
	    	println!("Player chose scissors");
	    		scissors_chance-= 2;
	    		rock_chance+=6;
	    }
	    	"q" => println!("Quitting game"),
	    	_ => println!("Please enter r, p, s, or q!!!!!"),
	    }
	    if (user_input.trim() == "q") | (user_input.trim() == "r") | (user_input.trim() == "s") | (user_input.trim() == "p") {
		    if user_string.trim()== "q"{
		    	end_game(game_data);
		    }
		    	match ai.trim(){
		    	"r" => println!("Opponent chose {:?}", AIChoice::Rock),
		    	"p" => println!("Opponent chose {:?}", AIChoice::Paper),
		    	"s" => println!("Opponent chose {:?}", AIChoice::Scissors),
		    	_ => println!("AI has gone rogue. Skynet is well and truly here"),
		    }
			play_game(user_input, ai,&mut game_data);
		}
	}
}
	