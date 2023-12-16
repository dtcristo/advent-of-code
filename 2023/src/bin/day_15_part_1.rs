fn main() {
    let input = include_bytes!("../../input/day_15");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &[u8]) -> u32 {
    input
        .split(|&byte| byte == b',')
        .map(|slice| hash(slice) as u32)
        .sum()
}

fn hash(slice: &[u8]) -> u8 {
    slice.iter().fold(0, |acc, &byte| {
        (((acc as u16 + byte as u16) * 17) % 256) as u8
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::*;

    #[test]
    fn test_solution() {
        let input = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = solution(input);
        let expected = 1320;
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(b"rn=1", 30)]
    #[case(b"cm-", 253)]
    #[case(b"qp=3", 97)]
    fn test_hash(#[case] input: &[u8], #[case] expected: u8) {
        let result = hash(input);
        assert_eq!(result, expected);
    }
}
