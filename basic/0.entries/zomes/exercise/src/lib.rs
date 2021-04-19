use hdk::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

#[hdk_entry(id = "greeting")]
pub struct Greeting(String);

entry_defs![Greeting::entry_def()];

#[hdk_extern] 
pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    return create_entry(Greeting(input.content));
}