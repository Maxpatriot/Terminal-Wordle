// Using filename will return a vector<String> where each string is a different line in the file
// Uses Result to check if file can be read, else return Err
fn file_to_vec(filename: &str) -> std::io::Result<Vec<String>> { 
    let file_in = std::fs::File::open(filename)?; 
    let file_reader = std::io::BufReader::new(file_in); 
    Ok(std::io::BufRead::lines(file_reader).filter_map(std::io::Result::ok).collect()) 
} 

fn main() {
    // Import used for coloring output to save redundency
    use colored::Colorize;

    // Loads both all possible words all valid guesses
    let possible_words: Vec<String> = file_to_vec("wordle-answers-alphabetical.txt").expect("couldn't load words from file");
    let possible_guess: Vec<String> = file_to_vec("wordle-allowed-guesses.txt").expect("couldn't load guesses from file");
    
    // clears terminal before the program starts
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // UNIMPLEMENTED
    // 2 vectors to store all letters and all currently guessed letters
    let _all_letters = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut guessed_letters: Vec<char> = Vec::new();

    // Uses rand crate to first create a generator and then use said generator to create a usize between 0 and possible guesses
    let mut random_gen = rand::thread_rng();
    let random_num: usize = rand::Rng::gen_range(&mut random_gen, 0..possible_words.len());

    let mut word = possible_words[random_num].to_owned();

    println!("Welcome to {}\n", "WORDLE".magenta().bold());
    

    let args = std::env::args();
    let mut word_check = false;
    for arg in args {
        if word_check && arg.len() == 5{
            word = arg
        }else if arg == "-help" || arg == "-h" {
            println!("{}", "========HELP===================================================================================".bright_cyan());
            println!("Guess 5 letter words to try and find the answer.
If the letter appears in {}, that means it is the correct letter in the correct position.
If the letter appears in {}, that means it is the correct letter in the incorrect position.
And if the letter is White, that means it does not appear in the word.
You have 6 guesses to get it right, so use all your previous guesses to make your best guess.
Good Luck -MD\n{}\n", "GREEN".green(), "YELLOW".yellow(), "===============================================================================================".bright_cyan());
        }else if arg == "-o" {
            word_check = true;
            continue;
        }
    }

    //println!("{}", word);

    // Loops 6 times for each guess in the game, will break if player guesses above word
    for i in 1..=6 {
        // word_vec is mut'd later so it needs to be redefined each loop

        // let mut word_vec: Vec<char> = word.chars().into_iter().collect();
        let mut word_vec: Vec<char> = word.chars().into_iter().collect();

        println!("Guess number {}:", i);

        // Defines buffer to be used by stdin() and used after loop below
        let mut user_guess:String;
        loop {
            // Promt user for 5 letter word and loads into user_guess
            user_guess = String::new();
            std::io::stdin().read_line(&mut user_guess).expect("error reading line");
            user_guess = user_guess.trim().to_string().to_lowercase();

            // Checks that the user_guess is valid, if not reprompt user
            if user_guess.len() == 5 {
                if possible_guess.contains(&user_guess) || possible_words.contains(&user_guess) || user_guess == word{
                    break;
                }
            }
            println!("{}", "Invalid word! Try again".red());
        }

        // seperate the user guess into a Vec<chars> to later itterate
        let mut guess_arr: Vec<(char, usize)> = Vec::new();
        for c in user_guess.chars() {
            guess_arr.push((c, 0));
        }

        // first itteration checks if any letters are in the correct position
        let mut index = word_vec.iter_mut();
        for (ch, status) in &mut guess_arr {
            let curr = index.next().unwrap();
            if curr == ch {
                //println!("{}, match", ch);
                *status = 2;
                *curr = '.';
            }
        }

        // second itteration checks if any remaining letters are present in word
        for (ch, status) in &mut guess_arr {
            if word_vec.contains(ch) && *status != 2{
                *status = 1;
                let i = word_vec.iter().position(|&r| r == *ch).unwrap();
                word_vec[i] = '.';
            }
        }

        // third itteration prints out letters according to second value in tuple
        for (ch, status) in guess_arr {
            if status == 2 {
                print!("{} ", ch.to_ascii_uppercase().to_string().green());
            } else if status == 1 {
                print!("{} ", ch.to_ascii_uppercase().to_string().yellow());
            } else {
                print!("{} ", ch.to_ascii_uppercase());
            }
            if guessed_letters.contains(&ch) {
                guessed_letters.push(ch);
            }
        }
        
        println!();

        // Win Condition
        if word == user_guess {
            return;
        }
    }
    // Lose Condition
    println!("{}", word.to_uppercase().bold().red()); 
}
