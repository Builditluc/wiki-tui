mod tests {
    #[test]
    fn it_works() {
        println!("Hello World!")
    }

    #[test]
    fn test_env() {
        use dotenv::dotenv;
        use std::env;

        dotenv::dotenv().ok();

        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
}