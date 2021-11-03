struct Baby {
    name: String,
    gender: String,
    birthday: String,
    weight: String,
}

impl Baby {
    fn cry(&self) {
        println!("Hello World !");
        println!("My name is {}. I'm a {}.", self.name, self.gender);
        println!("I was born at {}, and I'm {}.", self.birthday, self.weight);
    }
}

fn main() {
    let baby = Baby {
        name: String::from("徐寻墨"),
        gender: String::from("女孩"),
        birthday: String::from("阳历2017-02-28 09:13:00, 农历丁酉年二月初三"),
        weight: String::from("7斤9两")
    };
    baby.cry()
}
