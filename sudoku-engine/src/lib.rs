use rand::Rng;

/// 
fn generate_quadrant(size: u8, is_empty: bool) -> Vec<Vec<u8>> {
    let mut quadrant: Vec<Vec<u8>> = Vec::with_capacity(size as usize);

    if is_empty {
        let row: Vec<u8> = (0..size).map(|_| 0).collect();
        for _ in 0..size {
            quadrant.push(row.clone())
        }
    } else {
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<u8> = (1..=(size * size)).collect();

        for _ in 0..size {
            let mut row: Vec<u8> = Vec::with_capacity(size as usize);
            for _ in 0..size {
                let index = rng.gen_range(0..numbers.len());
                row.push(numbers.remove(index));
            }
            quadrant.push(row)
        }
    }

    quadrant
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_generate_quadrant() {
        let result = generate_quadrant(4, false);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 4);

        println!("{result:?}");

        let mut seen: HashSet<u8> = HashSet::with_capacity(4 * 4);
        for row in result.into_iter() {
            for number in row.into_iter() {
                if seen.contains(&number) {
                    panic!("number already seen");
                } else {
                    seen.insert(number);
                }
            }
        }
    }
}
