use std::marker::PhantomData;

struct OpenState;
struct ClosedState;

struct MemoryFile<State> {
    name: String,
    phantom: PhantomData<State>,
}

impl MemoryFile<OpenState> {
    pub fn close(&self) -> MemoryFile<ClosedState> {
        MemoryFile {
            name: self.name.clone(),
            phantom: PhantomData,
        }
    }

    pub fn read(&self) -> String {
        self.name.clone()
    }
}

impl MemoryFile<ClosedState> {
    pub fn new(name: &str) -> MemoryFile<ClosedState> {
        MemoryFile {
            name: name.to_string(),
            phantom: PhantomData,
        }
    }

    pub fn open(&self) -> MemoryFile<OpenState> {
        MemoryFile {
            name: self.name.clone(),
            phantom: PhantomData,
        }
    }
}
