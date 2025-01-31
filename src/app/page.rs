pub mod home;

use eframe::egui::Ui;

pub trait Page {
    fn ui(&mut self, ui: &mut Ui);
}

pub struct NavigationController {
    pages: Vec<Box<dyn Page>>,
}

impl NavigationController {
    pub fn new(init_page: Box<dyn Page>) -> Self {
        Self {
            pages: vec![init_page],
        }
    }

    pub fn current_page(&mut self) -> &mut Box<dyn Page> {
        self.pages.last_mut().unwrap()
    }

    pub fn push_page(&mut self, page: Box<dyn Page>) {
        self.pages.push(page);
    }

    pub fn pop_page(&mut self) {
        self.pages.pop();
    }
}
