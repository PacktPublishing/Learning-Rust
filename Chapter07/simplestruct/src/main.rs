struct Area {
    width: f32,
    length: f32,
}

struct Window {
    window_area: Area,
    window_type: String,
    has_blinds: bool,
    curtain_color: String,
    has_lock: bool,
    top_open: bool,
    single_window: bool,
}

struct Room {
    is_upstairs: bool,
    number_of_doors: i32,
    window: Vec<Window>,
    wood_or_carpet: bool,
    carpet_color: String,
    room_name: String,
    has_wardrobe: bool,
    room_area: Area,
}

fn main() {
    let mut room = Room {
        is_upstairs: true,
        number_of_doors: 1,
        wood_or_carpet: true,
        carpet_color: "Red".to_owned(),
        room_name: "Bedroom 1".to_owned(),
        has_wardrobe: true,
        room_area: Area {
            width: 2.3f32,
            length: 4.3f32,
        },
        window: vec![
            Window {
                window_area: Area {
                    width: 1.3f32,
                    length: 1.4f32,
                },
                window_type: "Main".to_owned(),
                has_blinds: true,
                curtain_color: "Blue".to_owned(),
                has_lock: false,
                top_open: true,
                single_window: true,
            },
            Window {
                window_area: Area {
                    width: 0.9f32,
                    length: 1.1f32,
                },
                window_type: "Small".to_owned(),
                has_blinds: true,
                curtain_color: "Blue".to_owned(),
                has_lock: false,
                top_open: true,
                single_window: true,
            },
        ],
    };

    println!(
        "Bedroom {} has {} door",
        room.room_name,
        room.number_of_doors
    );

    println!(
        "The room width is {}m by {}m",
        room.room_area.width,
        room.room_area.length
    );
    let ref window_two = room.window[1];
    println!(
        "Window 2 is {}m by {}m and has {} curtains",
        window_two.window_area.width,
        window_two.window_area.length,
        window_two.curtain_color
    );

}
