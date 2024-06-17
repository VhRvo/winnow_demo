use crate::common::Hex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "0xZZ";
        println!("{:}\n", input.parse::<Hex>().unwrap_err());
    }
    #[test]
    fn test2() {
        let input = "100";
        println!("{:}\n", input.parse::<Hex>().unwrap_err());
    }
    #[test]
    fn test3() {
        let input = "0b5";
        println!("{:}\n", input.parse::<Hex>().unwrap_err().to_string());
    }
}
