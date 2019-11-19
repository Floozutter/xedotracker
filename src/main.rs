// test building with cursive using cursive/examples/hello_world.rs

use cursive::views::TextView;
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Add a simple view
    siv.add_layer(TextView::new(
        "Hello, world!\n\
         Press q to quit the application.",
    ));

    // Run the event loop
    siv.run();
}