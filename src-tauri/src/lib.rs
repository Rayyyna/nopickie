use tauri::{Manager, Emitter, WebviewUrl, WebviewWindowBuilder};
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::io::{BufReader, BufRead};
use std::thread;
use std::path::PathBuf;

// Python 进程管理器
pub struct PythonDetector {
    process: Option<Child>,
}

impl PythonDetector {
    fn new() -> Self {
        Self { process: None }
    }
    
    fn is_running(&self) -> bool {
        self.process.is_some()
    }
    
    fn stop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
            println!("🛑 Python 进程已停止");
        }
    }
}

// 自动查找系统中可用的 Python
fn find_python() -> Result<PathBuf, String> {
    println!("🔍 正在查找系统 Python...");
    
    // 常见 Python 安装路径（按优先级）
    let candidates = vec![
        "/Library/Frameworks/Python.framework/Versions/3.11/bin/python3",
        "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3",
        "/Library/Frameworks/Python.framework/Versions/3.10/bin/python3",
        "/usr/local/bin/python3",
        "/opt/homebrew/bin/python3",
        "/usr/bin/python3",
    ];
    
    // 检查预定义路径
    for candidate in &candidates {
        let path = PathBuf::from(candidate);
        if path.exists() {
            println!("✅ 找到 Python: {}", candidate);
            return Ok(path);
        }
    }
    
    // 尝试使用 which python3 查找系统 PATH
    println!("📍 尝试从 PATH 查找 python3...");
    if let Ok(output) = Command::new("which").arg("python3").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let path = PathBuf::from(&path_str);
                if path.exists() {
                    println!("✅ 从 PATH 找到 Python: {}", path_str);
                    return Ok(path);
                }
            }
        }
    }
    
    Err("未找到 Python 3。请确保已安装 Python 3.10 或更高版本。".to_string())
}

// 应用状态（用于跟踪主窗口显隐）
pub struct AppState {
    pub main_is_hidden: bool,
    pub debug_is_hidden: bool,
}

impl AppState {
    fn new() -> Self {
        Self { main_is_hidden: false, debug_is_hidden: true }
    }
}

// Tauri 命令：启动检测
#[tauri::command]
async fn start_detection(
    app: tauri::AppHandle,
    detector: tauri::State<'_, Arc<Mutex<PythonDetector>>>
) -> Result<String, String> {
    let mut detector_guard = detector.lock().map_err(|e| e.to_string())?;
    
    // 检查进程状态，如果已退出则清理
    if let Some(ref mut child) = detector_guard.process {
        if let Ok(Some(status)) = child.try_wait() {
            // 进程已退出
            detector_guard.process = None;
            println!("🧹 清理已退出的 Python 进程 (退出码: {:?})", status.code());
        }
    }
    
    if detector_guard.is_running() {
        return Ok("检测已在运行".to_string());
    }
    
    println!("🚀 启动 Python 检测器...");
    
    // 获取 Python 脚本路径（兼容开发和生产模式）
    let resource_dir = app.path()
        .resource_dir()
        .map_err(|e| format!("无法获取资源目录: {}", e))?;
    
    // 尝试多个可能的路径
    println!("🔍 资源目录: {:?}", resource_dir);
    
    let python_dir = resource_dir.join("python");
    let python_dir_up = Some(resource_dir.join("_up_").join("python"));
    
    println!("🔍 尝试路径 1: {:?}", python_dir.join("main_simple.py"));
    println!("🔍 路径 1 存在: {}", python_dir.join("main_simple.py").exists());
    
    if let Some(ref dir_up) = python_dir_up {
        println!("🔍 尝试路径 2: {:?}", dir_up.join("main_simple.py"));
        println!("🔍 路径 2 存在: {}", dir_up.join("main_simple.py").exists());
    }
    
    let (final_python_dir, script_path) = if python_dir.join("main_simple.py").exists() {
        let path = python_dir.join("main_simple.py");
        println!("✅ 使用路径 1");
        (python_dir.clone(), path)
    } else if let Some(dir_up) = python_dir_up {
        if dir_up.join("main_simple.py").exists() {
            let path = dir_up.join("main_simple.py");
            println!("✅ 使用路径 2");
            (dir_up, path)
        } else {
            let err = format!("找不到 Python 脚本，尝试的路径: {:?} 和 {:?}", 
                python_dir.join("main_simple.py"), dir_up.join("main_simple.py"));
            println!("❌ {}", err);
            return Err(err);
        }
    } else {
        let err = format!("找不到 Python 脚本: {:?}", python_dir.join("main_simple.py"));
        println!("❌ {}", err);
        return Err(err);
    };
    
    println!("📂 最终 Python 脚本路径: {:?}", script_path);
    println!("📂 工作目录: {:?}", final_python_dir);
    
    // 查找系统中的 Python
    let python_path = find_python()?;
    
    // 启动 Python 进程
    let mut child = Command::new(&python_path)
        .arg(&script_path)
        .current_dir(&final_python_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("无法启动 Python: {}", e))?;
    
    println!("✅ Python 进程已启动");
    
    // 获取 stdout 用于读取 JSON
    let stdout = child.stdout.take().ok_or("无法获取 stdout")?;
    let stderr = child.stderr.take().ok_or("无法获取 stderr")?;
    
    // 保存进程句柄
    detector_guard.process = Some(child);
    drop(detector_guard);
    
    // 启动线程读取 Python 输出 (stdout - JSON 事件)
    let app_clone = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("📨 Python 输出: {}", line);
                
                // 解析 JSON 并发送事件到前端
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                    let event_type = json["event"].as_str().unwrap_or("unknown");
                    
                    // 检测到挠头时显示通知条
                    if event_type == "scratch_detected" {
                        let app_handle = app_clone.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Err(e) = show_alert(app_handle).await {
                                println!("❌ 显示通知条失败: {}", e);
                            }
                        });
                    }
                    
                    // 发送到前端（用于界面显示）
                    let _ = app_clone.emit(event_type, json.clone());
                }
            }
        }
        println!("📭 Python stdout 读取线程结束");
    });
    
    // 启动线程读取 Python 错误输出 (stderr - 日志)
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("⚠️ Python stderr: {}", line);
            }
        }
        println!("📭 Python stderr 读取线程结束");
    });
    
    Ok("检测已启动".to_string())
}

