pub struct Pagination<T> {
    pub items: Vec<T>,
    pub current_page: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_previous: bool,
}

pub const DEFAULT_PAGE_SIZE: u64 = 10;
