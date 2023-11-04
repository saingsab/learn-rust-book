struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String,  username: String) -> User {
    User {
        active: true,
        username, //when the member inside the struct is same as parameter we don't have to assign =
        email,
        sign_in_count: 1,
    }
}
fn main() {
    //  Create an instance of the struct
    let mut user1 = User { //mutable will allow update the data inside the struct
        active: true,
        username: String::from("saingsab"),
        email: String::from("saing.sab@gmail.com"),
        sign_in_count: 1
    };

    // Changing the value in the email field of a User instance
    user1.email = String::from("saing.sab1@gmail.com");

    // Updateing , copying the entire struct and replace few items we need
    // Copy the user1
    let user2 = User {
        email: String::from("saing3@gmail.com"),
        ..user1 // The ..user1 must come last to specify that any remaining fields 
               // should get their values from the corresponding fields in user1
    };
}
