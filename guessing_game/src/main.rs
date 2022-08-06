

struct Employee {
    first_name: String,
    second_name: String,
    experience: u8

}

fn main() {
    let employee1 = Employee {
        first_name : String::from("John"),
        second_name: String:: from("Doe"),
        experience: 10
    };
    get_display_detail(employee1)
}

fn get_display_detail(employee: Employee){
println!("first name is {} ", employee.first_name);
println!("second name is {} ", employee.second_name);
println!("experince is {} ", employee.experience);
}