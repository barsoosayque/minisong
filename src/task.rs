use iocraft::prelude::*;

#[derive(Debug)]
pub struct Task<T: Unpin + Send + Sync + 'static> {
    state: State<TaskStatus<T>>,
}

impl<T: Unpin + Send + Sync + 'static> Task<T> {
    pub fn status(&self) -> impl std::ops::Deref<Target = TaskStatus<T>> + use<'_, T> {
        self.state.read()
    }
}

#[derive(Debug, Default)]
pub enum TaskStatus<T: Unpin + Send + Sync + 'static> {
    #[default]
    InProgress,
    Done(T),
    Error(eyre::Error),
}

pub trait UseTask<T: Unpin + Send + Sync + 'static> {
    fn use_task<F>(&mut self, f: F) -> Task<T>
    where
        F: FnOnce() -> eyre::Result<T> + Send + 'static;
}

impl<'a, T: Unpin + Send + Sync + 'static> UseTask<T> for Hooks<'_, '_> {
    fn use_task<F>(&mut self, f: F) -> Task<T>
    where
        F: FnOnce() -> eyre::Result<T> + Send + 'static,
    {
        let mut state = self.use_state(|| TaskStatus::<T>::InProgress);

        self.use_future(async move {
            let result = smol::unblock(move || f()).await;
            state.set(match result {
                Ok(done) => TaskStatus::Done(done),
                Err(error) => TaskStatus::Error(error),
            });
        });

        Task { state }
    }
}
