// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Self::valid_input(&title, &description, &status);
        Ticket {
            title,
            description,
            status,
        }
    }

    // I am aware that this is ridiculously verbs but it's fine for this exercise. I guess :|
    fn valid_input(title: &str, description: &str, status: &str) {
        Self::valid_title_content(title);
        Self::valid_title_length(title);
        Self::valid_description_content(description);
        Self::valid_description_length(description);
        Self::valid_status(status);
    }
    fn valid_title_content(title: &str) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
    }
    fn valid_title_length(title: &str) {
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
    }
    fn valid_description_content(description: &str) {
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
    }
    fn valid_description_length(description: &str) {
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
    }
    fn valid_status(status: &str) {
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, title: String) {
        Self::valid_title_length(&title);
        Self::valid_title_content(&title);
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        Self::valid_description_content(&description);
        Self::valid_description_length(&description);
        self.description = description;
    }

    pub fn set_status(&mut self, status: String) {
        Self::valid_status(&status);
        self.status = status;
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{ overly_long_description, overly_long_title, valid_description, valid_title };

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title(
            overly_long_title()
        )
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description(
            overly_long_description()
        )
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
