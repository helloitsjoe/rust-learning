use std::io;

pub struct Input {
  pub mock_responses: Vec<String>,
  pub is_mock: bool,
}

impl Input {
  pub fn new(mock_responses: Vec<&str>) -> Input {
    let mock_responses = mock_responses
      .iter()
      .map(|&res| String::from(res))
      .collect::<Vec<String>>();

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
