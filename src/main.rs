
// GUI appilcation for the game wordle written in Rust, using the iced framework.
use std::fs;
use iced::Color;
use rand::prelude::*;


use iced::{
    button, text_input,  Button,
    Column, Container, Element, Length, Row,  Sandbox,
    Settings, Text, TextInput,
};


pub fn main() -> iced::Result {
    Wordle::run(Settings::default())
}



pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};


pub const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};



pub const GRAY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
    

#[derive(Default)]
struct Puzzle{
    solution: String,
    max_tries: usize,
    word_length: usize,
    valid_words: Vec<String>,    
}


pub enum CharGuess {
    Correct,
    CorrectWrongPlace,
    Wrong
}

impl Puzzle{
    pub fn new() -> Puzzle{
        let max_tries = 5;
        let word_length = 5;
        let file_path = "words-5.txt";
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        let valid_words = contents.split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        // pick a random word from the list of valid words
        let solution = valid_words.choose(&mut rand::thread_rng()).unwrap().to_string();
        Puzzle{
            solution,
            max_tries,
            word_length,
            valid_words
        }
    }

   
}

pub fn check_word(word: &String, solution: &String) -> Vec<CharGuess>{
    let mut result = Vec::new();
    let mut chars_available = solution.chars().collect::<Vec<char>>();
    // iterate over each char and check if it is at the correct position, in the word at all, or not in the word at all.
    // If it is in the word at all, remove it from the list of available chars.

    for (idx, c) in word.chars().enumerate(){        
        if chars_available.contains(&c){
            if c == solution.chars().nth(idx).unwrap(){
                result.push(CharGuess::Correct);
            }
            else{
                result.push(CharGuess::CorrectWrongPlace);
            }
            if let Some(pos) = chars_available.iter().position(|x| *x == c) {
                chars_available.remove(pos);
            }
        } 
        else {
            result.push(CharGuess::Wrong);
        }
    }
    
    result        
} 

#[derive(Default)]
struct Wordle{
    puzzle: Puzzle,
    words_tried: Vec<String>,

    input: text_input::State,
    input_value: String,
    button: button::State,
    notification: String,
    toggle_new_game: bool    
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
}




fn colorize_word<'a>(word: &'a String, solution: &String) -> Row<'a, Message> {
    let chars_guessed = check_word(&word, &solution) ;
    // create for each char guess a Text element with the corresponding color
    let mut row = Row::new().spacing(10);
    for (i, c) in word.chars().enumerate(){
        let color = match chars_guessed[i] {
            CharGuess::Correct => GREEN,
            CharGuess::CorrectWrongPlace => YELLOW,
            CharGuess::Wrong => GRAY,
        };
        row = row.push(Text::new(c).color(color)); 
    }
    row
}

impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        Wordle{ puzzle: Puzzle::new(), 
            words_tried: Vec::new(), 
            input: text_input::State::new(), 
            input_value: String::new(), 
            button: button::State::new(),
            notification: String::new(), 
            toggle_new_game: false 
        }
    }

    fn title(&self) -> String {
        String::from("Wordle")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                //let mut truncated = value.clone();
                //truncated.truncate(self.word_length);
                self.input_value = value.to_uppercase();
                self.input_value.truncate(self.puzzle.word_length);
            },
            Message::ButtonPressed => {
                if self.toggle_new_game {
                    // reset the game
                    self.words_tried = Vec::new();
                    self.puzzle = Puzzle::new();    
                    self.input_value = String::new();
                    self.notification = String::from("New game.");
                    self.toggle_new_game = false;                

                }
                else{
                    let mut iter = self.puzzle.valid_words.iter();
                    let valid_word = iter.find( |x| x == &&self.input_value);
                    match valid_word {
                        Some(word)=> {
                            self.notification = String::from("");


                            self.words_tried.push(word.to_string());
                            self.input_value = String::new();

                            if word == &self.puzzle.solution{
                                self.notification = String::from("You win!");
                                self.toggle_new_game = true;

                            }
                            else if self.words_tried.len() >= self.puzzle.max_tries{
                                self.notification = String::from("You lose! The word was: ") + &self.puzzle.solution;
                                self.toggle_new_game = true;
                            }
                            
                        }
                        None => {
                            self.notification = format!("{} is not a valid word", self.input_value);
                        }
                    }
                }
            }
        }
    }



    
    fn view(&mut self) -> Element<Message> {
        

        let text_input = TextInput::new(
            &mut self.input,
            "Your guess",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);
        
        let mut button_text = "Submit";
        if self.toggle_new_game {
            button_text = "New game";

        }
        let button = Button::new(&mut self.button, Text::new(button_text))
            .padding(10)
            .on_press(Message::ButtonPressed);
        
        let warning: Text = Text::new(format!("{}", self.notification))
            .size(20)
            .color([0.5, 0.5, 0.5]);


        let mut content = Column::new()
        .spacing(20)
        .padding(20)
        .max_width(600);

        for word in self.words_tried.iter(){
            let row = colorize_word(&word, &self.puzzle.solution);
            content = content.push(row);
        }
        content = content.push(Row::new().spacing(10)
        .push(text_input)
        .push(button))
        .push(Row::new().spacing(10).push(warning));

        Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }


    fn background_color(&self) -> Color {
        Color::BLACK        
    }
    
}