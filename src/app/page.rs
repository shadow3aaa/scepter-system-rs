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

    pub fn set_current_page(&mut self, page: Box<dyn Page>) {
        self.pages.clear();
        self.pages.push(page);
    }

    pub fn current_page(&mut self) -> &mut Box<dyn Page> {
        self.pages.last_mut().unwrap()
    }

    pub fn push(&mut self, page: Box<dyn Page>) {
        self.pages.push(page);
    }

    pub fn pop(&mut self) {
        self.pages.pop();
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }
}
