mod utils;

use utils::environment::get_env;
use utils::file_helper::read_file;


//Features not adding/out of scope for this project:
// - Cronjobs and scheduling tasks
// - 

//Features that will be added:
//- Different types of notifications (email, telegram, discord, etc)
//- Message templates, and persisting to database
// -Queuing system, retries, and error handling



fn main() {
    println!("{}", get_env("HOME"));
    println!("{}", read_file("my_file.txt"));
}
