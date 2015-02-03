
use std::collections::HashMap;
use ui::markup::tags;


// Library
pub struct Library {
    views: HashMap<String, tags::View>,
    templates: HashMap<String, tags::Template>,
}

impl Library {

    pub fn new(views: HashMap<String, tags::View>, templates: HashMap<String, tags::Template>) -> Library {
        Library {
            views: views,
            templates: templates
        }
    }
}
