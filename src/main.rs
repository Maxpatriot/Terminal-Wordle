fn file_to_vec(filename: &str) -> std::io::Result<Vec<String>> { 
    let file_in = std::fs::File::open(filename)?; 
    let file_reader = std::io::BufReader::new(file_in); 
    Ok(std::io::BufRead::lines(file_reader).filter_map(std::io::Result::ok).collect()) 
} 

fn main() {
    use colored::Colorize;

    // ctrlc::set_handler(move || {
    //     println!("received Ctrl+C!");
    // }).expect("Error setting Ctrl-C handler");

    let possible_words: Vec<String> = file_to_vec("wordle-answers-alphabetical.txt").expect("couldn't load words from file");
    let possible_guess: Vec<String> = file_to_vec("wordle-allowed-guesses.txt").expect("couldn't load guesses from file");

    let mut random_gen = rand::thread_rng();
    let random_num: usize = rand::Rng::gen_range(&mut random_gen, 0..possible_words.len());

    let mut word = possible_words[random_num].to_owned();
    
    //unimplemented function that allows setting of word to cmd line arg
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1].len() == 5 {
        word = args[1].to_owned();
    }

    //println!("{}", word);

    for i in 1..=6 {
        let mut word_vec: Vec<char> = word.chars().into_iter().collect();
        println!("Guess number {}:", i);
        let mut user_guess:String;
        loop {
            user_guess = String::new();
            std::io::stdin().read_line(&mut user_guess).expect("error reading line");
            user_guess = user_guess.trim().to_string();
            if user_guess.len() == 5 {
                if possible_guess.contains(&user_guess) || possible_words.contains(&user_guess) || user_guess == word{
                    break;
                }
            }
            println!("{}", "Invalid word! Try again".red());
        }
        let guess_arr = user_guess.trim().chars();
        let mut index = 0;
        for ch in guess_arr {
            if word_vec.contains(&ch){
                if word_vec[index] == ch {
                    print!("{} ", ch.to_string().green());
                } else {
                    print!("{} ", ch.to_string().yellow());
                }
                //removes the ch from the guess list to avoid dupes
                let index = word_vec.iter().position(|x| *x == ch).unwrap();
                word_vec[index] = '.';
            } else {
                print!("{} ", ch);
            }
            index += 1;
        }
        println!();
        if user_guess == word{
            return;
        }
    }
    
    println!("{}", word.to_uppercase().bold().red());
    
}
