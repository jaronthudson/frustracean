//Jeff Thomson
//Rust Final Project: Frustracean

//Much credit goes to Quantum Badger, creator of Speedy2d.
//I learned the Speedy2d framework from the User_events.rs example
//(https://github.com/QuantumBadger/Speedy2D/blob/master/examples/user_events.rs)
//I kept some of the variable names (MyWindowHelper) and what not.

extern crate wikipedia;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;
use speedy2d::window::{WindowCreationOptions, WindowHandler, WindowHelper, WindowSize};
use speedy2d::{Graphics2D, Window};

use regex::Regex;
use std::rc::Rc;

use speedy2d::font::{Font, FormattedTextBlock, TextAlignment, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

fn main() {
    let window: Window<String> = Window::new_with_user_events(
        "FRUSTRACEAN!",
        WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Vector2::new(1200, 800)),
            None,
        ),
    )
    .unwrap();

    let font = Font::new(include_bytes!("../NotoSans-Regular.ttf")).unwrap();

    let t = "Welcome to FЯUSTRACёΛὴ! \
        Identify the language of randomly selected \
        Wikipedia texts in 50 Languages!"
        .to_string();

    let text = font.layout_text(
        &t,
        64.0,
        TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
    );

    window.run_loop(MyWindowHandler {
        text,
        mouse_pos: Vector2::ZERO,
        mouse_button_down: false,

        lang_1: ("1".to_string(), "Click Here To Start".to_string()),
        lang_2: ("2".to_string(), "".to_string()),
        lang_3: ("3".to_string(), "".to_string()),
        lang_4: ("4".to_string(), "".to_string()),
        actual_lang: ("1".to_string(), "Click Here To Start".to_string()),
        mouse_color: (0.5, 0.5, 0.5),
        color_mouse: false,
        correct: 0,
        incorrect: 0,
    });
}

struct MyWindowHandler {
    mouse_pos: Vector2<f32>,
    mouse_button_down: bool,
    text: Rc<FormattedTextBlock>,
    lang_1: (String, String),
    lang_2: (String, String),
    lang_3: (String, String),
    lang_4: (String, String),
    actual_lang: (String, String),
    mouse_color: (f32, f32, f32),
    color_mouse: bool,
    correct: u8,
    incorrect: u8,
}

impl WindowHandler<String> for MyWindowHandler {
    fn on_draw(&mut self, _helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);

        //color for initial screen
        if self.actual_lang == ("1".to_string(), "Click Here To Start".to_string()) {
            graphics.draw_text((100.0, 90.0), Color::from_rgb(1.0, 0.27, 0.0), &self.text);
        } else {
            graphics.draw_text((100.0, 90.0), Color::BLACK, &self.text);
        }

        let font = Font::new(include_bytes!("../NotoSans-Regular.ttf")).unwrap();

        //Button 1
        let but1 = Rectangle::new(Vector2::new(100.0, 600.0), Vector2::new(500.0, 700.0));

        graphics.draw_rectangle(but1, Color::from_rgb(0.87, 0.72, 0.53));

