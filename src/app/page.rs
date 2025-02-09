pub mod home;
pub mod mind;

use std::ptr;

use eframe::egui::Ui;

pub trait Page {
    /// Provides some top panel tools
    #[allow(unused_variables)]
    fn top_panel_ui(&mut self, ui: &mut Ui, nav_controller: &mut NavigationController) {}
    /// Main of the page
    fn main_ui(&mut self, ui: &mut Ui, nav_controller: &mut NavigationController);
}

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

    pub fn set_current_page(&mut self, page: Box<dyn Page>) {
        self.pages.clear();
        self.pages.push(page);
    }

    fn current_page(&mut self) -> &mut Box<dyn Page> {
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

    fn safe_self_ref(&self) -> &'static mut NavigationController {
        unsafe { &mut *self.self_ref } // this is fucking safe if nobody fucks the memory
    }

    pub fn top_panel_ui(&mut self, ui: &mut Ui) {
        let nav_ref = self.safe_self_ref();
        self.current_page().top_panel_ui(ui, nav_ref);
    }

    pub fn main_ui(&mut self, ui: &mut Ui) {
        let nav_ref = self.safe_self_ref(); // this is fucking safe if nobody fucks the memory
        self.current_page().main_ui(ui, nav_ref);
    }
}
