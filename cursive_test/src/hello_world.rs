use cursive::views::TextView;
use cursive::Cursive;

pub fn main() {
    let mut siv = cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Add a simple view
    siv.add_layer(TextView::new(
        "Hello World!\n\
         Press q to quit the application.",
    ));

    // Run the event loop
    siv.run();
}
