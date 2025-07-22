use tauri::Manager;
use tokio::time::{sleep, Duration};
use std::time::{Instant};
use mouse_rs::{Mouse};
use display_info::DisplayInfo;
use std::thread;
use windows::{
    Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
};

#[tauri::command]
async fn toggle_popup_window(app: tauri::AppHandle) -> Result<(), String> {
    let seconds = 1.4;
    if let Some(window) = app.get_webview_window("image_popup") {
        window.show().map_err(|e| e.to_string())?;
        sleep(Duration::from_secs_f64(seconds)).await;
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn fade_popup_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("image_popup2") {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        let visible = 500;
        let duration = 1500;
        let steps = 20;

        set_opacity(hwnd, 0);
        window.show().map_err(|e| e.to_string())?;
        fade_window(hwnd, true, duration, steps);
        thread::sleep(std::time::Duration::from_millis(visible));
        fade_window(hwnd, false, duration, steps);
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn grab_mouse_position() -> Result<(i32, i32), String> { 
    let mouse = Mouse::new();

    let pos = mouse.get_position().map_err(|e| e.to_string())?;
    Ok((pos.x, pos.y))
}

#[tauri::command]
fn cursor_move(target_x: i32, target_y: i32, duration_secs: f64) -> Result<(), String> {
    let mouse = Mouse::new();

    let start_pos = mouse.get_position().map_err(|e| e.to_string())?;
    let start_pos_x = start_pos.x as f64;
    let start_pos_y = start_pos.y as f64;

    let end_pos_x = target_x as f64;
    let end_pos_y = target_y as f64;

    let start_time = Instant::now();
    let total_duration = Duration::from_secs_f64(duration_secs);
    let frame_duration = Duration::from_millis(5);

    while start_time.elapsed() < total_duration {
        let t = (start_time.elapsed().as_secs_f64() / total_duration.as_secs_f64()).min(1.0);
        
        let current_x = lerp(start_pos_x, end_pos_x, ease_out_cubic(t));
        let current_y = lerp(start_pos_y, end_pos_y, ease_out_cubic(t));

        mouse.move_to(current_x as i32, current_y as i32).map_err(|e| e.to_string())?;

        thread::sleep(frame_duration);
    }

    mouse.move_to(target_x, target_y).map_err(|e| e.to_string())?; 
    Ok(())
}

#[tauri::command]
fn cursor_corners() -> Result<(), String> {
    let mouse = Mouse::new();

    let display_infos = DisplayInfo::all().map_err(|e| e.to_string())?; 

    //&display_infos borrows the first display info instead of changing it
    let display = &display_infos[0];
    let width = display.width as i32;
    let height = display.height as i32;

    let corners = [
        (0, 0), //0 - Topleft
        (width - 10, 0), //1 - Topright
        (0, height - 20), //2 - Bottomleft
        (width - 10, height - 20), //3 - Bottomright
    ];

    mouse.move_to(corners[0].0, corners[0].1).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(150));
    mouse.move_to(corners[1].0, corners[1].1).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(150));
    mouse.move_to(corners[2].0, corners[2].1).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(150));
    mouse.move_to(corners[3].0, corners[3].1).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(150));

    
    Ok(())
}

#[tauri::command]
async fn terminate_program(_app: tauri::AppHandle) -> Result<(), String> {
    tokio::spawn(async move {
        let mut was_pressed = false;
        
        loop {
            unsafe {
                let ctrl = GetAsyncKeyState(VK_CONTROL.0 as i32) < 0;
                let alt = GetAsyncKeyState(VK_MENU.0 as i32) < 0; // VK_MENU is Alt key
                let p = GetAsyncKeyState(0x50) < 0; // 0x50 is P key
                
                let combo_pressed = ctrl && alt && p;
                
                if combo_pressed && !was_pressed {
                    println!("Terminating program...");
                    std::process::exit(0);
                }
                
                was_pressed = combo_pressed;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });
    
    Ok(())
}


#[tauri::command]
fn close_app() {
    std::process::exit(0);
}

//Helper functions
fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + t * (end - start)
}

fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

fn set_opacity(hwnd: HWND, alpha: u8) {
    unsafe {
        let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        style |= WS_EX_LAYERED.0 as i32;
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);

        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
    }
}

fn fade_window(hwnd: HWND, fade_in: bool, duration_ms: u32, steps: u32) {
    unsafe {
        let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        style |= WS_EX_LAYERED.0 as i32;
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);

        let step_delay = duration_ms / steps;
        let values: Vec<u8> = if fade_in {
            (0..=150).collect()
        } else {
            (0..=150).rev().collect()
        };

        for i in values.iter().step_by((256 / steps) as usize) {
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), *i as u8, LWA_ALPHA);
            thread::sleep(std::time::Duration::from_millis(step_delay as u64));
        }
    }
}


//-----

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            toggle_popup_window, 
            cursor_move, 
            grab_mouse_position,
            close_app,
            cursor_corners,
            fade_popup_window,
            terminate_program,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}