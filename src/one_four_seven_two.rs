// https://leetcode.com/problems/design-browser-history/

struct BrowserHistory {
    current_index: usize,
    pages: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            current_index: 0,
            pages: vec![homepage],
        }
    }

    fn visit(&mut self, url: String) {
        self.current_index += 1;

        self.pages.drain(self.current_index..);
        self.pages.push(url)
    }

    fn back(&mut self, steps: i32) -> String {
        if self.current_index >= steps as usize {
            self.current_index -= steps as usize;
        } else {
            self.current_index = 0;
        }
        self.pages[self.current_index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current_index += steps as usize;
        if self.current_index >= self.pages.len() {
            self.current_index = self.pages.len() - 1
        }
        self.pages[self.current_index].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */

#[allow(dead_code)]
pub fn main() {
    let mut browser_history = BrowserHistory::new(String::from("zav.com"));
    browser_history.visit(String::from("kni.com"));
    assert_eq!(browser_history.back(7), String::from("zav.com"));
    assert_eq!(browser_history.back(7), String::from("zav.com"));
    assert_eq!(browser_history.forward(5), String::from("kni.com"));
    assert_eq!(browser_history.forward(1), String::from("kni.com"));
    browser_history.visit(String::from("pwrrbnw.com"));
    browser_history.visit(String::from("mosohif.com"));
    assert_eq!(browser_history.back(9), String::from("zav.com"));

    let mut browser_history = BrowserHistory::new(String::from("leetcode.com"));
    browser_history.visit(String::from("google.com")); // You are in "leetcode.com". Visit "google.com"
    browser_history.visit(String::from("facebook.com")); // You are in "google.com". Visit "facebook.com"
    browser_history.visit(String::from("youtube.com")); // You are in "facebook.com". Visit "youtube.com"
    assert_eq!(browser_history.back(1), String::from("facebook.com")); // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
    assert_eq!(browser_history.back(1), String::from("google.com")); // You are in "facebook.com", move back to "google.com" return "google.com"
    assert_eq!(browser_history.forward(1), String::from("facebook.com")); // You are in "google.com", move forward to "facebook.com" return "facebook.com"
    browser_history.visit(String::from("linkedin.com")); // You are in "facebook.com". Visit "linkedin.com"
    assert_eq!(browser_history.forward(2), String::from("linkedin.com")); // You are in "linkedin.com", you cannot move forward any steps.
    assert_eq!(browser_history.back(2), String::from("google.com")); // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
    assert_eq!(browser_history.back(7), String::from("leetcode.com")); // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"
}
