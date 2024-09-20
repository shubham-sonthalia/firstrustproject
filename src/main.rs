use std::{time::Duration, thread::sleep};

fn main() {
    let mut client = MyClient {ticks_left : 4};
    let mut app = App {
        title: "My First Rust Window App".to_string(),
        running: true,
        client: &mut client,
    };
    app.run();
}
// Rust doesn't have classes. 
// Lifetimes and Traits (things to learn about)

struct App<'a> {
    title:String,
    running:bool,
    // whenever we use a trait as a property of a struct, need to give dyn keyword
    // to tell the compiler that the actual type will be decided at runtime.;
    client: &'a mut dyn Client,
}
struct MyClient {
    ticks_left: usize,
}
impl App<'_> {
    fn run (&mut self){
        println!("=== You are now playing {} ===", self.title);
        loop {
            self.client.update(self);
            self.client.render();

            if !self.running {
                break;
            }
            sleep(Duration::from_secs(1));
        }
    }
}

trait Client {
    fn update(&mut self, app: &mut App);
    fn render(&self);
}

impl Client for MyClient {
    fn update(&mut self, app: &mut App){
        self.ticks_left -=1; 
        if self.ticks_left == 0 {
            app.running = false;
        }
    }
    fn render(&self){
        if self.ticks_left > 0 {
            println!("You turn the crank ...");
        }
        else{
            println!("jack pops out of the box!!");
        }
    }
}

// impl <struct name> 
// impl <trait name> for <struct name>