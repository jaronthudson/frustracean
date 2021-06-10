Jeff Thomson
Final Project: Frustracean
https://github.com/jaronthudson/frustracean/
MIT License: https://github.com/jaronthudson/frustracean/blob/main/LICENSE

My project is a language guessing game. The basic idea is to generate random text from Wikipedia
in one of fifty languages, then display the text along with four options for guessing the language.
I chose 4 somewhat arbitrarily -- to make it not too difficult -- and it can be changed fairly
easily (aside from the 'buttons', which are hard-coded).The project is meant to be a fun language
guessing game that also teaches people to recognize different languages. 

This project was the original/fallback idea mentioned in my course proposal: I found the idea I wrote about in
the proposal pretty interesting, but ultimately didn't think I knew enough about NLP to be able
to do the project without asking for a lot of assistance. Bart Ok'd the project change.

How to build the project:
It is important to have the NotoSans-Regular.ttf (free font from Google) in the main folder 
(where the cargo.toml is located). It is important also to have an internet connection: it will return
an error if it is unable to make requests from Wikipedia. Other than that, it should be fairly straightforward
to build: install rust, then run "cargo build" and "cargo run" in the main folder.

The behind-the-scenes language work uses the Wikipedia crates for connecting to Wikipedia and filtering 
out the html tags. I created a large vector of tuples of languages ("en", "English"), and by constructing
a url out of the language abbreviation content in different languages can be requested from Wikipedia.
I then randomly get a page, then get the content from the page. I split the strings into a vector, then
return the longest string from the vector. This creates strings of widely varying lengths: I choose the 
longest to avoid getting strings that are far too short. Some languages have many very short pages
(such as Nahua and Ojibwe, which I ended up removing from my language list) while others commonly
have much longer strings. Originally I truncated the text to a certain length, which looked nice in
the output window, but it would occasionally (maybe 1/20 times) panic, due to a character misalignment.
This was a very tricky problem to solve: I didn't notice the panic until after I had written the truncate
code and moved on, and thought that it was occurring elsewhere. Ultimately, running a backtrace allowed me to
figure out that it was due to the truncate code.

For graphics, I used the Speedy2d graphics framework. I found it from a Reddit post on the Rust subreddit for
good frameworks for someone new to programming games. In retrospect, I would maybe have chosen ggez 
(another game framework), since it has been in use for some time, there's a large team behind it,
and also there's a lot more documentation related to it. However, for just being created by one person,
Speedy2d is very impressive!

The on-screen graphics look fairly simple, but actually took me a fair amount of time to program, and it was
a good amount of effort to understand the Speedy2d graphics framework. I started out by playing around with 
one of the examples (https://github.com/QuantumBadger/Speedy2D/blob/master/examples/user_events.rs), I still 
use some of the variable names and basic setup, and with that figured out how different things worked.

An example of code interacting with the graphics framework:
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
        
This code is for one of the buttons on the screen. The location is hard-coded: this code is checked when the
mouse button is pressed. If the location of the mouse pointer matches up with the location of the button, then
the code within is run. It checks to see if the language shown at the button matches up with the actual language
of the displayed text. If it is, it updates the correct value and resets the language and buttons. If not, it
increments the incorrect counter and changes the text on the button.

I did a lot of testing during development. Much of it was to make sure that strings were correct and, the 
random selection was occurring correctly and there were no errors in changing languages. I have some of 
these tests at the bottom of my main.rs file. I also did a great of testing with the graphics, though 
I am less sure about how to write that out as tests in my main.rs file. Much of that testing involved
compiling and running to see how things looked and operated in the game window.

The main thing I would like to add to the game would be the option to take the displayed text and run it through
Google Translate or something else. It wouldn't be a perfect translation, but it would be cool to see a guess
at the English translation of the displayed text. I also was disappointed that my Truncate code was occasionally
causing the program to panic. I think this *may* be able to be fixed by checking to see if the result from Truncate
would cause a panic, and continuing to generate new text until it doesn't panic from a character misalignment.

I'm pretty happy with the results! I think it looks deceptively basic, since there was a lot of effort put into
making sure that things were correct with the text and the graphics. I kind of naively thought that it would be
simple to do things with 2d graphics, found out that there was a ton that I didn't know (and in general a lot
I didn't know about programming a game that runs in a game loop). This was the first game I've made
(aside from 'Uno' in CS202) and the first program I've written that included working with any sort of graphics
framework. It was a very good experience! I learned a lot and it was probably the most fun programming
challenge that I've had. 
