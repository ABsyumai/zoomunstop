#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tauri::Manager;
use std::thread;
use std::time::{self, Duration};
use std::collections::HashMap;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use sysinfo::{System, SystemExt, ProcessExt, ProcessRefreshKind, Pid};

#[derive(Debug, Default)]
struct State {
  pub confno: String,
  pub pwd: String,
  pub zc: String,
  pub uname: String,
  pub interval: Duration,
  pub pids: HashMap<Pid,u64>,
  pub sys: System
}

impl State {
  pub fn uri(&self) -> Option<String> {
    if self.pwd.is_empty() || self.confno.is_empty() { return None }
    let e = utf8_percent_encode;
    let mut s = format!(
      "zoommtg://zoom.us/join?confno={}&pwd={}",
      e(&self.confno, NON_ALPHANUMERIC),
      e(&self.pwd, NON_ALPHANUMERIC),
    );
    if !self.zc.is_empty() {
      s.push_str("&zc=");
      s.push_str(&e(&self.zc, NON_ALPHANUMERIC).to_string());
    }
    if !self.uname.is_empty() {
      s.push_str("&uname=");
      s.push_str(&e(&self.uname, NON_ALPHANUMERIC).to_string());
    }
    Some(s)
  }
  pub fn refresh_pids(&mut self) {
    let Self{
      sys,
      pids,
      ..
    } = self;
    sys.refresh_processes_specifics(ProcessRefreshKind::everything());
    for i in sys.processes_by_name("Zoom") {
      pids.insert(i.pid(), i.run_time());
    }
  }

  pub fn check_pids(&mut self) -> bool {
    let Self{
      sys,
      pids,
      ..
    } = self;
    sys.refresh_processes_specifics(ProcessRefreshKind::everything());
    for i in sys.processes_by_name("Zoom") {
      match pids.get(&i.pid()) {
        Some(&b) if b <= i.run_time() => (),
        _ => return true,
      }
    }
    false
  }
}

static STATE: OnceCell<Mutex<State>> = OnceCell::new();

#[tauri::command]
fn set_confno(s: String){
  dbg!(&s);
  STATE.get().unwrap().lock().unwrap().confno = s;
}

#[tauri::command]
fn set_pwd(s: String){
  dbg!(&s);
  STATE.get().unwrap().lock().unwrap().pwd = s;
}

#[tauri::command]
fn set_zc(s: String){
  dbg!(&s);
  STATE.get().unwrap().lock().unwrap().zc = s;
}

#[tauri::command]
fn set_uname(s: String){
  dbg!(&s);
  STATE.get().unwrap().lock().unwrap().uname = s;
}

#[tauri::command]
fn set_interval(i: u64) {
  let d = time::Duration::from_millis(i);
  STATE.get().unwrap().lock().unwrap().interval = d;
}

#[tauri::command]
async fn reload_zoom() {
  STATE.get().unwrap().lock().unwrap().pids.clear();
}

#[cfg(target_os = "windows")]
async fn launch_uri(uri: String) -> bool {
  let uri = windows::core::HSTRING::from(uri);
  let uri = windows::Foundation::Uri::CreateUri(uri).unwrap();
  windows::System::Launcher::LaunchUriAsync(uri).unwrap().await.is_ok()
}
#[cfg(target_os = "macos")]
async fn launch_uri(uri: String) -> bool {
  tokio::process::Command::new("open")
    .arg(&uri)
    .spawn()
    .expect("Failed to launch \"open\"")
    .wait()
    .await
    .is_ok()
}
#[cfg(target_os = "linux")]
async fn launch_uri(uri: String) -> bool {
  tokio::process::Command::new("xdg-open")
    .arg(&uri)
    .spawn()
    .expect("Failed to launch \"xdg-open\"")
    .wait()
    .await
    .is_ok()
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
async fn launch_uri(uri: String) -> bool {
  compile_error!()
}
fn main() {
  let _ = STATE.set(Mutex::new(State{
    interval: Duration::from_secs(5),
    ..Default::default()
  }));
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      set_confno,
      set_pwd,
      set_zc,
      set_uname,
      set_interval,
      reload_zoom
    ])
    .setup(|app|{
      let h = app.app_handle();
      thread::spawn(move ||{
        STATE.get().unwrap().lock().unwrap().refresh_pids();
        loop {
          let refresh_zoom = STATE.get().unwrap().lock().unwrap().check_pids();
          if refresh_zoom {
            let d;
            let uri;
            {
              let mut lock = STATE.get().unwrap().lock().unwrap();
              lock.pids.clear();
              d = lock.interval;
              uri = lock.uri();
            }
            if let Some(uri) = uri {
              let succeed = tauri::async_runtime::block_on(launch_uri(uri));
              thread::sleep(d);
              h.emit_all("reload", succeed).unwrap();
            }
          }
          STATE.get().unwrap().lock().unwrap().refresh_pids();
          thread::sleep(time::Duration::from_secs(1));
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
