use crate::export::{export_to_csv, export_to_xlsx};
use crate::models::{ExportFormat, ExportRequest};

#[tauri::command]
pub async fn export_results(request: ExportRequest) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || match request.format {
        ExportFormat::Csv => export_to_csv(&request.output_path, &request.items),
        ExportFormat::Xlsx => export_to_xlsx(&request.output_path, &request.items),
    })
    .await
    .map_err(|e| format!("エクスポート処理実行エラー: {}", e))?
}
