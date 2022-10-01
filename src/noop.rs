#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct NoopHandler(());

impl NoopHandler {
    pub(crate) fn new() -> Self {
        NoopHandler(())
    }
}