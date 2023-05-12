pub fn function(pos: &str) -> u32 {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7p1_works() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        let res = function(inp);
        assert_eq!(res, 37);
    }
    #[test]
    #[ignore]
    fn d7p2_works() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        let res = function(inp);
        assert_eq!(res, 168_0);
    }
}
