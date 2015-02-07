
use std::collections::HashMap;
use ui::markup::tags;


// Library
pub struct Library {
    pub views: HashMap<String, tags::View>,
    pub templates: HashMap<String, tags::Template>,
}

impl Library {

    pub fn new(views: HashMap<String, tags::View>, templates: HashMap<String, tags::Template>) -> Library {
        Library {
            views: views,
            templates: templates
        }
    }

    pub fn merge(&mut self, other: Library) {
        for (key, val) in other.views.into_iter() {
            self.views.insert(key, val);
        }

        for (key, val) in other.templates.into_iter() {
            self.templates.insert(key, val);
        }
    }
}
