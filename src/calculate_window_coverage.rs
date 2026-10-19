use hyprland::data::{Client, FullscreenMode, Monitor};

pub fn calculate_window_coverage(monitor: &Monitor, clients: &[Client]) -> u8 {
    let scale = if monitor.scale > 0.0 {
        monitor.scale
    } else {
        1.0
    };

    let monitor_x = monitor.x;
    let monitor_y = monitor.y;
    let monitor_width = (monitor.width as f32 / scale).round() as i32;
    let monitor_height = (monitor.height as f32 / scale).round() as i32;
    let monitor_area = (monitor_width as i64) * (monitor_height as i64);

    let active_workspace = monitor.active_workspace.id;
    let mut total_intersection_area: i64 = 0;

    for client in clients {
        if client.workspace.id != active_workspace {
            continue;
        }

        let (FullscreenMode::None, FullscreenMode::None) =
            (client.fullscreen, client.fullscreen_client)
        else {
            return 100;
        };

        if client.floating {
            continue;
        }

        let window_x = client.at.0 as i32;
        let window_y = client.at.1 as i32;
        let window_width = (client.size.0 as i32).max(0);
        let window_height = (client.size.1 as i32).max(0);

        let intersection_x_start = window_x.max(monitor_x);
        let intersection_y_start = window_y.max(monitor_y);
        let intersection_x_end = (window_x + window_width).min(monitor_x + monitor_width);
        let intersection_y_end = (window_y + window_height).min(monitor_y + monitor_height);

        let intersection_width = (intersection_x_end - intersection_x_start).max(0);
        let intersection_height = (intersection_y_end - intersection_y_start).max(0);

        total_intersection_area += (intersection_width as i64) * (intersection_height as i64);
    }

    if monitor_area > 0 {
        ((total_intersection_area * 100) / monitor_area).min(100) as u8
    } else {
        0
    }
}
