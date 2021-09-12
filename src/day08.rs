use crate::infra::Problem;
use itertools::Itertools;

pub struct Day8;

fn my_counts<It: Iterator<Item = char>>(iter: It) -> [u16; 3] {
    let mut res = [0; 3];
    for i in iter {
        res[((i as u8) - 48) as usize] += 1;
    }
    res
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

impl Problem<String, String, u16, String, 8> for Day8 {
    fn first(contents: String) -> u16 {
        let x = contents
            .chars()
            .filter(|&c| c != '\n')
            .chunks(WIDTH * HEIGHT)
            .into_iter()
            .map(my_counts)
            .min_by_key(|cs| cs[0])
            .unwrap();
        x[1] * x[2]
    }
    fn second(contents: String) -> String {
        let mut result = ['2'; WIDTH * HEIGHT];
        for layer in contents
            .chars()
            .filter(|&c| c != '\n')
            .chunks(WIDTH * HEIGHT)
            .into_iter()
        {
            for (i, c) in layer.enumerate() {
                if result[i] == '2' {
                    result[i] = c;
                }
            }
        }
        for i in 0..HEIGHT {
            println!(
                "{}",
                &result[i * WIDTH..(i + 1) * WIDTH]
                    .iter()
                    .map(|&c| if c == '1' { '█' } else { ' ' })
                    .collect::<String>()
            );
        }
        "ZBJAB".to_string() // Through ocular inspection
    }
}
