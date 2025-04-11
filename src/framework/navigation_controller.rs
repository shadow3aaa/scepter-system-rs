use std::ptr;

use eframe::{egui::Ui, Frame, Storage};

use super::page::Page;

pub struct NavigationController {
    pages: Vec<Box<dyn Page>>,
    self_ref: *mut NavigationController,
}

impl NavigationController {
    #[allow(clippy::unnecessary_box_returns)]
    pub fn new(init_page: Box<dyn Page>) -> Box<Self> {
        let mut nav_controller = Box::new(Self {
            pages: vec![init_page],
            self_ref: ptr::null_mut(),
        });
        nav_controller.self_ref = &mut *nav_controller;
        nav_controller
    }

    pub fn set_current_page(&mut self, page: Box<dyn Page>, storage: &mut dyn Storage) {
        while !self.pages.is_empty() {
            self.pop(storage);
        }
        self.push(page, storage);
    }

    fn current_page(&self) -> &dyn Page {
        self.pages.last().unwrap().as_ref()
    }

    fn current_page_mut(&mut self) -> &mut dyn Page {
        self.pages.last_mut().unwrap().as_mut()
    }

    pub fn push(&mut self, mut page: Box<dyn Page>, storage: &mut dyn Storage) {
        page.on_enter(storage);
        self.pages.push(page);
    }

    pub fn pop(&mut self, storage: &mut dyn Storage) {
        if let Some(mut page) = self.pages.pop() {
            page.on_exit(storage);
        }
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    fn safe_self_ref(&self) -> &'static mut Self {
        unsafe { &mut *self.self_ref } // this is fucking safe if nobody fucks the memory
    }

    pub fn pages_mut(&mut self) -> impl IntoIterator<Item = &mut Box<dyn Page>> {
        &mut self.pages
    }

    pub fn top_panel_ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let nav_ref = self.safe_self_ref();
        self.current_page_mut().top_panel(ui, frame, nav_ref);
    }

    pub fn side_panel_ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let nav_ref = self.safe_self_ref();
        self.current_page_mut().side_panel(ui, frame, nav_ref);
    }

    pub fn show_side_panel(&self) -> bool {
        self.current_page().show_side_panel()
    }

    pub fn main_ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        let nav_ref = self.safe_self_ref(); // this is fucking safe if nobody fucks the memory
        self.current_page_mut().main(ui, frame, nav_ref);
    }
}
