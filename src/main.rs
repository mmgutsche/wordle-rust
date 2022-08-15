
// GUI appilcation for the game wordle written in Rust, using the iced framework.

use std::env;
use std::fs;
use rand::prelude::*;


use iced::{
    button, text_input, Alignment, Button, Checkbox,
    Column, Container, Element, Length, ProgressBar, Radio, Row, Rule, Sandbox,
    Scrollable, Settings, Slider, Space, Text, TextInput, Toggler,
};


pub fn main() -> iced::Result {
    Wordle::run(Settings::default())
}

#[derive(Default)]
struct Puzzle{
    solution: String,
    max_tries: usize,
    word_length: usize,
    valid_words: Vec<String>,    
}

impl Puzzle{
    pub fn new() -> Puzzle{
        let max_tries = 5;
        let word_length = 5;
        let file_path = "words.txt";
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

#[derive(Default)]
struct Wordle{
    puzzle: Puzzle,
    words_tried: Vec<String>,

    input: text_input::State,
    input_value: String,
    button: button::State,
    notification: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
}




impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        Wordle{ puzzle: Puzzle::new(), 
            words_tried: Vec::new(), 
            input: text_input::State::new(), 
            input_value: String::new(), 
            button: button::State::new(),
            notification: String::new()
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
                match self.puzzle.valid_words.iter().find(self.input_value){
                    Some => {
                        self.words_tried.push(self.input_value.clone())
                    }
                }
                if self.input_value 
            }
            }
    }

    fn view(&mut self) -> Element<Message> {
        

        let text_input = TextInput::new(
            &mut self.input,
            "Type something...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);

        let button = Button::new(&mut self.button, Text::new("Submit"))
            .padding(10)
            .on_press(Message::ButtonPressed);
        
        let warning: Text = Text::new(format!("{}", self.notification))
            .size(20)
            .color([0.5, 0.5, 0.5])

        let content = Column::new()
        .spacing(20)
        .padding(20)
        .max_width(600)
        .push(Row::new().spacing(10).push(text_input).push(button))
        .push(Row::new().spacing(10).push(warning));

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()

       
    }
}