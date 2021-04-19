use hdk::prelude::*;
use holo_hash::EntryHashB64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    title: String,
    content: String,
}

#[hdk_entry(id = "book")]
pub struct Book {
    pub title: String,
    pub content: String,
}

entry_defs![Book::entry_def()];

#[hdk_extern] 
pub fn add_book(input: SomeExternalInput) -> ExternResult<EntryHashB64> {    
    let book : Book = Book { 
        title: input.title, 
        content: input.content 
    };
    let _header_hash : HeaderHash = create_entry(&book)?;
    let entry_hash: EntryHash = hash_entry(&book)?;
    Ok(EntryHashB64::from(entry_hash))    
}

#[hdk_extern] 
pub fn get_book(hash: EntryHashB64) -> ExternResult<Book> {    
    let element: Element = get(EntryHash::from(hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Couldn't find book")))?;
    let book_option: Option<Book> = element.entry().to_app_option()?;
    let book: Book = book_option.unwrap();
    Ok(book)

}
