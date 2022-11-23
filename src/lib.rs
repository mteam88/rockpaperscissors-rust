#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_choice_creation() {
        let choice = Choice::new("rock".to_string()).unwrap();
        assert_eq!(choice.text, "rock");
        let choice = Choice::new("paper".to_string()).unwrap();
        assert_eq!(choice.text, "paper");
        let choice = Choice::new("scissors".to_string()).unwrap();
        assert_eq!(choice.text, "scissors");
    }

    #[test]
    fn test_choice_creation_shortcut() {
        let choice = Choice::new("r".to_string()).unwrap();
        assert_eq!(choice.text, "rock");
        let choice = Choice::new("p".to_string()).unwrap();
        assert_eq!(choice.text, "paper");
        let choice = Choice::new("s".to_string()).unwrap();
        assert_eq!(choice.text, "scissors");
    }

    #[test]
    fn test_choice_creation_fails() {
        let error = Choice::new("gibberish".to_string()).unwrap_err();
        assert_eq!(error, String::from("Invalid choice"));
    }

    #[test]
    fn test_choice_comparison() {
        let choice1 = Choice::new("rock".to_string()).unwrap();
        let choice2 = Choice::new("paper".to_string()).unwrap();
        let choice3 = Choice::new("scissors".to_string()).unwrap();
        assert_eq!(choice1.compare(&choice2), Outcome::Lose);
        assert_eq!(choice1.compare(&choice3), Outcome::Win);
        assert_eq!(choice2.compare(&choice1), Outcome::Win);
        assert_eq!(choice2.compare(&choice3), Outcome::Lose);
        assert_eq!(choice3.compare(&choice1), Outcome::Lose);
        assert_eq!(choice3.compare(&choice2), Outcome::Win);
        assert_eq!(choice1.compare(&choice1), Outcome::Draw);
        assert_eq!(choice2.compare(&choice2), Outcome::Draw);
        assert_eq!(choice3.compare(&choice3), Outcome::Draw);
    }
}

#[derive(Debug, PartialEq)]
pub struct Choice {
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Choice {
    pub fn new(text: String) -> Result<Choice, String> {
        if text == "r" || text == "rock" {
            Ok(Choice { text: "rock".to_string() })
        } else if text == "p" || text == "paper" {
            Ok(Choice { text: "paper".to_string() })
        } else if text == "s" || text == "scissors" {
            Ok(Choice { text: "scissors".to_string() })
        } else {
            Err(String::from("Invalid choice"))
        }
    }

    pub fn compare(&self, other: &Choice) -> Outcome {
        if self.text == "rock" {
            if other.text == "rock" {
                Outcome::Draw
            } else if other.text == "paper" {
                Outcome::Lose
            } else if other.text == "scissors" {
                Outcome::Win
            } else {
                panic!("Invalid choice");
            }

        } else if self.text == "paper" {
            if other.text == "rock" {
                Outcome::Win
            } else if other.text == "paper" {
                Outcome::Draw
            } else if other.text == "scissors" {
                Outcome::Lose
            } else {
                panic!("Invalid choice");
            }

        } else if self.text == "scissors" {
            if other.text == "rock" {
                Outcome::Lose
            } else if other.text == "paper" {
                Outcome::Win
            } else if other.text == "scissors" {
                Outcome::Draw
            } else {
                panic!("Invalid choice");
            }

        } else {
            panic!("Invalid choice");
        }
    }
}