use hdk::prelude::*;
use holo_hash::HeaderHashB64;
use holo_hash::EntryHashB64;

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderAndEntryHash{
    entry_hash: EntryHashB64,
    header_hash: HeaderHashB64
}


#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderAndEntryHash> {
    let header_hash = create_entry(&input)?;
    let entry_hash = hash_entry(input)?;
    let ret : HeaderAndEntryHash = HeaderAndEntryHash { entry_hash: entry_hash.into(), header_hash: header_hash.into() };
    Ok(ret)
}

#[hdk_extern]
pub fn get_by_header_hash(hash: HeaderHashB64) -> ExternResult<SnackingLog> {
    let element: Element = get(HeaderHash::from(hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Couldn't find snackage")))?;
    let snack_option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack: SnackingLog = snack_option.unwrap();
    Ok(snack)
}

#[hdk_extern]
pub fn get_by_entry_hash(hash: EntryHashB64) -> ExternResult<SnackingLog> {
    let element: Element = get(EntryHash::from(hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Couldn't find snackage")))?;
    let snack_option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack: SnackingLog = snack_option.unwrap();
    Ok(snack)
}

#[hdk_extern]
pub fn get_header_hash_by_content(input: SnackingLog) -> ExternResult<HeaderHashB64> {
    let entry_hash: EntryHash = hash_entry(&input)?;
    let element: Element = get(entry_hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Couldn't find snackage")))?;
    let header_hash: HeaderHash = element.header_address().clone();
    Ok(HeaderHashB64::from(header_hash))
}
