// Credits => https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust

use std::{time::Duration, thread::sleep};

enum UpdateResult {
    Continue,
    QuitApplication
}

fn main() {
    let client = Box::new(MyClient {ticks_left : 4});
    let mut app = App {
        title: "My First Rust Window App".to_string(),
        client: client,
    };
    app.run();
}
// Rust doesn't have classes. 
// Lifetimes and Traits (things to learn about)

struct App{
    title:String,
    // whenever we use a trait as a property of a struct, need to give dyn keyword
    // to tell the compiler that the actual type will be decided at runtime.;
    //client: &mut dyn Client,
    client:Box<dyn Client>
}
struct MyClient {
    ticks_left: usize,
}
impl App {
    fn run (&mut self){
        println!("=== You are now playing {} ===", self.title);
        loop {
            let update_result = self.client.update();
            self.client.render();
            
            match update_result {
                UpdateResult::QuitApplication => {return;},
                UpdateResult::Continue => {}   
            }
            sleep(Duration::from_secs(1));
        }
    }
}

trait Client {
    fn update(&mut self) -> UpdateResult;
    fn render(&self);
}

impl Client for MyClient {
    fn update(&mut self) -> UpdateResult{
        self.ticks_left -=1; 
        if self.ticks_left == 0 {
            return UpdateResult::QuitApplication;
        }
        return UpdateResult::Continue;
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