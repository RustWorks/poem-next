/// Newtype for collecting path segments into a path
#[derive(Debug, Clone, PartialEq)]
pub struct Path(String);

impl Path {
    /// Create new path from a prefix
    pub(crate) fn prefix(value: &str) -> Self {
        Path(value.to_string())
    }

    pub(crate) fn append(self, segment: &str) -> Path {
        if self.0.is_empty() {
            Path(segment.to_string())
        } else {
            let mut path = self.0.trim_end_matches('/').to_string();
            path.push('/');
            path.push_str(segment.trim_start_matches('/'));
            Path(path)
        }
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}