use super::bounds::check_bounds;

pub fn check_cpu_instructions(instructions: u32, limit: u64) -> Result<(), String> {
    check_bounds(u64::from(instructions), limit, "CPU Instructions")
}

pub fn check_read_bytes(bytes: u32, limit: u64) -> Result<(), String> {
    check_bounds(u64::from(bytes), limit, "Read Bytes")
}

pub fn check_write_bytes(bytes: u32, limit: u64) -> Result<(), String> {
    check_bounds(u64::from(bytes), limit, "Write Bytes")
}
