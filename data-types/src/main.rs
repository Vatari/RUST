fn main() {
    // let numbers = [1, 2, 3, 4, 5, 6];
    // let apples = ["Granny Smith", "Macintosh", "Red Delicious"];
    // let seasons = ["Spring", "Summer", "Fall", "Winter"];

    // println!("Length: {}", apples.len());
    // println!("Length: {}", apples.join(", "));
    // println!("Length: {}", numbers[1]);
    // println!("Length: {:?}", seasons);
    // println!("Length: {:#?}", seasons);

    // dbg!(seasons);

    // let employee = ("Peter", 46, "Development");
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    // let (name, age, department) = employee;

    let month_days = 1..31; // 1 to 30
    let month_days = 1..=31; //1 to 31

    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letters = 'b'..='f';

    for letter in letters {
        println!("{letter}");
    }






    // let currency_rates:[f64; 0] = [];
}

