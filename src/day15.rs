pub fn part_one(data: &str) -> usize {
    let initialization_sequence: Vec<&str> = data.split(',').collect();
    initialization_sequence.iter().map(|string| hash(string)).sum()
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

pub fn part_two(data: &str) -> usize {
    let initialization_sequence: Vec<&str> = data.split(',').collect();
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for step in initialization_sequence {
        let label = step.chars().take_while(|c| c.is_alphabetic()).collect::<String>();
        let hash = hash(&label);
        let operation = step.chars().skip_while(|c| c.is_alphabetic()).next().unwrap();
        let box_lens = &mut boxes[hash as usize];

        match operation {
            '-' => {
                if let Some(index) = box_lens.iter().position(|lens| lens.label == label) {
                    box_lens.remove(index);
                }
            }
            '=' => {
                let value: usize = step.chars().skip_while(|c| c.is_alphabetic()).skip(1).collect::<String>().parse().unwrap();
                let lens = Lens { label: label.clone(), focal_length: value };
                if let Some(index) = box_lens.iter().position(|lens| lens.label == label) {
                    box_lens[index] = lens;
                } else {
                    box_lens.push(lens);
                }
            }
            _ => (),
        }
    }

    boxes.iter().enumerate().map(|(i, box_lens)| {
        box_lens.iter().enumerate().map(|(j, lens)| {
            (i + 1) * (j + 1) * lens.focal_length
        }).sum::<usize>()
    }).sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part_one_examples_one() {
        let training: &str = indoc! {"HASH"};

        assert_eq!(52, part_one(training));
    }

    #[test]
    fn part_one_examples_two() {
        let training: &str = indoc! {"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"};

        assert_eq!(1320, part_one(training));
    }
}