// Tauri 命令：停止检测
#[tauri::command]
async fn stop_detection(
    detector: tauri::State<'_, Arc<Mutex<PythonDetector>>>
) -> Result<String, String> {
    let mut detector_guard = detector.lock().map_err(|e| e.to_string())?;
    detector_guard.stop();
    Ok("检测已停止".to_string())
}

// Tauri 命令：获取检测状态
#[tauri::command]
async fn get_detection_status(
    detector: tauri::State<'_, Arc<Mutex<PythonDetector>>>
) -> Result<bool, String> {
    let detector_guard = detector.lock().map_err(|e| e.to_string())?;
    Ok(detector_guard.is_running())
}

// 系统通知已删除，改用自定义弹窗

// Tauri 命令：切换调试窗口
#[tauri::command]
async fn toggle_debug_window(app: tauri::AppHandle) -> Result<(), String> {
    println!("🧪 切换调试窗口");
    
    if let Some(debug_window) = app.get_webview_window("debug") {
        // 窗口存在，切换显隐
        let is_visible = debug_window.is_visible().unwrap_or(false);
        if is_visible {
            debug_window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
            println!("🙈 调试窗口已隐藏");
        } else {
            debug_window.show().map_err(|e| format!("显示窗口失败: {}", e))?;
            debug_window.set_focus().map_err(|e| format!("聚焦窗口失败: {}", e))?;
            println!("👁️ 调试窗口已显示");
        }
    } else {
        // 窗口不存在，创建
        let _ = WebviewWindowBuilder::new(
            &app,
            "debug",
            WebviewUrl::App("debug.html".into())
        )
        .title("NoPickie - 调试器")
        .resizable(true)
        .inner_size(1024.0, 768.0)
        .visible(true)
        .build()
        .map_err(|e| format!("创建调试窗口失败: {}", e))?;
        
        println!("✅ 调试窗口已创建");
    }
    
    Ok(())
}

// Tauri 命令：打开截图文件夹
#[tauri::command]
async fn open_screenshots_folder() -> Result<(), String> {
    println!("📂 打开截图文件夹");
    
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users".to_string());
    let screenshots_path = format!("{}/Pictures/NoPickie/screenshots", home);
    
    // 确保目录存在
    std::fs::create_dir_all(&screenshots_path)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    println!("📍 截图目录: {}", screenshots_path);
    
    // macOS 使用 open 命令
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("open")
            .arg(&screenshots_path)
            .output()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
        
        if output.status.success() {
            println!("✅ 已打开截图文件夹");
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("打开文件夹失败: {}", error_msg));
        }
    }
    
    Ok(())
}

