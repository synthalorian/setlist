pub mod chordpro;
pub mod transpose;

/// Tauri command: transpose a ChordPro chart. Wired for the future UI.
#[tauri::command]
fn transpose_text(text: &str, semitones: i32) -> String {
    chordpro::transpose_chart(text, semitones)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![transpose_text])
        .run(tauri::generate_context!())
        .expect("error while running setlist");
}
