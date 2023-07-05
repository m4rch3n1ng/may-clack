pub trait Prompt<T> {
	// todo error handling
	fn init(&self);

	fn out(&self, value: T);
}