// Tauri 命令：显示通知条（简化版）
#[tauri::command]
async fn show_alert(app: tauri::AppHandle) -> Result<(), String> {
    println!("🎨 显示提醒通知条");
    
    // 获取或创建通知条窗口
    let window = if let Some(win) = app.get_webview_window("alert") {
        win
    } else {
        // 创建长条形通知窗口（左上角）
        let window = WebviewWindowBuilder::new(
            &app,
            "alert",
            WebviewUrl::App("alert.html".into())
        )
        .title("")
        .decorations(false)
        .always_on_top(true)
        .resizable(false)
        .skip_taskbar(true)
        .visible(false)
        .inner_size(360.0, 60.0)   // 长条形（加宽以容纳文案）
        .position(50.0, 50.0)       // 左上角
        .build()
        .map_err(|e| format!("创建通知条失败: {}", e))?;
        
        println!("✅ 通知条窗口已创建");
        window
    };
    
    // 总是发送抖动事件（不管窗口是否已显示）
    window.emit("shake-alert", ()).map_err(|e| format!("发送抖动事件失败: {}", e))?;
    println!("📳 已发送抖动事件");
    
    // 检查窗口是否已经可见
    let is_visible = window.is_visible().unwrap_or(false);
    
    if is_visible {
        // 窗口已经显示，只抖动不重复显示
        println!("ℹ️ 通知条已在显示中，触发抖动");
    } else {
        // 窗口未显示，显示它
        window.show().map_err(|e| format!("显示窗口失败: {}", e))?;
        println!("✅ 通知条已显示并抖动");
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("========================================");
    println!("🧠 NoPickie v1.0 (Tauri) 启动...");
    println!("========================================");
    
    // 创建 Python 检测器状态
    let detector = Arc::new(Mutex::new(PythonDetector::new()));
    // 创建应用状态
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let app_state_for_setup = app_state.clone();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(detector.clone())
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            start_detection,
            stop_detection,
            get_detection_status,
            show_alert,
            toggle_debug_window,
            open_screenshots_folder
        ])
        .setup(move |app| {
            // 获取主窗口并设置关闭行为（隐藏而不是销毁）
            if let Some(main_window) = app.get_webview_window("main") {
                let window_clone = main_window.clone();
                let state_handle = app_state_for_setup.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止窗口关闭，改为隐藏
                        api.prevent_close();
                        window_clone.hide().ok();
                        if let Ok(mut s) = state_handle.lock() {
                            s.main_is_hidden = true;
                        }
                        println!("🙈 主窗口已隐藏（点击 Dock 图标可重新显示）");
                    }
                });
                println!("✅ 主窗口关闭行为已配置：隐藏而不是销毁");
            }
            
            // 创建托盘菜单
            let toggle_main = tauri::menu::MenuItemBuilder::with_id("toggle_main", "显示/隐藏主窗口").build(app)?;
            let toggle_debug = tauri::menu::MenuItemBuilder::with_id("toggle_debug", "显示/隐藏调试器").build(app)?;
            let start_stop = tauri::menu::MenuItemBuilder::with_id("start_stop", "启动/停止检测").build(app)?;
            let status = tauri::menu::MenuItemBuilder::with_id("status", "查看检测状态").build(app)?;
            let view_screenshots = tauri::menu::MenuItemBuilder::with_id("view_screenshots", "📂 查看截图文件夹").build(app)?;
            let quit = tauri::menu::MenuItemBuilder::with_id("quit", "退出程序").build(app)?;
            
            let menu = tauri::menu::MenuBuilder::new(app)
                .item(&toggle_main)
                .item(&toggle_debug)
                .item(&start_stop)
                .item(&status)
                .item(&view_screenshots)
                .separator()
                .item(&quit)
                .build()?;
            
            // 创建托盘图标
            let detector_clone = detector.clone();
            let state_for_menu = app_state_for_setup.clone();
            
            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon_as_template(true)
                .title("🖐️")
                .tooltip("NoPickie - 挠头检测器")
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "toggle_main" => {
                            println!("👁️ 点击了：显示/隐藏主窗口");
                            if let Some(main_window) = app.get_webview_window("main") {
                                if let Ok(mut s) = state_for_menu.lock() {
                                    if s.main_is_hidden {
                                        let _ = main_window.show();
                                        let _ = main_window.unminimize();
                                        let _ = main_window.set_focus();
                                        s.main_is_hidden = false;
                                        println!("👁️ 主窗口已显示（通过菜单）");
                                    } else {
                                        let _ = main_window.hide();
                                        s.main_is_hidden = true;
                                        println!("🙈 主窗口已隐藏（通过菜单）");
                                    }
                                }
                            }
                        }
                        "toggle_debug" => {
                            println!("🧪 点击了：显示/隐藏调试器");
                            // 懒创建/切换调试窗口
                            if let Ok(mut s) = state_for_menu.lock() {
                                if let Some(debug_window) = app.get_webview_window("debug") {
                                    if s.debug_is_hidden {
                                        let _ = debug_window.show();
                                        let _ = debug_window.unminimize();
                                        let _ = debug_window.set_focus();
                                        s.debug_is_hidden = false;
                                        println!("🧪 调试窗口已显示");
                                    } else {
                                        let _ = debug_window.hide();
                                        s.debug_is_hidden = true;
                                        println!("🧪 调试窗口已隐藏");
                                    }
                                } else {
                                    // 创建调试窗口
                                    let _ = tauri::webview::WebviewWindowBuilder::new(
                                        app,
                                        "debug",
                                        tauri::WebviewUrl::App("debug.html".into())
                                    )
                                    .title("NoPickie - 调试器")
                                    .resizable(true)
                                    .inner_size(1024.0, 768.0)
                                    .visible(true)
                                    .build();
                                    s.debug_is_hidden = false;
                                    println!("🧪 调试窗口已创建并显示");
                                }
                            }
                        }
                        "start_stop" => {
                            println!("▶️ 点击了：启动/停止检测");
                            
                            let is_running = {
                                let guard = detector_clone.lock().unwrap();
                                guard.is_running()
                            };
                            
                            let app_handle = app.clone();
                            let detector = detector_clone.clone();
                            
                            // 在后台线程中异步处理
                            std::thread::spawn(move || {
                                if is_running {
                                    // 停止检测
                                    let mut guard = detector.lock().unwrap();
                                    guard.stop();
                                    println!("✅ 检测已停止（通过菜单栏）");
                                    
                                    // 通知前端更新 UI
                                    let _ = app_handle.emit("detection_stopped_from_menu", ());
                                } else {
                                    // 启动检测
                                    println!("🚀 启动检测（通过菜单栏）...");
                                    
                                    // 调用启动逻辑
                                    let result = tauri::async_runtime::block_on(async {
                                        // 获取资源目录
                                        let resource_dir = app_handle.path().resource_dir()
                                            .map_err(|e| format!("无法获取资源目录: {}", e))?;
                                        
                                        // 尝试多个可能的路径
                                        let python_dir = resource_dir.join("python");
                                        let python_dir_up = Some(resource_dir.join("_up_").join("python"));
                                        
                                        let (final_python_dir, script_path) = if python_dir.join("main_simple.py").exists() {
                                            let path = python_dir.join("main_simple.py");
                                            (python_dir.clone(), path)
                                        } else if let Some(dir_up) = python_dir_up {
                                            if dir_up.join("main_simple.py").exists() {
                                                let path = dir_up.join("main_simple.py");
                                                (dir_up, path)
                                            } else {
                                                return Err(format!("找不到 Python 脚本"));
                                            }
                                        } else {
                                            return Err(format!("找不到 Python 脚本"));
                                        };
                                        
                                        println!("📂 Python 脚本路径: {:?}", script_path);
                                        
                                        // 查找系统中的 Python
                                        let python_path = find_python()?;
                                        
                                        // 启动 Python 进程
                                        let mut child = Command::new(&python_path)
                                            .arg(&script_path)
                                            .current_dir(&final_python_dir)
                                            .stdout(Stdio::piped())
                                            .stderr(Stdio::piped())
                                            .spawn()
                                            .map_err(|e| format!("无法启动 Python: {}", e))?;
                                        
                                        println!("✅ Python 进程已启动（通过菜单栏）");
                                        
                                        // 获取 stdout 和 stderr
                                        let stdout = child.stdout.take().ok_or("无法获取 stdout")?;
                                        let stderr = child.stderr.take().ok_or("无法获取 stderr")?;
                                        
                                        // 保存进程句柄
                                        {
                                            let mut guard = detector.lock().unwrap();
                                            guard.process = Some(child);
                                        }
                                        
                                        // 启动读取线程
                                        let app_clone = app_handle.clone();
                                        std::thread::spawn(move || {
                                            let reader = BufReader::new(stdout);
                                            for line in reader.lines() {
                                                if let Ok(line) = line {
                                                    println!("📨 Python 输出: {}", line);
                                                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                                                        let event_type = json["event"].as_str().unwrap_or("unknown");
                                                        
                                                        // 检测到挠头时显示通知条
                                                        if event_type == "scratch_detected" {
                                                            let app_handle = app_clone.clone();
                                                            tauri::async_runtime::spawn(async move {
                                                                if let Err(e) = show_alert(app_handle).await {
                                                                    println!("❌ 显示通知条失败: {}", e);
                                                                }
                                                            });
                                                        }
                                                        
                                                        let _ = app_clone.emit(event_type, json.clone());
                                                    }
                                                }
                                            }
                                        });
                                        
                                        std::thread::spawn(move || {
                                            let reader = BufReader::new(stderr);
                                            for line in reader.lines() {
                                                if let Ok(line) = line {
                                                    println!("⚠️ Python stderr: {}", line);
                                                }
                                            }
                                        });
                                        
                                        Ok::<(), String>(())
                                    });
                                    
                                    if let Err(e) = result {
                                        println!("❌ 启动失败: {}", e);
                                    }
                                    
                                    // 通知前端更新 UI
                                    let _ = app_handle.emit("detection_started_from_menu", ());
                                }
                            });
                        }
                        "status" => {
                            println!("📊 点击了：查看检测状态");
                            
                            let is_running = {
                                let guard = detector_clone.lock().unwrap();
                                guard.is_running()
                            };
                            
                            let status_msg = if is_running {
                                "✅ 检测器正在运行\\n\\n挠头时会自动弹窗提醒并保存截图。"
                            } else {
                                "⏸️ 检测器未运行\\n\\n点击菜单栏的「启动/停止检测」来启动。"
                            };
                            
                            // 使用 osascript 显示对话框 (macOS)
                            #[cfg(target_os = "macos")]
                            {
                                use std::process::Command;
                                let _ = Command::new("osascript")
                                    .arg("-e")
                                    .arg(format!("display dialog \"{}\" with title \"NoPickie - 检测状态\" buttons {{\"好\"}} default button \"好\"", status_msg))
                                    .spawn();
                            }
                        }
                        "view_screenshots" => {
                            println!("📂 点击了：查看截图文件夹");
                            
                            // 获取用户主目录
                            let home = std::env::var("HOME").unwrap_or_else(|_| "/Users".to_string());
                            let screenshots_path = format!("{}/Pictures/NoPickie/screenshots", home);
                            
                            // 确保目录存在
                            let _ = std::fs::create_dir_all(&screenshots_path);
                            
                            println!("📍 截图目录: {}", screenshots_path);
                            
                            // 打开 Finder
                            #[cfg(target_os = "macos")]
                            {
                                use std::process::Command;
                                let result = Command::new("open")
                                    .arg(&screenshots_path)
                                    .spawn();
                                
                                match result {
                                    Ok(_) => println!("✅ 已打开截图文件夹"),
                                    Err(e) => println!("❌ 打开文件夹失败: {}", e)
                                }
                            }
                        }
                        "quit" => {
                            println!("👋 退出程序...");
                            
                            // 停止 Python 进程
                            let mut guard = detector_clone.lock().unwrap();
                            guard.stop();
                            
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            println!("✅ 菜单栏图标已创建");
            
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run({
            let app_state_for_run = app_state.clone();
            move |app_handle, event| {
                match event {
                    tauri::RunEvent::Reopen { .. } => {
                        if let Some(main_window) = app_handle.get_webview_window("main") {
                            let _ = main_window.show();
                            let _ = main_window.unminimize();
                            let _ = main_window.set_focus();
                            if let Ok(mut s) = app_state_for_run.lock() {
                                s.main_is_hidden = false;
                            }
                            println!("🪟 Dock Reopen：主窗口已显示");
                        }
                    }
                    _ => {}
                }
            }
        });
}
