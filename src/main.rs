use rfd::FileDialog;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;
use sysinfo::System;

fn toggle_dll(old_name: &Path, new_name: &OsString) {
    match fs::rename(old_name, new_name) {
        Ok(_) => println!("\nToggling DLL..."),
        Err(e) => println!("\nError renaming file: {}", e),
    };
}

fn main() {
    //  C:\Program Files (x86)\Mirillis\Action!\vulkan_x64\MirillisActionVulkanLayer.dll
    println!("\n.Input your problematic DLL file");
    let Some(problematic_dll) = FileDialog::new().pick_file() else {
        eprintln!("No file selected.");
        std::process::exit(1);
    };
    println!("=> Problematic DLL: {}", problematic_dll.to_str().unwrap());

    let old_name = Path::new(problematic_dll.as_os_str());

    if let Some(extension) = old_name.extension() {
        if extension == "dll" || extension == "DLL" {
        } else if extension == "bak" {
            let formatted = old_name.file_stem().unwrap();

            let binding = OsString::from(old_name.with_file_name(formatted));

            let new_name = &binding;
            toggle_dll(old_name, new_name);
            std::process::exit(0);
        } else {
            eprintln!("Please, provide a valid DLL file!");
            std::process::exit(1);
        }
    }

    //  C:\zed.exe
    println!("\n.Input your EXEcutable file");
    let Some(executable) = FileDialog::new().pick_file() else {
        eprintln!("No file selected.");
        std::process::exit(1);
    };
    println!("=> Executable: {}", executable.to_str().unwrap());

    let check_exe = Path::new(executable.as_os_str());
    if let Some(extension) = check_exe.extension() {
        if extension != "exe" && extension != "EXE" {
            eprintln!("Please, provide a valid EXE file!");
            std::process::exit(1);
        }
    }
    let formatted = format!("{}.bak", problematic_dll.display());
    let formatted = Path::new(&formatted);

    let binding = OsString::from(old_name.with_file_name(formatted));
    let new_name = &binding;

    toggle_dll(old_name, new_name);

    let formatted = format!("{}.exe", executable.file_stem().unwrap().to_str().unwrap());
    let target_process: &str = &formatted;

    println!("\n>>> LAUNCHING {} :)", executable.to_str().unwrap());

    Command::new(&executable)
        .output()
        .expect("Failed to execute command");

    let mut is_running = true;

    while is_running {
        let mut system = System::new_all();
        system.refresh_all();

        is_running = system
            .processes()
            .values()
            .any(|process| process.name().eq_ignore_ascii_case(target_process));
    }

    let (old_name, new_name) = (new_name, old_name);
    let old_name: &Path = &Path::new(old_name);

    let new_name: &OsString = &OsString::from(new_name);

    println!("\n>>> {} CLOSED :(", executable.to_str().unwrap());

    toggle_dll(old_name, new_name);
}
