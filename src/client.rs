
use reqwest::blocking::Client;
use serde_json::{Value, json};
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use uuid::Uuid;
use tauri::command;
use std::process::Command;
use reqwest::StatusCode;

#[cfg(target_os = "windows")]
use std::process::Stdio;

// Constants for user_email and user_token
const USER_EMAIL: &str = "fe.encina.c@gmail.com";
const USER_TOKEN: &str = "5NN8CxWDdAJQu4DX1FuQ";
const TMP_FOLDER: &str = "./tmp";
const BASE_URL: &str = "https://sendu.garagelabs.cl/api/v2";

// Mutex to store the last downloaded file path
lazy_static::lazy_static! {
    static ref LAST_DOWNLOADED_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);
}

#[command]
pub fn get_label(order_id: String) -> Result<Value, String> {
    // Append orderID as query param
    let url = format!("{BASE_URL}/find_label?orderID={}", order_id);

    // Create a UUID for the unique directory name
    let uuid = Uuid::new_v4();

    // Create an HTTP client
    let client = Client::new();

    // Send a GET request with headers
    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header("X-User-Email", USER_EMAIL)
        .header("X-User-Token", USER_TOKEN)
        .send()
        .map_err(|err| err.to_string())?;

    if response.status() == StatusCode::NOT_FOUND {
        let error_message = response.text().unwrap_or("No error message found".to_string());
        return Err(format!("Error 404: {}", error_message));
    }

    // Parse the JSON response
    let json: Value = response.json().map_err(|err| err.to_string())?;

    // Extract the label URL from the response
    let label_url= json
        .get("response")
        .and_then(|response| response.get("label"))
        .and_then(|label| label.as_str())
        .ok_or("No label URL found in response")?;

    // Extract order_id from the response, used to create a unique directory for the downloaded file
    let order_id = json
        .get("response")
        .and_then(|response| response.get("work_order_id"))
        .and_then(|order_id| order_id.as_i64())
        .ok_or("No order_id found in response")?;

    // Extract order from the response
    let order = json
        .get("response")
        .and_then(|response| response.get("order"))
        .and_then(|order| order.as_str())
        .ok_or("No order found in response")?;

    // Download the label file from the label URL
    let mut file_response = client.get(label_url).send().map_err(|err| err.to_string())?;

    // Get the original filename from the URL
    let original_filename = Path::new(label_url)
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("unknown_file");

    // Use the order_id and UUID to create a unique directory for the downloaded file
    let unique_dir = format!("{TMP_FOLDER}/{order_id}_{}", uuid);
    create_dir_all(&unique_dir).map_err(|err| err.to_string())?;

    // Define the path to save the file in the unique directory
    let file_path = Path::new(&unique_dir).join(original_filename);

    // Create a file in the unique directory and write the downloaded content to it
    let mut dest = File::create(&file_path).map_err(|err| err.to_string())?;
    copy(&mut file_response, &mut dest).map_err(|err| err.to_string())?;

   // Store the file path in LAST_DOWNLOADED_FILE for later use
   *LAST_DOWNLOADED_FILE.lock().unwrap() = Some(file_path.clone());

    match print_file_to_default_printer(&file_path) {
        Ok(_) => Ok(json!({
            "message": "File downloaded successfully",
            "file_path": file_path.to_str().unwrap_or("Unknown file path"),
            "order": order,
            "work_order_id": order_id
        })),
        Err(err) => Err(format!("Failed to send file to the Print process: {}" ,err)),
    }
}

fn print_file_to_default_printer(file_path: &PathBuf) -> Result<(), String> {
    let file_str = file_path.to_str().ok_or("Invalid file path")?;

    #[cfg(target_os = "windows")]
    {
        let powershell_script = format!(
            r#"Add-Type -AssemblyName System.Drawing;
            $printer = [System.Drawing.Printing.PrinterSettings]::new().PrinterName;
            $process = Start-Process -FilePath "{}" -ArgumentList '/p' -NoNewWindow -WindowStyle Hidden -PassThru;
            $process.WaitForExit();"#,
            file_str
        );

        let mut output = Command::new("powershell")
            .args(&["-Command", &powershell_script])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "Failed to execute PowerShell".to_string())?;

        output.wait().map_err(|_| "Printing process failed in Windows".to_string())?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        let status = Command::new("lp")
            .arg(file_str)
            .status()
            .map_err(|err| err.to_string())?;
        if !status.success() {
            return Err(format!("Failed to print file in Linux OS: {}", status));
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        let status = Command::new("lp")
            .args(file_str)
            .status()
            .map_err(|err| err.to_string())?;

        if !status.success() {
            return Err(format!("Failed to print file in MacOs: {}", status));
        }

        Ok(())
    }
}
