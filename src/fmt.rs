#[derive(Copy, Clone, Default)]
pub(crate) struct DocumentFormatter {
    pub(crate) multiline_array: bool,
    is_value: bool,
}
