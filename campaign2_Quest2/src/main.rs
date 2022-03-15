
struct Userdetails {
    firstname: String,
    lastname: String,
}

fn main() {
    let user = Userdetails {
        firstname : String::from("Saksham"),
        lastname : String::from("Gurbhele")
    };
    println!("My full name is {} {}", user.firstname, user.lastname);
}
