use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Entry, Label, ListBox, ListBoxRow, Popover, ScrolledWindow, Separator, TextView, TextBuffer, Image, CheckButton};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConnectionHistory {
    addresses: Vec<String>,
    last_connection: Option<ConnectionInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConnectionInfo {
    server_address: String,
    ssh_port: u16,
    username: String,
    password: String,
    local_port: u16,
    use_key_auth: bool,
    key_path: String,
}

impl Default for ConnectionHistory {
    fn default() -> Self {
        Self {
            addresses: Vec::new(),
            last_connection: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    Chinese,
    English,
}

struct Translations {
    title: &'static str,
    header: &'static str,
    server_address: &'static str,
    ssh_port: &'static str,
    username: &'static str,
    use_key_auth: &'static str,
    key_path: &'static str,
    password: &'static str,
    local_port: &'static str,
    connect: &'static str,
    disconnect: &'static str,
    connection_status: &'static str,
    security_tips: &'static str,
    security_tips_content: &'static str,
    error_ssh_port_must_be_number: &'static str,
    error_local_port_must_be_number: &'static str,
    error_server_address_empty: &'static str,
    error_username_empty: &'static str,
    error_key_path_empty: &'static str,
    error_password_empty: &'static str,
    connecting: &'static str,
    connected: &'static str,
    connection_failed: &'static str,
    disconnected: &'static str,
    key_auth: &'static str,
    password_auth: &'static str,
    switch_to_english: &'static str,
    switch_to_chinese: &'static str,
}

impl Translations {
    fn get(lang: Language) -> Self {
        match lang {
            Language::Chinese => Self {
                title: "SSH 端口转发",
                header: "SSH 端口转发",
                server_address: "服务器地址",
                ssh_port: "SSH端口",
                username: "用户名",
                use_key_auth: "使用密钥认证",
                key_path: "密钥路径",
                password: "密码",
                local_port: "本地端口",
                connect: "连接",
                disconnect: "断开",
                connection_status: "连接状态",
                security_tips: "安全提示",
                security_tips_content: "• 推荐使用SSH密钥认证，避免密码泄露\n• 首次连接时请验证服务器指纹\n• 配置文件已设置为仅本人可读(600)",
                error_ssh_port_must_be_number: "错误：SSH端口必须是数字",
                error_local_port_must_be_number: "错误：本地端口必须是数字",
                error_server_address_empty: "错误：服务器地址不能为空",
                error_username_empty: "错误：用户名不能为空",
                error_key_path_empty: "错误：密钥路径不能为空",
                error_password_empty: "错误：密码不能为空",
                connecting: "正在连接到服务器...",
                connected: "服务器连接成功\n服务器：{}:{}\n认证方式：{}\n本地端口：{} 已开启 SOCKS5 代理",
                connection_failed: "连接失败：{}",
                disconnected: "连接已断开",
                key_auth: "密钥认证",
                password_auth: "密码认证",
                switch_to_english: "English",
                switch_to_chinese: "简体中文",
            },
            Language::English => Self {
                title: "SSH Port Forwarding",
                header: "SSH Port Forwarding",
                server_address: "Server Address",
                ssh_port: "SSH Port",
                username: "Username",
                use_key_auth: "Use Key Auth",
                key_path: "Key Path",
                password: "Password",
                local_port: "Local Port",
                connect: "Connect",
                disconnect: "Disconnect",
                connection_status: "Connection Status",
                security_tips: "Security Tips",
                security_tips_content: "• Recommended to use SSH key authentication to avoid password leakage\n• Verify server fingerprint on first connection\n• Config file is set to readable only by owner (600)",
                error_ssh_port_must_be_number: "Error: SSH port must be a number",
                error_local_port_must_be_number: "Error: Local port must be a number",
                error_server_address_empty: "Error: Server address cannot be empty",
                error_username_empty: "Error: Username cannot be empty",
                error_key_path_empty: "Error: Key path cannot be empty",
                error_password_empty: "Error: Password cannot be empty",
                connecting: "Connecting to server...",
                connected: "Server connected successfully\nServer: {}:{}\nAuth Method: {}\nLocal Port: {} SOCKS5 proxy enabled",
                connection_failed: "Connection failed: {}",
                disconnected: "Connection disconnected",
                key_auth: "Key Auth",
                password_auth: "Password Auth",
                switch_to_english: "English",
                switch_to_chinese: "简体中文",
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".ssh_proxy_gtk");
    path.push("config.json");
    path
}

fn load_history() -> ConnectionHistory {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(history) = serde_json::from_str(&content) {
                return history;
            }
        }
    }
    ConnectionHistory::default()
}

fn save_history(history: &ConnectionHistory) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(content) = serde_json::to_string_pretty(history) {
        let _ = fs::write(&path, content);
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.SshProxyGtk")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let current_lang = Arc::new(Mutex::new(Language::Chinese));
    let lang_clone1 = current_lang.clone();
    let lang_clone2 = current_lang.clone();
    let lang_clone3 = current_lang.clone();
    let lang_clone4 = current_lang.clone();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(Translations::get(*lang_clone1.lock().unwrap()).title)
        .default_width(520)
        .default_height(600)
        .resizable(true)
        .build();

    let history = load_history();

    let main_box = Box::new(gtk4::Orientation::Vertical, 0);
    
    let header_box = Box::new(gtk4::Orientation::Horizontal, 12);
    header_box.set_margin_top(16);
    header_box.set_margin_start(16);
    header_box.set_margin_end(16);
    
    let header_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).header));
    header_label.add_css_class("heading");
    header_box.append(&header_label);
    
    let lang_button = Button::new();
    lang_button.set_label(Translations::get(*current_lang.lock().unwrap()).switch_to_english);
    lang_button.set_size_request(80, 0);
    header_box.append(&lang_button);
    
    main_box.append(&header_box);

    let form_box = Box::new(gtk4::Orientation::Vertical, 16);
    form_box.set_margin_start(16);
    form_box.set_margin_end(16);
    form_box.set_margin_top(8);
    
    let server_box = Box::new(gtk4::Orientation::Horizontal, 8);
    server_box.set_hexpand(true);
    
    let server_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).server_address));
    server_label.set_halign(gtk4::Align::End);
    server_label.set_width_chars(12);
    server_box.append(&server_label);
    
    let server_address_entry = Entry::new();
    server_address_entry.set_hexpand(true);
    server_address_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("network-server"));
    server_address_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Secondary, Some("pan-down"));
    server_box.append(&server_address_entry);
    form_box.append(&server_box);

    let ssh_port_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let ssh_port_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).ssh_port));
    ssh_port_label.set_halign(gtk4::Align::End);
    ssh_port_label.set_width_chars(12);
    ssh_port_box.append(&ssh_port_label);
    
    let ssh_port_entry = Entry::new();
    ssh_port_entry.set_max_length(5);
    ssh_port_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("network-port"));
    ssh_port_box.append(&ssh_port_entry);
    form_box.append(&ssh_port_box);

    let username_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let username_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).username));
    username_label.set_halign(gtk4::Align::End);
    username_label.set_width_chars(12);
    username_box.append(&username_label);
    
    let username_entry = Entry::new();
    username_entry.set_hexpand(true);
    username_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("system-users"));
    username_box.append(&username_entry);
    form_box.append(&username_box);

    let auth_method_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let use_key_auth_check = CheckButton::with_label(Translations::get(*current_lang.lock().unwrap()).use_key_auth);
    use_key_auth_check.set_halign(gtk4::Align::End);
    auth_method_box.append(&use_key_auth_check);
    form_box.append(&auth_method_box);

    let key_path_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let key_path_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).key_path));
    key_path_label.set_halign(gtk4::Align::End);
    key_path_label.set_width_chars(12);
    key_path_box.append(&key_path_label);
    
    let key_path_entry = Entry::new();
    key_path_entry.set_hexpand(true);
    key_path_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("key"));
    key_path_entry.set_text("~/.ssh/id_ed25519");
    key_path_box.append(&key_path_entry);
    form_box.append(&key_path_box);

    let password_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let password_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).password));
    password_label.set_halign(gtk4::Align::End);
    password_label.set_width_chars(12);
    password_box.append(&password_label);
    
    let password_entry = Entry::new();
    password_entry.set_hexpand(true);
    password_entry.set_visibility(false);
    password_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("lock"));
    password_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Secondary, Some("eye"));
    password_box.append(&password_entry);
    form_box.append(&password_box);

    let local_port_box = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let local_port_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).local_port));
    local_port_label.set_halign(gtk4::Align::End);
    local_port_label.set_width_chars(12);
    local_port_box.append(&local_port_label);
    
    let local_port_entry = Entry::new();
    local_port_entry.set_max_length(5);
    local_port_entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("globe"));
    local_port_box.append(&local_port_entry);
    form_box.append(&local_port_box);

    main_box.append(&form_box);

    let button_box = Box::new(gtk4::Orientation::Horizontal, 8);
    button_box.set_margin_start(16);
    button_box.set_margin_end(16);
    button_box.set_margin_top(8);
    button_box.set_halign(gtk4::Align::Center);
    
    let connect_button = Button::with_label(Translations::get(*current_lang.lock().unwrap()).connect);
    connect_button.add_css_class("suggested-action");
    connect_button.set_size_request(120, 0);
    
    let disconnect_button = Button::with_label(Translations::get(*current_lang.lock().unwrap()).disconnect);
    disconnect_button.add_css_class("destructive-action");
    disconnect_button.set_size_request(120, 0);
    disconnect_button.set_sensitive(false);
    
    button_box.append(&connect_button);
    button_box.append(&disconnect_button);
    main_box.append(&button_box);

    let separator = Separator::new(gtk4::Orientation::Horizontal);
    separator.set_margin_start(16);
    separator.set_margin_end(16);
    separator.set_margin_top(16);
    main_box.append(&separator);

    let status_box = Box::new(gtk4::Orientation::Vertical, 8);
    status_box.set_margin_start(16);
    status_box.set_margin_end(16);
    status_box.set_margin_bottom(16);
    
    let status_header = Box::new(gtk4::Orientation::Horizontal, 8);
    
    let status_icon = Image::from_icon_name("network-wired");
    status_header.append(&status_icon);
    
    let status_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).connection_status));
    status_label.add_css_class("caption");
    status_header.append(&status_label);
    status_box.append(&status_header);

    let status_text_buffer = TextBuffer::new(None);
    let status_text_view = TextView::new();
    status_text_view.set_buffer(Some(&status_text_buffer));
    status_text_view.set_editable(false);
    status_text_view.set_cursor_visible(false);
    status_text_view.set_wrap_mode(gtk4::WrapMode::Word);
    status_text_view.set_hexpand(true);
    status_text_view.set_vexpand(true);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&status_text_view));
    scrolled_window.set_min_content_height(100);
    scrolled_window.set_max_content_height(150);
    scrolled_window.set_hexpand(true);
    status_box.append(&scrolled_window);
    main_box.append(&status_box);

    let security_tips_box = Box::new(gtk4::Orientation::Vertical, 4);
    security_tips_box.set_margin_start(16);
    security_tips_box.set_margin_end(16);
    security_tips_box.set_margin_bottom(16);
    
    let security_tips_label = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).security_tips));
    security_tips_label.add_css_class("caption");
    security_tips_box.append(&security_tips_label);
    
    let security_tips_content = Label::new(Some(Translations::get(*current_lang.lock().unwrap()).security_tips_content));
    security_tips_content.set_wrap(true);
    security_tips_content.add_css_class("small");
    security_tips_box.append(&security_tips_content);
    main_box.append(&security_tips_box);

    window.set_child(Some(&main_box));

    if let Some(last) = &history.last_connection {
        server_address_entry.set_text(&last.server_address);
        ssh_port_entry.set_text(&last.ssh_port.to_string());
        username_entry.set_text(&last.username);
        password_entry.set_text(&last.password);
        local_port_entry.set_text(&last.local_port.to_string());
        use_key_auth_check.set_active(last.use_key_auth);
        if !last.key_path.is_empty() {
            key_path_entry.set_text(&last.key_path);
        }
    } else {
        ssh_port_entry.set_text("22");
        local_port_entry.set_text("1088");
    }

    let _use_key_auth_check_clone = use_key_auth_check.clone();
    let password_entry_clone = password_entry.clone();
    let key_path_entry_clone = key_path_entry.clone();
    
    use_key_auth_check.connect_toggled(move |check: &CheckButton| {
        let active = check.is_active();
        password_entry_clone.set_sensitive(!active);
        key_path_entry_clone.set_sensitive(active);
    });
    
    password_entry.set_sensitive(!use_key_auth_check.is_active());
    key_path_entry.set_sensitive(use_key_auth_check.is_active());

    let history_clone = history.clone();
    let server_address_entry_clone = server_address_entry.clone();
    server_address_entry.connect_icon_press(move |_, pos| {
        if pos != gtk4::EntryIconPosition::Secondary {
            return;
        }
        
        let popup = Popover::new();
        let list_box = ListBox::new();
        
        for addr in &history_clone.addresses {
            let row = ListBoxRow::new();
            let label = Label::new(Some(addr));
            label.set_halign(gtk4::Align::Start);
            row.set_child(Some(&label));
            list_box.append(&row);
        }
        
        let popup_clone = popup.clone();
        let entry_clone = server_address_entry_clone.clone();
        list_box.connect_row_activated(move |_, row| {
            if let Some(child) = row.child() {
                if let Some(label) = child.downcast_ref::<Label>() {
                    entry_clone.set_text(&label.text());
                }
            }
            popup_clone.popdown();
        });
        
        popup.set_child(Some(&list_box));
        popup.set_parent(&server_address_entry_clone);
        popup.popup();
    });

    let ssh_process = Arc::new(Mutex::new(None::<std::process::Child>));
    let status_message = Arc::new(Mutex::new(None::<String>));
    let status_message_clone = status_message.clone();
    let connect_button_clone_for_timer = connect_button.clone();
    let disconnect_button_clone_for_timer = disconnect_button.clone();
    let status_text_buffer_clone_for_timer = status_text_buffer.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        if let Some(msg) = status_message.lock().unwrap().take() {
            status_text_buffer_clone_for_timer.set_text(&msg);
            let t = Translations::get(*lang_clone1.lock().unwrap());
            if msg.starts_with(t.connected.split('\n').next().unwrap_or("")) {
                connect_button_clone_for_timer.set_sensitive(false);
                disconnect_button_clone_for_timer.set_sensitive(true);
            } else {
                connect_button_clone_for_timer.set_sensitive(true);
                disconnect_button_clone_for_timer.set_sensitive(false);
            }
        }
        gtk4::glib::ControlFlow::Continue
    });

    let server_address_entry_clone = server_address_entry.clone();
    let ssh_port_entry_clone = ssh_port_entry.clone();
    let username_entry_clone = username_entry.clone();
    let password_entry_clone = password_entry.clone();
    let local_port_entry_clone = local_port_entry.clone();
    let use_key_auth_check_clone2 = use_key_auth_check.clone();
    let key_path_entry_clone = key_path_entry.clone();
    let status_text_buffer_clone = status_text_buffer.clone();
    let ssh_process_clone = ssh_process.clone();
    let history_clone = history;

    connect_button.connect_clicked(move |button| {
        let server_address = server_address_entry_clone.text().to_string();
        let ssh_port_str = ssh_port_entry_clone.text().to_string();
        let username = username_entry_clone.text().to_string();
        let password = password_entry_clone.text().to_string();
        let local_port_str = local_port_entry_clone.text().to_string();
        let use_key_auth = use_key_auth_check_clone2.is_active();
        let key_path = key_path_entry_clone.text().to_string();

        let ssh_port = match ssh_port_str.parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                let t = Translations::get(*lang_clone2.lock().unwrap());
                status_text_buffer_clone.set_text(t.error_ssh_port_must_be_number);
                return;
            }
        };

        let local_port = match local_port_str.parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                let t = Translations::get(*lang_clone2.lock().unwrap());
                status_text_buffer_clone.set_text(t.error_local_port_must_be_number);
                return;
            }
        };

        if server_address.is_empty() {
            let t = Translations::get(*lang_clone2.lock().unwrap());
            status_text_buffer_clone.set_text(t.error_server_address_empty);
            return;
        }

        if username.is_empty() {
            let t = Translations::get(*lang_clone2.lock().unwrap());
            status_text_buffer_clone.set_text(t.error_username_empty);
            return;
        }

        if use_key_auth && key_path.is_empty() {
            let t = Translations::get(*lang_clone2.lock().unwrap());
            status_text_buffer_clone.set_text(t.error_key_path_empty);
            return;
        }

        if !use_key_auth && password.is_empty() {
            let t = Translations::get(*lang_clone2.lock().unwrap());
            status_text_buffer_clone.set_text(t.error_password_empty);
            return;
        }

        let mut new_history = history_clone.clone();
        if !new_history.addresses.contains(&server_address) {
            new_history.addresses.insert(0, server_address.clone());
            if new_history.addresses.len() > 10 {
                new_history.addresses.pop();
            }
        }
        new_history.last_connection = Some(ConnectionInfo {
            server_address: server_address.clone(),
            ssh_port,
            username: username.clone(),
            password: password.clone(),
            local_port,
            use_key_auth,
            key_path: key_path.clone(),
        });
        save_history(&new_history);

        button.set_sensitive(false);
        let t = Translations::get(*lang_clone2.lock().unwrap());
        status_text_buffer_clone.set_text(t.connecting);

        let ssh_process_clone2 = ssh_process_clone.clone();
        let status_message_clone2 = status_message_clone.clone();
        let lang_clone5 = lang_clone2.clone();

        thread::spawn(move || {
            let result = if use_key_auth {
                let expanded_key_path = if key_path.starts_with("~") {
                    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
                    key_path.replacen("~", home.to_str().unwrap_or("."), 1)
                } else {
                    key_path
                };
                
                Command::new("ssh")
                    .arg("-i")
                    .arg(&expanded_key_path)
                    .arg("-D")
                    .arg(format!("{}", local_port))
                    .arg("-p")
                    .arg(format!("{}", ssh_port))
                    .arg("-N")
                    .arg(format!("{}@{}", username, server_address))
                    .spawn()
            } else {
                Command::new("sshpass")
                    .arg("-p")
                    .arg(&password)
                    .arg("ssh")
                    .arg("-D")
                    .arg(format!("{}", local_port))
                    .arg("-p")
                    .arg(format!("{}", ssh_port))
                    .arg("-N")
                    .arg(format!("{}@{}", username, server_address))
                    .spawn()
            };

            match result {
                Ok(child) => {
                    *ssh_process_clone2.lock().unwrap() = Some(child);
                    let t = Translations::get(*lang_clone5.lock().unwrap());
                    let auth_method = if use_key_auth { t.key_auth } else { t.password_auth };
                    let connected_msg = t.connected
                        .replace("{}", &server_address)
                        .replace("{}", &ssh_port.to_string())
                        .replace("{}", auth_method)
                        .replace("{}", &local_port.to_string());
                    *status_message_clone2.lock().unwrap() = Some(connected_msg);
                }
                Err(e) => {
                    let t = Translations::get(*lang_clone5.lock().unwrap());
                    *status_message_clone2.lock().unwrap() = Some(t.connection_failed.replace("{}", &e.to_string()));
                }
            }
        });
    });

    let ssh_process_clone = ssh_process.clone();
    let status_text_buffer_clone = status_text_buffer.clone();
    let connect_button_clone_for_disconnect = connect_button.clone();

    disconnect_button.connect_clicked(move |button| {
        if let Some(mut child) = ssh_process_clone.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        
        button.set_sensitive(false);
        connect_button_clone_for_disconnect.set_sensitive(true);
        let t = Translations::get(*lang_clone3.lock().unwrap());
        status_text_buffer_clone.set_text(t.disconnected);
    });

    let window_clone = window.clone();
    let header_label_clone = header_label.clone();
    let server_label_clone = server_label.clone();
    let ssh_port_label_clone = ssh_port_label.clone();
    let username_label_clone = username_label.clone();
    let use_key_auth_check_clone3 = use_key_auth_check.clone();
    let key_path_label_clone = key_path_label.clone();
    let password_label_clone = password_label.clone();
    let local_port_label_clone = local_port_label.clone();
    let connect_button_clone_for_lang = connect_button.clone();
    let disconnect_button_clone_for_lang = disconnect_button.clone();
    let status_label_clone = status_label.clone();
    let security_tips_label_clone = security_tips_label.clone();
    let security_tips_content_clone = security_tips_content.clone();

    lang_button.connect_clicked(move |button| {
        let mut lang = lang_clone4.lock().unwrap();
        *lang = match *lang {
            Language::Chinese => Language::English,
            Language::English => Language::Chinese,
        };
        let t = Translations::get(*lang);
        
        window_clone.set_title(Some(t.title));
        header_label_clone.set_text(t.header);
        server_label_clone.set_text(t.server_address);
        ssh_port_label_clone.set_text(t.ssh_port);
        username_label_clone.set_text(t.username);
        use_key_auth_check_clone3.set_label(Some(t.use_key_auth));
        key_path_label_clone.set_text(t.key_path);
        password_label_clone.set_text(t.password);
        local_port_label_clone.set_text(t.local_port);
        connect_button_clone_for_lang.set_label(t.connect);
        disconnect_button_clone_for_lang.set_label(t.disconnect);
        status_label_clone.set_text(t.connection_status);
        security_tips_label_clone.set_text(t.security_tips);
        security_tips_content_clone.set_text(t.security_tips_content);
        
        button.set_label(if *lang == Language::Chinese { t.switch_to_english } else { t.switch_to_chinese });
    });

    window.show();
}