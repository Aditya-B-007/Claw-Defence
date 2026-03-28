// Using the raw identifier `r#trait` because `trait` is a reserved Rust keyword.
pub mod r#trait;

pub mod input {
    pub mod prompt_injection;
    pub mod rate_limit;
}

pub mod output {
    pub mod redaction;
}