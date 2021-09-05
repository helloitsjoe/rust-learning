use std::io;

#[derive(Clone)]
pub struct Input {
  pub mock_responses: Vec<String>,
  pub is_mock: bool,
}

impl Input {
  pub fn new(mock_responses: Vec<String>) -> Input {
    let is_mock = mock_responses.len() > 0;
    Input {
      mock_responses,
      is_mock,
    }
  }

  pub fn get_input(&mut self) -> String {
    if self.is_mock {
      self.mock_responses.pop().unwrap_or(String::from(""))
    } else {
      let mut buf = String::new();

      io::stdin()
        .read_line(&mut buf)
        .expect("Something went wrong");

      String::from(buf.trim())
    }
  }
}
