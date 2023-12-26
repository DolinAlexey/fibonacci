#![allow(dead_code)]
/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/

fn main() {
    fib(8);
    fib_second(8);
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn fib_second(n: u32) -> Vec<u32> {
    //let mut vector: <u32> = Vec::new();
    let mut vector = Vec::new();
    for i in 0..n {
        vector.push(fib(i));
    }
    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
        assert_eq!(fib_second(0), []);
        assert_eq!(fib_second(1), [0]);
        assert_eq!(fib_second(2), [0, 1]);
        assert_eq!(fib_second(8), [0, 1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(
            fib_second(16),
            [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610]
        );
    }
}
