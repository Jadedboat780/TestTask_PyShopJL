use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;
// количество фиксаций в течение матча
const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;
// вероятность изменения счета при каждой фиксации
const PROBABILITY_HOME_SCORE: f64 = 0.45;
// вероятность того, что изменение счета будет для домашней команды
const OFFSET_MAX_STEP: i32 = 3;    // определяет максимальный шаг (разницу во времени) между последовательными фиксациями

// начальная фиксация состояния счета
const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

// структура для представления состояния счета
#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

// структура для представления фиксации состояния счета
#[derive(Debug, Clone, Copy)]
pub struct Stamp {
    offset: i32,
    score: Score,
}

// функция для генерации новой фиксации состояния счета
pub fn generate_stamp(previous_value: Stamp) -> Stamp {
    // генерация случайных значений для изменения состояния счета
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    // создание новой фиксации на основе предыдущей и сгенерированных значений
    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },
            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },
        },
    }
}

// функция для генерации списка фиксаций состояния счета в течение матча
pub fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}


// функция для получения состояния счета на определенный момент времени
pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    // инициализация переменных для хранения ближайшей фиксации и минимальной разницы
    let mut closest_stamp = &game_stamps[0];
    let mut min_difference = i32::MAX;

    // итерация по всем фиксациям в списке
    for stamp in game_stamps {
        // разница между временем фиксации и указанным временем
        let difference = (stamp.offset - offset).abs();

        // если найдена более близкая фиксация, обновляем переменные
        if difference < min_difference {
            min_difference = difference;
            closest_stamp = stamp;
        }
    }

    // возвращаем состояние счета из ближайшей фиксации
    (closest_stamp.score.home, closest_stamp.score.away)
}


#[cfg(test)]
mod tests {
    use super::*;

    // точное совпадение времени
    #[test]
    fn test_get_score_exact_offset() {
        let game_stamps = vec![
            Stamp { offset: 0, score: Score { home: 1, away: 2 } },
            Stamp { offset: 100, score: Score { home: 3, away: 4 } },
            Stamp { offset: 200, score: Score { home: 5, away: 6 } },
        ];

        let result = get_score(&game_stamps, 100);

        assert_eq!(result, (3, 4));
    }

    // ближайшее время
    #[test]
    fn test_get_score_closest_offset() {
        let game_stamps = vec![
            Stamp { offset: 0, score: Score { home: 1, away: 2 } },
            Stamp { offset: 100, score: Score { home: 3, away: 4 } },
            Stamp { offset: 200, score: Score { home: 5, away: 6 } },
        ];

        // запрос на время, которое между двумя фиксациями (между 100 и 200)
        let result = get_score(&game_stamps, 150);
        assert_eq!(result, (3, 4));
    }
}