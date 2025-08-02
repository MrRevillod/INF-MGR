pub struct Pagination<T> {
    pub items: Vec<T>,
    pub current_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub has_next: bool,
    pub has_previous: bool,
}
