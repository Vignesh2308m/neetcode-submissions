impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut out = String::new();

        for i in strs {
            out.push_str(&i.len().to_string());
            out.push('#');
            out.push_str(&i);
        }

        out
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut out = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            // Find '#'
            let mut j = i;

            while bytes[j] != b'#' {
                j += 1;
            }

            // Get the length before '#'
            let len: usize = s[i..j].parse().unwrap();

            // Move past '#'
            j += 1;

            // Extract exactly `len` bytes
            let string = s[j..j + len].to_string();

            out.push(string);

            // Move to the next encoded string
            i = j + len;
        }

        out
    }
}