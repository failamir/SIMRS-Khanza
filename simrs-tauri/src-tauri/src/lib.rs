mod database;
mod handlers;
mod models;
mod services;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database connection pool
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async {
                if let Err(e) = database::init_pool(app_handle).await {
                    eprintln!("Failed to initialize database pool: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Dashboard
            handlers::dashboard::get_dashboard_stats,
            handlers::dashboard::get_recent_patients,
            // Pasien
            handlers::pasien::get_pasien_list,
            handlers::pasien::get_pasien_by_id,
            handlers::pasien::create_pasien,
            handlers::pasien::update_pasien,
            handlers::pasien::delete_pasien,
            handlers::pasien::search_pasien,
            // Registrasi
            handlers::registrasi::get_registrasi_list,
            handlers::registrasi::create_registrasi,
            handlers::registrasi::get_antrian,
            // Settings
            handlers::settings::get_settings,
            handlers::settings::update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}