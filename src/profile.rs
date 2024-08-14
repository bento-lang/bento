pub struct Profile {
    max_stack_depth: Option<usize>,
    max_heap_size: Option<usize>,
    max_time_ms: Option<usize>,
    capabilities: Capabilities
}

pub struct Capabilities {
    io: bool,
    network: bool,
    filesystem: bool,
    async_await: bool,
}

impl Profile {
    pub fn default() -> Self {
        Self {
            max_stack_depth: None,
            max_heap_size: None,
            max_time_ms: None,
            capabilities: Capabilities {
                io: false,
                network: false,
                filesystem: false,
                async_await: false,
            }
        }
    }
}