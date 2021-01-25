#[derive(Clone)]
pub(crate) struct NoDebug<T>(T);

impl<T> std::fmt::Debug for NoDebug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "...")
    }
}

impl<T> std::ops::Deref for NoDebug<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for NoDebug<T> {
    fn from(w: T) -> Self {
        NoDebug(w)
    }
}
