use crate::models::CellPreviewData;
use crate::search::preview::extract_cell_preview;

#[tauri::command]
pub async fn get_cell_preview(
    file_path: String,
    sheet_name: String,
    row_index: u32,
    col_index: u32,
) -> Result<CellPreviewData, String> {
    tauri::async_runtime::spawn_blocking(move || {
        extract_cell_preview(&file_path, &sheet_name, row_index, col_index)
    })
    .await
    .map_err(|e| format!("タスク実行エラー: {}", e))?
}
