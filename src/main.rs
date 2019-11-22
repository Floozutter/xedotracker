// test build with cursive using cursive/examples/hello_world.rs

use cursive::views::TextView;
use cursive::Cursive;

mod beeptest;

fn main() {
    let mut siv = Cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Test sound by pressing `s`
    siv.add_global_callback('s', |_c| {
        beeptest::run();
    });

    // Add a simple view
    siv.add_layer(TextView::new(
        "Hello, world!\n\
         Press q to quit the application.\n\
	 Press s to test sound.",
    ));

    // Run the event loop
    siv.run();
}
