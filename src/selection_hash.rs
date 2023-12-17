use sha2::{Sha256, Digest};

pub fn selection_hash(n: usize, f: usize) {
    // переменная для отслеживания найденных значений хеша
    let mut found_count = 0;

    for i in 1.. {
        // вычисляем хеш
        let hash = calculate_sha256_hash(i);

        // проверяем, заканчивается ли дайджест хеша N символами нуля
        if hash.ends_with(&"0".repeat(n)) {
            println!("{}, \"{}\"", i, hash);

            // увеличиваем счетчик найденных значений
            found_count += 1;

            // если достигнуто заданное количество значений F, завершаем программу
            if found_count == f {
                break;
            }
        }
    }
}


// функция для вычисления хеша
fn calculate_sha256_hash(number: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string());
    let result = hasher.finalize();
    return format!("{:x}", result)
}