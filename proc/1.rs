fn main() {
    //Создаём переменную для хранения количества людей
    let people = 10;
    //Добавляем людей в супермаркет))
    let mut market = people;
    println!("В супермаркете: {}", market);
    //Люди ушли (
    let market = 0;
    println!("В супермаркете: {}", market);
}