        let but1_text = font.layout_text(
            &self.lang_1.1,
            32.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((100.0, 650.0), Color::BLACK, &but1_text);

        //Button 2
        let but2 = Rectangle::new(Vector2::new(100.0, 450.0), Vector2::new(500.0, 550.0));
        graphics.draw_rectangle(but2, Color::from_rgb(0.85, 0.63, 0.98));

        let but2_text = font.layout_text(
            &self.lang_2.1,
            32.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((100.0, 500.0), Color::BLACK, &but2_text);

        //Button 3
        let but3 = Rectangle::new(Vector2::new(700.0, 450.0), Vector2::new(1100.0, 550.0));
        graphics.draw_rectangle(but3, Color::from_rgb(0.56, 0.93, 0.56));

        let but3_text = font.layout_text(
            &self.lang_3.1,
            32.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((700.0, 500.0), Color::BLACK, &but3_text);

        //Button 4
        let but4 = Rectangle::new(Vector2::new(700.0, 600.0), Vector2::new(1100.0, 700.0));
        graphics.draw_rectangle(but4, Color::from_rgb(0.7, 0.7, 1.0));

        let but4_text = font.layout_text(
            &self.lang_4.1,
            32.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((700.0, 650.0), Color::BLACK, &but4_text);

        //Score Button
        let score_but = Rectangle::new(Vector2::new(550.0, 525.0), Vector2::new(650.0, 625.0));
        graphics.draw_rectangle(score_but, Color::from_rgb(0.8, 0.5, 0.5));

        //let score_string = "Score: ".to_string() + &self.score.0.to_string() + &self.score.1.to_string();
        let score_correct_string = "Right: ".to_string() + &self.correct.to_string();
        let score_correct = font.layout_text(
            &score_correct_string,
            20.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((550.0, 550.0), Color::BLACK, &score_correct);

        let score_incorrect_string = "Wrong: ".to_string() + &self.incorrect.to_string();
        let score_incorrect = font.layout_text(
            &score_incorrect_string,
            20.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        graphics.draw_text((550.0, 575.0), Color::BLACK, &score_incorrect);

        if self.color_mouse {
            let r = self.mouse_color.0;
            let g = self.mouse_color.1;
            let b = self.mouse_color.2;
            //draw the mouse
            graphics.draw_circle(self.mouse_pos, 20.0, Color::from_rgb(r, g, b));
        }

        //redraw frame
        _helper.request_redraw();
    }

    //fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton)
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<String>, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_down = true;
        }

        //check to see if the click was in the boundaries of a button:

        //button 1
        if (self.mouse_pos.x > 100.0)
            && (self.mouse_pos.x < 500.0)
            && (self.mouse_pos.y > 600.0)
            && (self.mouse_pos.y < 700.0)
        {
            if self.lang_1 == self.actual_lang {
                //update score if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.correct += 1;
                }

                MyWindowHandler::reset_lang(self);

                if self.color_mouse {
                    MyWindowHandler::change_mouse_color(self);
                }
            } else {
                self.lang_1 = ("null".to_string(), "No!".to_string());

                //if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.incorrect += 1;
                }
            }
        }

        //button 2
        if (self.mouse_pos.x > 100.0)
            && (self.mouse_pos.x < 500.0)
            && (self.mouse_pos.y > 450.0)
            && (self.mouse_pos.y < 550.0)
        {
            if self.lang_2 == self.actual_lang {
                //update score if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.correct += 1;
                }
                MyWindowHandler::reset_lang(self);

                if self.color_mouse {
                    MyWindowHandler::change_mouse_color(self);
                }
            } else {
                self.lang_2 = ("null".to_string(), "Incorrect!".to_string());

                //if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.incorrect += 1;
                }
            }
        }

        //button 3
        if (self.mouse_pos.x > 700.0)
            && (self.mouse_pos.x < 1100.0)
            && (self.mouse_pos.y > 450.0)
            && (self.mouse_pos.y < 550.0)
        {
            if self.lang_3 == self.actual_lang {
                //update score if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.correct += 1;
                }
                MyWindowHandler::reset_lang(self);

                if self.color_mouse {
                    MyWindowHandler::change_mouse_color(self);
                }
            } else {
                self.lang_3 = ("null".to_string(), "Nope!".to_string());
                //if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.incorrect += 1;
                }
            }
        }

        //button 4
        if (self.mouse_pos.x > 700.0)
            && (self.mouse_pos.x < 1100.0)
            && (self.mouse_pos.y > 600.0)
            && (self.mouse_pos.y < 700.0)
        {
            //turn on the color mouse if we are at the start screen
            if self.actual_lang == ("1".to_string(), "Click Here To Start".to_string()) {
                self.color_mouse = true;
                //break; //make sure correct
            } else if self.lang_4 == self.actual_lang {
                //update score if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.correct += 1;
                }
                MyWindowHandler::reset_lang(self);

                if self.color_mouse {
                    MyWindowHandler::change_mouse_color(self);
                }
            } else {
                self.lang_4 = ("null".to_string(), "Wrong!".to_string());
                //if not on the start screen
                if self.actual_lang.0 != (*"1") {
                    self.incorrect += 1;
                }
            }
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<String>, position: Vector2<f32>) {
        self.mouse_pos = position;

        helper.request_redraw();
    }
} //end of impls

