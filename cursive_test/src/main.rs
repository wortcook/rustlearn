mod hello_world;
mod logs;
mod lorem;
mod ctrl_c;

fn main() {
    // read the first argument from the terminal
    let args: Vec<String> = std::env::args().collect();
    let example = &args[1];

    // find a class that matches the argument
    // run the class
    if example == "hello_world" {
        hello_world::main();
    }else if example == "logs" {
        logs::main();
    }else if example == "lorem" {
        lorem::main();
    }else if example=="ctrl_c"{
        ctrl_c::main();
    }
}
