/* Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: среднее значение; медиану (значение элемента из середины списка после его сортировки); моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз; HashMap будет полезна в данном случае).*/

use std::collections::HashMap;
fn average(lst: &[i32]) -> f32 {
    lst.iter().sum::<i32>() as f32 / lst.len() as f32
}

fn median(lst: &[i32]) -> f64 {
    let mut sorted_lst = lst.to_vec(); // Создаём копию, чтобы не менять оригинал
    sorted_lst.sort();

    let len = sorted_lst.len();
    if len % 2 == 0 {
        let ind_left = len / 2 -1;
        let ind_right = len / 2;
        (sorted_lst[ind_left] + sorted_lst[ind_right]) as f64 / 2.0 as f64
    } else {
        sorted_lst[len / 2] as f64
    }
}


fn mode(lst: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in lst {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}


fn main() {
    let mut lst = vec![9, 6, 4, 1, 2, 5, 8, 3, 7];
    println!("AVERAGE: {}", average(&lst));

    // let mut lst_for_median = lst.clone(); // Чтобы сохранить порядок исходного списка
    // println!("MEDIAN: {}", median(&mut lst_for_median));
    println!("MEDIAN: {}", median(&mut lst));
    println!("MODE: {:#?}", mode(&lst));
}