impl MyWindowHandler {
    fn reset_lang(&mut self) {
        //reset the language

        let font = Font::new(include_bytes!("../NotoSans-Regular.ttf")).unwrap();
        let lang_vec = get_langs(4);
        self.lang_1 = lang_vec[0].clone();
        self.lang_2 = lang_vec[1].clone();
        self.lang_3 = lang_vec[2].clone();
        self.lang_4 = lang_vec[3].clone();

        //https://stackoverflow.com/questions/34215280/how-can-i-randomly-select-one-element-from-a-vector-or-array/34215930
        self.actual_lang = (*lang_vec.choose(&mut rand::thread_rng()).unwrap()).clone();

        let t_new = get_text(&self.actual_lang.0); //check that the borrow is right
        self.text = font.layout_text(
            &t_new,
            32.0,
            TextOptions::new().with_wrap_to_width(1000.0, TextAlignment::Left),
        );
        //*/

        //change what is stores in self too
    }

    fn change_mouse_color(&mut self) {
        //https://docs.rs/rand/0.5.0/rand/trait.Rng.html#method.gen_range
        let mut rng = thread_rng();

        let r: f32 = rng.gen_range(0, 100) as f32 * 0.01;
        let g: f32 = rng.gen_range(0, 100) as f32 * 0.01;
        let b: f32 = rng.gen_range(0, 100) as f32 * 0.01;

        self.mouse_color = (r, g, b);

        //make sure that the mouse color is visible (not too light)
        if (r > 0.8) && (g > 0.8) && (b > 0.8) {
            MyWindowHandler::change_mouse_color(self);
        }
    }
} //end of other impls

fn get_text(lang: &str) -> String {
    let mut wiki2 = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();

    let use_url = "https://".to_owned() + lang + ".wikipedia.org/w/api.php";

    //change the url
    wiki2.set_base_url(&use_url);

    let ran = wiki2.random();
    let ran_un = ran.unwrap().unwrap(); //unwraps error and panics if abbrev is bad
    let ran_page = wiki2.page_from_title(ran_un);
    let ran_content = ran_page.get_content().unwrap();

    let re = Regex::new(r"\n").unwrap();

    //this is for splitting the text on the \n newline
    let spl: Vec<&str> = re.split(&ran_content).collect();

    //I modified code to find the shortest string to instead find the longest
    //https://users.rust-lang.org/t/find-the-shortest-string-in-a-vector/1247
    let long = spl.iter().fold(
        spl[0],
        |acc, &item| {
            if item.len() > acc.len() {
                item
            } else {
                acc
            }
        },
    );

    //this was the code I used for Truncate
    //I removed it because was causing a character misalign panic sometimes
    //truncate the string (make sure this works with Unicode)
    //let mut long_trunc = long.to_string();
    //long_trunc.truncate(600);
    //return long_trunc.to_string();

    long.to_string()
}

