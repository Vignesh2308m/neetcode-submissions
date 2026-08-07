impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs.join("|")
    }

    pub fn decode(s: String) -> Vec<String> {
        s.split("|").map(|s| s.to_string()).collect()
    }
}
