fn main() {
    #[derive(Debug)] //enables uses of printline! for structs
    //declare struct
    struct User {
        username: String,
        email: String,
        login_times: u32,
        active: bool,
    }

    //initialize mutable struct
    let mut user1 = User {
        username: String::from("Mario"),
        email: String::from("mgalactico@outlook.com"),
        login_times: 0,
        active: true,
    };

    //print and update struct
    println!("User 1: {:?}", user1);
    println!("User 1 email: {:#?}", user1.email);
    user1.email = String::from("another email");
    println!("User 1: {:?}", user1);

    //as function parameters match struct field names there is no need to list them below
    fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            login_times: 0,
            active: true,
        }
    }

    //call function to create new user
    let user2 = build_user(String::from("Diego"), String::from("jhsdjh@yahoo.com"));
    println!("User 2: {:?}", user2);

    //initialize a user using another user's fields
    let user3 = build_user(user1.username, user2.email);
    println!("User 3: {:?}", user3);
}