fn get_langs(num: usize) -> Vec<(String, String)> {
    let ret = vec![
        ("pt".to_string(), "Portuguese".to_string()),
        ("es".to_string(), "Spanish".to_string()),
        ("hu".to_string(), "Hungarian".to_string()),
        ("fr".to_string(), "French".to_string()),
        ("ru".to_string(), "Russian".to_string()),
        ("af".to_string(), "Afrikaans".to_string()),
        ("ca".to_string(), "Catalan".to_string()),
        ("cs".to_string(), "Czech".to_string()),
        ("cy".to_string(), "Welsh".to_string()),
        ("da".to_string(), "Danish".to_string()),
        ("de".to_string(), "German".to_string()),
        ("el".to_string(), "Greek".to_string()),
        ("eo".to_string(), "Esperanto".to_string()),
        ("et".to_string(), "Estonian".to_string()),
        ("eu".to_string(), "Basque".to_string()),
        ("fi".to_string(), "Finnish".to_string()),
        ("ga".to_string(), "Irish / Gaeilge".to_string()),
        ("haw".to_string(), "Hawaiian".to_string()),
        ("hi".to_string(), "Hindi".to_string()),
        ("hr".to_string(), "Croatian".to_string()),
        ("id".to_string(), "Indonesian".to_string()),
        ("is".to_string(), "Icelandic".to_string()),
        ("kl".to_string(), "Greenlandic".to_string()),
        ("la".to_string(), "Latin".to_string()),
        ("lt".to_string(), "Lithuanian".to_string()),
        ("lv".to_string(), "Latvian".to_string()),
        ("mi".to_string(), "Māori".to_string()),
        ("mk".to_string(), "Macedonian".to_string()),
        ("mn".to_string(), "Mongol".to_string()),
        ("pl".to_string(), "Polish".to_string()),
        ("ro".to_string(), "Romanian".to_string()),
        ("sk".to_string(), "Slovenian".to_string()),
        ("so".to_string(), "Somali".to_string()),
        ("sr".to_string(), "Serbian".to_string()),
        ("sv".to_string(), "Swedish".to_string()),
        ("tk".to_string(), "Turkmen".to_string()),
        ("tl".to_string(), "Tagalog".to_string()),
        ("tr".to_string(), "Turkish".to_string()),
        ("uk".to_string(), "Ukrainian".to_string()),
        ("uz".to_string(), "Uzbek".to_string()),
        ("vi".to_string(), "Vietnamese".to_string()),
        ("it".to_string(), "Italian".to_string()),
        ("nb".to_string(), "Norwegian".to_string()),
        ("mt".to_string(), "Maltese".to_string()),
        ("ms".to_string(), "Malay".to_string()),
        ("ast".to_string(), "Asturian".to_string()),
        ("yo".to_string(), "Yoruba".to_string()),
        ("az".to_string(), "Azeri".to_string()),
        ("sco".to_string(), "Scots".to_string()),
        ("ne".to_string(), "Nepali".to_string()),
    ];

    //https://stackoverflow.com/questions/34215280/how-can-i-randomly-select-one-element-from-a-vector-or-array/34215930
    let sample: Vec<_> = ret.choose_multiple(&mut rand::thread_rng(), num).collect();

    //ret
    //https://stackoverflow.com/questions/38285671/unable-to-create-a-vector-and-shuffle-it

    //let mut rng = rand::thread_rng();
    //sample.shuffle(&mut rng)
    let mut ret_vec = vec![];

    //cloning all the items in the vector so it fits the return type
    for item in sample {
        ret_vec.push(item.clone());
    }

    ret_vec
}

#[test]
fn test_url_add() {
    let lang = "pt";
    let use_url = "https://".to_owned() + lang + ".wikipedia.org/w/api.php";
    assert!(use_url == "https://pt.wikipedia.org/w/api.php");
}

#[test]
fn test_get_langs() {
    let lang_vec = get_langs(4);
    //run with cargo test --nocapture to print
    println!("lang_vec: {:#?}", lang_vec);

    assert!(lang_vec.len() == 4);
}

#[test]
//use get_text for all languages, make sure all appear in the vec
fn test_get_langs2() {
    let lang_vec = get_langs(50);

    for item in &lang_vec {
        //vector test from Harmic on Stackoverflow
        //https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector
        assert!(lang_vec.iter().any(|i| &i == &item));
    }

    //run with cargo test --nocapture to print
    println!("lang_vec: {:#?}", lang_vec);
    assert!(lang_vec.len() == 50);
}

#[test]
//generates 100 numbers to make sure that they are appropriate for r,g,b
fn test_gen_range_code() {
    let mut rng = thread_rng();

    for _i in 0..100 {
        let r: f32 = rng.gen_range(0, 100) as f32 * 0.01;
        assert!((r >= 0.0) && (r <= 1.0));
    }
}

#[test]
//make sure get_langs grabs unique languages and not duplicates
fn test_get_langs_unique() {
    use itertools::Itertools;
    let from_get_langs = get_langs(10);
    let counted = from_get_langs.iter().unique();

    assert!(counted.count() == 10);
}

#[test]
fn test_choose_1() {
    let lang_vec = get_langs(10);

    let t = (*lang_vec.choose(&mut rand::thread_rng()).unwrap()).clone();

    assert!(t.0 != t.1);
}

#[test]
fn test_get_text() {
    let t = get_text("pt");
    assert!(t.len() > 0);
}

#[test]
#[should_panic]
fn test_get_text_fake() {
    let _t = get_text("fake");
}

//test to make sure that can get text for all languages in vec
#[test]
fn test_all_langs() {
    //would set to 50, but don't want to spam Wikipedia
    //this will randomly choose 4 and make sure it can get pages from the tuple
    let lang_vec = get_langs(4);
    for item in lang_vec {
        get_text(&item.0);
    }
}
