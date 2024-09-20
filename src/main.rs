use std::{time::Duration, thread::sleep};

fn main() {
    let mut app = App {
        title: "My First Rust Window App".to_string(),
        ticks_left: 4, 
        running: true,
    };
    println!("=== You are now playing {} ===", app.title);

    loop {
        app.update();
        app.render();
        // called an associated function using the :: operator
        // App::awyiin();
        
        if !app.running{
            break;
        }
        sleep(Duration::from_secs(1));
    }
}
// Rust doesn't have classes. 
// Lifetimes and Traits (things to learn about)

struct App {
    title:String,
    ticks_left: usize, 
    running:bool,
}

impl App {

    // a method because it uses self 'caller'
    fn update(&mut self) {
        self.ticks_left -= 1; 
        if self.ticks_left == 0{
            self.running = false;
        }
    }

    // a method because it uses self caller.
    fn render(&self) {
        if self.ticks_left > 0 {
            println!("You turn the crank...");
        }
        else{
            println!("Jack POPS OUT OF THE BOX");
        }
    }

    // an associated function which needs to be called with :: and not '.'
    fn awyiin(){
        println!("1");
    }
}