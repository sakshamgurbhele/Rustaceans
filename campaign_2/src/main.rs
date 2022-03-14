
struct Userdetails {
    firstname: String,
    lastname: String,
}

fn main() {
    let user1 = Userdetails {
        firstname : String::from("Saksham"),
        lastname : String::from("Gurbhele")
    };
    println!("My full name is {} {}", user1.firstname, user1.lastname);
}
