use std::env;

pub fn get_env(var_name: &str) -> String{

    let val: String = env::var(var_name).unwrap();

    return val;
}
