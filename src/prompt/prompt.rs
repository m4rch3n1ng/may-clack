pub trait Prompt<T> {
	fn init(&self);

	fn out(&self, value: &T);
}
