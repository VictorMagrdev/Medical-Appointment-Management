
pub trait GetDb<T> {
    fn get_db(&self) -> &T;
}