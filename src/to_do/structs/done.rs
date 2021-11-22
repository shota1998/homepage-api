use super::base::Base;

/// This struct defines a to do item for a done to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Done {
    pub super_struct: Base
}

impl Done {

    /// The constructor for the Done struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    ///
    /// # Returns
    /// (Done): the constructed Done struct
    pub fn new(input_title: String) -> Done {
        let input_status: String = String::from("done");
        let base:         Base   = Base::new(input_title, input_status);
        return Done{super_struct: base}
    }
}

#[cfg(test)]
mod done_test {
     use super::Done;

     #[test]
     fn new() {
         let title:           String = String::from("washing");
         let expected_title:  String = String::from("washing");
         let expected_status: String = String::from("done");

         let done: Done = Done::new(title);
         assert_eq!(expected_status, done.super_struct.status);
         assert_eq!(expected_title, done.super_struct.title);
     }
}