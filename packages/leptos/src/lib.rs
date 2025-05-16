//! Leptos port of [Lucide](https://lucide.dev/).
//!
//! Lucide is a beautiful & consistent icon toolkit made by the community.
//!
//! See [the Rust Lucide book](https://lucide.rustforweb.org/leptos.html) for more documenation.

#[cfg(any(feature = "text", feature = "design"))]
mod a_arrow_down;
#[cfg(any(feature = "text", feature = "design"))]
mod a_arrow_up;
#[cfg(any(feature = "text", feature = "design"))]
mod a_large_small;
#[cfg(any(feature = "accessibility", feature = "medical"))]
mod accessibility;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "social",
    feature = "science",
    feature = "multimedia"
))]
mod activity;
#[cfg(feature = "home")]
mod air_vent;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "brands"
))]
mod airplay;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
mod alarm_clock;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
mod alarm_clock_check;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
mod alarm_clock_minus;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
mod alarm_clock_off;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
mod alarm_clock_plus;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
mod alarm_smoke;
#[cfg(any(feature = "photography", feature = "multimedia"))]
mod album;
#[cfg(feature = "text")]
mod align_center;
#[cfg(feature = "layout")]
mod align_center_horizontal;
#[cfg(feature = "layout")]
mod align_center_vertical;
#[cfg(feature = "layout")]
mod align_end_horizontal;
#[cfg(feature = "layout")]
mod align_end_vertical;
#[cfg(feature = "layout")]
mod align_horizontal_distribute_center;
#[cfg(feature = "layout")]
mod align_horizontal_distribute_end;
#[cfg(feature = "layout")]
mod align_horizontal_distribute_start;
#[cfg(feature = "layout")]
mod align_horizontal_justify_center;
#[cfg(feature = "layout")]
mod align_horizontal_justify_end;
#[cfg(feature = "layout")]
mod align_horizontal_justify_start;
#[cfg(feature = "layout")]
mod align_horizontal_space_around;
#[cfg(feature = "layout")]
mod align_horizontal_space_between;
#[cfg(feature = "text")]
mod align_justify;
#[cfg(feature = "text")]
mod align_left;
#[cfg(feature = "text")]
mod align_right;
#[cfg(feature = "layout")]
mod align_start_horizontal;
#[cfg(feature = "layout")]
mod align_start_vertical;
#[cfg(feature = "layout")]
mod align_vertical_distribute_center;
#[cfg(feature = "layout")]
mod align_vertical_distribute_end;
#[cfg(feature = "layout")]
mod align_vertical_distribute_start;
#[cfg(feature = "layout")]
mod align_vertical_justify_center;
#[cfg(feature = "layout")]
mod align_vertical_justify_end;
#[cfg(feature = "layout")]
mod align_vertical_justify_start;
#[cfg(feature = "layout")]
mod align_vertical_space_around;
#[cfg(feature = "layout")]
mod align_vertical_space_between;
#[cfg(any(feature = "medical", feature = "transportation"))]
mod ambulance;
#[cfg(any(feature = "text", feature = "development"))]
mod ampersand;
#[cfg(any(feature = "text", feature = "development"))]
mod ampersands;
#[cfg(any(feature = "food-beverage", feature = "gaming"))]
mod amphora;
#[cfg(any(feature = "transportation", feature = "text"))]
mod anchor;
#[cfg(feature = "emoji")]
mod angry;
#[cfg(feature = "emoji")]
mod annoyed;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "communication"))]
mod antenna;
#[cfg(any(feature = "buildings", feature = "tools", feature = "gaming"))]
mod anvil;
#[cfg(feature = "photography")]
mod aperture;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
mod app_window;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
mod app_window_mac;
#[cfg(feature = "food-beverage")]
mod apple;
#[cfg(any(feature = "files", feature = "mail"))]
mod archive;
#[cfg(any(feature = "files", feature = "mail"))]
mod archive_restore;
#[cfg(any(feature = "files", feature = "mail"))]
mod archive_x;
#[cfg(feature = "home")]
mod armchair;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod arrow_big_down;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "gaming",
    feature = "files"
))]
mod arrow_big_down_dash;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod arrow_big_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod arrow_big_left_dash;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod arrow_big_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod arrow_big_right_dash;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "text",
    feature = "development",
    feature = "gaming"
))]
mod arrow_big_up;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "text",
    feature = "development",
    feature = "gaming"
))]
mod arrow_big_up_dash;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_down;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_0_1;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_1_0;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_a_z;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "files"))]
mod arrow_down_from_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_down_left;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_narrow_wide;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_down_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_down_to_dot;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "files",
    feature = "development"
))]
mod arrow_down_to_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_down_up;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_wide_narrow;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_down_z_a;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_left_from_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_left_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_left_to_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_right_from_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_right_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
mod arrow_right_to_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_up;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_0_1;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_1_0;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_a_z;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_up_down;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_up_from_dot;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "files",
    feature = "development"
))]
mod arrow_up_from_line;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_up_left;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_narrow_wide;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod arrow_up_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "files"))]
mod arrow_up_to_line;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_wide_narrow;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
mod arrow_up_z_a;
#[cfg(any(feature = "arrows", feature = "transportation", feature = "mail"))]
mod arrows_up_from_line;
#[cfg(any(feature = "text", feature = "math", feature = "development"))]
mod asterisk;
#[cfg(any(feature = "text", feature = "account"))]
mod at_sign;
#[cfg(feature = "science")]
mod atom;
#[cfg(any(feature = "multimedia", feature = "communication"))]
mod audio_lines;
#[cfg(any(feature = "multimedia", feature = "communication"))]
mod audio_waveform;
#[cfg(any(feature = "account", feature = "sports", feature = "gaming"))]
mod award;
#[cfg(any(feature = "tools", feature = "gaming"))]
mod axe;
#[cfg(feature = "design")]
mod axis_3_d;
#[cfg(any(feature = "accessibility", feature = "people"))]
mod baby;
#[cfg(any(feature = "gaming", feature = "photography", feature = "travel"))]
mod backpack;
#[cfg(any(feature = "account", feature = "social", feature = "shapes"))]
mod badge;
#[cfg(any(feature = "account", feature = "social"))]
mod badge_alert;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_cent;
#[cfg(feature = "social")]
mod badge_check;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_dollar_sign;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_euro;
#[cfg(any(feature = "accessibility", feature = "social"))]
mod badge_help;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_indian_rupee;
#[cfg(any(feature = "account", feature = "accessibility", feature = "social"))]
mod badge_info;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_japanese_yen;
#[cfg(feature = "social")]
mod badge_minus;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
mod badge_percent;
#[cfg(feature = "social")]
mod badge_plus;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_pound_sterling;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_russian_ruble;
#[cfg(any(feature = "shopping", feature = "finance"))]
mod badge_swiss_franc;
#[cfg(feature = "social")]
mod badge_x;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod baggage_claim;
#[cfg(feature = "account")]
mod ban;
#[cfg(feature = "food-beverage")]
mod banana;
#[cfg(feature = "medical")]
mod bandage;
#[cfg(feature = "finance")]
mod banknote;
#[cfg(feature = "finance")]
mod banknote_arrow_down;
#[cfg(feature = "finance")]
mod banknote_arrow_up;
#[cfg(feature = "finance")]
mod banknote_x;
#[cfg(feature = "shopping")]
mod barcode;
#[cfg(feature = "text")]
mod baseline;
#[cfg(feature = "travel")]
mod bath;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery_charging;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery_full;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery_low;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery_medium;
#[cfg(feature = "devices")]
mod battery_plus;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod battery_warning;
#[cfg(any(feature = "science", feature = "gaming"))]
mod beaker;
#[cfg(feature = "food-beverage")]
mod bean;
#[cfg(feature = "food-beverage")]
mod bean_off;
#[cfg(feature = "home")]
mod bed;
#[cfg(feature = "home")]
mod bed_double;
#[cfg(feature = "home")]
mod bed_single;
#[cfg(feature = "food-beverage")]
mod beef;
#[cfg(feature = "food-beverage")]
mod beer;
#[cfg(feature = "food-beverage")]
mod beer_off;
#[cfg(any(feature = "account", feature = "notifications"))]
mod bell;
#[cfg(any(feature = "account", feature = "notifications"))]
mod bell_dot;
#[cfg(any(feature = "devices", feature = "notifications", feature = "home"))]
mod bell_electric;
#[cfg(feature = "notifications")]
mod bell_minus;
#[cfg(feature = "notifications")]
mod bell_off;
#[cfg(feature = "notifications")]
mod bell_plus;
#[cfg(feature = "notifications")]
mod bell_ring;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
mod between_horizontal_end;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
mod between_horizontal_start;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
mod between_vertical_end;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
mod between_vertical_start;
#[cfg(feature = "emoji")]
mod biceps_flexed;
#[cfg(feature = "transportation")]
mod bike;
#[cfg(any(feature = "text", feature = "development"))]
mod binary;
#[cfg(any(
    feature = "navigation",
    feature = "nature",
    feature = "photography",
    feature = "science",
    feature = "travel",
    feature = "development"
))]
mod binoculars;
#[cfg(feature = "science")]
mod biohazard;
#[cfg(feature = "animals")]
mod bird;
#[cfg(any(feature = "brands", feature = "development", feature = "finance"))]
mod bitcoin;
#[cfg(any(
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "development"
))]
mod blend;
#[cfg(feature = "home")]
mod blinds;
#[cfg(any(feature = "development", feature = "shapes"))]
mod blocks;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod bluetooth;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod bluetooth_connected;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod bluetooth_off;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod bluetooth_searching;
#[cfg(feature = "text")]
mod bold;
#[cfg(any(feature = "tools", feature = "home"))]
mod bolt;
#[cfg(any(feature = "security", feature = "tools"))]
mod bomb;
#[cfg(any(feature = "animals", feature = "medical", feature = "gaming"))]
mod bone;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
mod book;
#[cfg(any(feature = "text", feature = "gaming"))]
mod book_a;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod book_audio;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
mod book_check;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
mod book_copy;
#[cfg(feature = "development")]
mod book_dashed;
#[cfg(feature = "development")]
mod book_down;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod book_headphones;
#[cfg(any(feature = "social", feature = "text", feature = "gaming"))]
mod book_heart;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files",
    feature = "social",
    feature = "shopping",
    feature = "travel"
))]
mod book_image;
#[cfg(any(feature = "development", feature = "security", feature = "gaming"))]
mod book_key;
#[cfg(any(feature = "development", feature = "security", feature = "gaming"))]
mod book_lock;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
mod book_marked;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
mod book_minus;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
mod book_open;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
mod book_open_check;
#[cfg(any(feature = "text", feature = "development"))]
mod book_open_text;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
mod book_plus;
#[cfg(any(feature = "text", feature = "gaming"))]
mod book_text;
#[cfg(any(feature = "text", feature = "design", feature = "gaming"))]
mod book_type;
#[cfg(feature = "development")]
mod book_up;
#[cfg(feature = "development")]
mod book_up_2;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
mod book_user;
#[cfg(any(feature = "text", feature = "gaming"))]
mod book_x;
#[cfg(feature = "account")]
mod bookmark;
#[cfg(feature = "account")]
mod bookmark_check;
#[cfg(feature = "account")]
mod bookmark_minus;
#[cfg(feature = "account")]
mod bookmark_plus;
#[cfg(feature = "account")]
mod bookmark_x;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
mod boom_box;
#[cfg(any(feature = "development", feature = "social"))]
mod bot;
#[cfg(any(feature = "development", feature = "social"))]
mod bot_message_square;
#[cfg(any(feature = "development", feature = "social"))]
mod bot_off;
#[cfg(any(feature = "gaming", feature = "tools"))]
mod bow_arrow;
#[cfg(any(
    feature = "shapes",
    feature = "gaming",
    feature = "development",
    feature = "math"
))]
mod r#box;
#[cfg(any(feature = "shapes", feature = "gaming", feature = "development"))]
mod boxes;
#[cfg(any(feature = "development", feature = "files"))]
mod braces;
#[cfg(any(feature = "development", feature = "files"))]
mod brackets;
#[cfg(any(feature = "medical", feature = "science"))]
mod brain;
#[cfg(any(feature = "science", feature = "development"))]
mod brain_circuit;
#[cfg(any(feature = "science", feature = "development"))]
mod brain_cog;
#[cfg(any(feature = "buildings", feature = "home"))]
mod brick_wall;
#[cfg(any(feature = "security", feature = "home", feature = "connectivity"))]
mod brick_wall_fire;
#[cfg(feature = "transportation")]
mod briefcase;
#[cfg(feature = "transportation")]
mod briefcase_business;
#[cfg(any(feature = "travel", feature = "transportation"))]
mod briefcase_conveyor_belt;
#[cfg(any(feature = "medical", feature = "transportation"))]
mod briefcase_medical;
#[cfg(any(feature = "design", feature = "layout"))]
mod bring_to_front;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod brush;
#[cfg(any(feature = "home", feature = "tools", feature = "design"))]
mod brush_cleaning;
#[cfg(feature = "weather")]
mod bubbles;
#[cfg(any(feature = "development", feature = "animals"))]
mod bug;
#[cfg(any(feature = "development", feature = "animals"))]
mod bug_off;
#[cfg(any(feature = "development", feature = "animals"))]
mod bug_play;
#[cfg(any(feature = "account", feature = "buildings"))]
mod building;
#[cfg(any(feature = "account", feature = "buildings"))]
mod building_2;
#[cfg(feature = "transportation")]
mod bus;
#[cfg(feature = "transportation")]
mod bus_front;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
mod cable;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod cable_car;
#[cfg(any(feature = "food-beverage", feature = "social", feature = "account"))]
mod cake;
#[cfg(any(feature = "food-beverage", feature = "social"))]
mod cake_slice;
#[cfg(any(feature = "math", feature = "devices"))]
mod calculator;
#[cfg(feature = "time")]
mod calendar;
#[cfg(feature = "time")]
mod calendar_1;
#[cfg(feature = "time")]
mod calendar_arrow_down;
#[cfg(feature = "time")]
mod calendar_arrow_up;
#[cfg(feature = "time")]
mod calendar_check;
#[cfg(feature = "time")]
mod calendar_check_2;
#[cfg(feature = "time")]
mod calendar_clock;
#[cfg(feature = "time")]
mod calendar_cog;
#[cfg(feature = "time")]
mod calendar_days;
#[cfg(any(feature = "time", feature = "files"))]
mod calendar_fold;
#[cfg(feature = "time")]
mod calendar_heart;
#[cfg(feature = "time")]
mod calendar_minus;
#[cfg(feature = "time")]
mod calendar_minus_2;
#[cfg(feature = "time")]
mod calendar_off;
#[cfg(feature = "time")]
mod calendar_plus;
#[cfg(feature = "time")]
mod calendar_plus_2;
#[cfg(feature = "time")]
mod calendar_range;
#[cfg(feature = "time")]
mod calendar_search;
#[cfg(any(feature = "arrows", feature = "time"))]
mod calendar_sync;
#[cfg(feature = "time")]
mod calendar_x;
#[cfg(feature = "time")]
mod calendar_x_2;
#[cfg(any(
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
mod camera;
#[cfg(any(
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
mod camera_off;
#[cfg(feature = "food-beverage")]
mod candy;
#[cfg(feature = "food-beverage")]
mod candy_cane;
#[cfg(feature = "food-beverage")]
mod candy_off;
#[cfg(feature = "nature")]
mod cannabis;
#[cfg(feature = "multimedia")]
mod captions;
#[cfg(feature = "multimedia")]
mod captions_off;
#[cfg(feature = "transportation")]
mod car;
#[cfg(feature = "transportation")]
mod car_front;
#[cfg(feature = "transportation")]
mod car_taxi_front;
#[cfg(any(feature = "transportation", feature = "travel", feature = "nature"))]
mod caravan;
#[cfg(feature = "food-beverage")]
mod carrot;
#[cfg(feature = "text")]
mod case_lower;
#[cfg(feature = "text")]
mod case_sensitive;
#[cfg(feature = "text")]
mod case_upper;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "multimedia",
    feature = "communication",
    feature = "files"
))]
mod cassette_tape;
#[cfg(any(feature = "devices", feature = "connectivity"))]
mod cast;
#[cfg(any(feature = "buildings", feature = "gaming"))]
mod castle;
#[cfg(feature = "animals")]
mod cat;
#[cfg(any(
    feature = "security",
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
mod cctv;
#[cfg(feature = "charts")]
mod chart_area;
#[cfg(feature = "charts")]
mod chart_bar;
#[cfg(feature = "charts")]
mod chart_bar_big;
#[cfg(feature = "charts")]
mod chart_bar_decreasing;
#[cfg(feature = "charts")]
mod chart_bar_increasing;
#[cfg(feature = "charts")]
mod chart_bar_stacked;
#[cfg(any(feature = "charts", feature = "finance"))]
mod chart_candlestick;
#[cfg(feature = "charts")]
mod chart_column;
#[cfg(feature = "charts")]
mod chart_column_big;
#[cfg(feature = "charts")]
mod chart_column_decreasing;
#[cfg(feature = "charts")]
mod chart_column_increasing;
#[cfg(feature = "charts")]
mod chart_column_stacked;
#[cfg(feature = "charts")]
mod chart_gantt;
#[cfg(feature = "charts")]
mod chart_line;
#[cfg(feature = "charts")]
mod chart_network;
#[cfg(feature = "charts")]
mod chart_no_axes_column;
#[cfg(feature = "charts")]
mod chart_no_axes_column_decreasing;
#[cfg(feature = "charts")]
mod chart_no_axes_column_increasing;
#[cfg(feature = "charts")]
mod chart_no_axes_combined;
#[cfg(any(
    feature = "charts",
    feature = "time",
    feature = "development",
    feature = "design"
))]
mod chart_no_axes_gantt;
#[cfg(any(feature = "charts", feature = "files"))]
mod chart_pie;
#[cfg(feature = "charts")]
mod chart_scatter;
#[cfg(feature = "charts")]
mod chart_spline;
#[cfg(feature = "notifications")]
mod check;
#[cfg(feature = "notifications")]
mod check_check;
#[cfg(feature = "food-beverage")]
mod chef_hat;
#[cfg(feature = "food-beverage")]
mod cherry;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod chevron_down;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod chevron_first;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod chevron_last;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod chevron_left;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "math",
    feature = "development"
))]
mod chevron_right;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "math",
    feature = "gaming"
))]
mod chevron_up;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod chevrons_down;
#[cfg(feature = "arrows")]
mod chevrons_down_up;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod chevrons_left;
#[cfg(feature = "arrows")]
mod chevrons_left_right;
#[cfg(any(
    feature = "communication",
    feature = "devices",
    feature = "multimedia",
    feature = "gaming"
))]
mod chevrons_left_right_ellipsis;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod chevrons_right;
#[cfg(feature = "arrows")]
mod chevrons_right_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod chevrons_up;
#[cfg(feature = "arrows")]
mod chevrons_up_down;
#[cfg(feature = "brands")]
mod chrome;
#[cfg(any(feature = "buildings", feature = "navigation"))]
mod church;
#[cfg(any(feature = "travel", feature = "transportation", feature = "medical"))]
mod cigarette;
#[cfg(any(feature = "travel", feature = "transportation", feature = "medical"))]
mod cigarette_off;
#[cfg(feature = "shapes")]
mod circle;
#[cfg(feature = "notifications")]
mod circle_alert;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod circle_arrow_down;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod circle_arrow_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_arrow_out_down_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_arrow_out_down_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
mod circle_arrow_out_up_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_arrow_out_up_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod circle_arrow_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod circle_arrow_up;
#[cfg(feature = "notifications")]
mod circle_check;
#[cfg(feature = "notifications")]
mod circle_check_big;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_chevron_down;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_chevron_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_chevron_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod circle_chevron_up;
#[cfg(any(feature = "development", feature = "shapes"))]
mod circle_dashed;
#[cfg(feature = "math")]
mod circle_divide;
#[cfg(feature = "finance")]
mod circle_dollar_sign;
#[cfg(any(feature = "development", feature = "shapes"))]
mod circle_dot;
#[cfg(any(feature = "development", feature = "shapes"))]
mod circle_dot_dashed;
#[cfg(any(feature = "layout", feature = "development"))]
mod circle_ellipsis;
#[cfg(feature = "math")]
mod circle_equal;
#[cfg(any(feature = "arrows", feature = "development"))]
mod circle_fading_arrow_up;
#[cfg(any(feature = "communication", feature = "social"))]
mod circle_fading_plus;
#[cfg(any(feature = "transportation", feature = "sports", feature = "science"))]
mod circle_gauge;
#[cfg(any(feature = "accessibility", feature = "text", feature = "notifications"))]
mod circle_help;
#[cfg(feature = "math")]
mod circle_minus;
#[cfg(feature = "shapes")]
mod circle_off;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod circle_parking;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod circle_parking_off;
#[cfg(feature = "multimedia")]
mod circle_pause;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
mod circle_percent;
#[cfg(feature = "multimedia")]
mod circle_play;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "cursors",
    feature = "gaming"
))]
mod circle_plus;
#[cfg(feature = "connectivity")]
mod circle_power;
#[cfg(any(feature = "development", feature = "math"))]
mod circle_slash;
#[cfg(any(feature = "shapes", feature = "math", feature = "development"))]
mod circle_slash_2;
#[cfg(any(feature = "shapes", feature = "medical"))]
mod circle_small;
#[cfg(feature = "multimedia")]
mod circle_stop;
#[cfg(feature = "account")]
mod circle_user;
#[cfg(feature = "account")]
mod circle_user_round;
#[cfg(any(feature = "math", feature = "development"))]
mod circle_x;
#[cfg(any(feature = "science", feature = "development"))]
mod circuit_board;
#[cfg(feature = "food-beverage")]
mod citrus;
#[cfg(feature = "multimedia")]
mod clapperboard;
#[cfg(feature = "text")]
mod clipboard;
#[cfg(feature = "text")]
mod clipboard_check;
#[cfg(any(feature = "text", feature = "arrows"))]
mod clipboard_copy;
#[cfg(feature = "text")]
mod clipboard_list;
#[cfg(any(feature = "text", feature = "medical"))]
mod clipboard_minus;
#[cfg(any(feature = "text", feature = "arrows"))]
mod clipboard_paste;
#[cfg(feature = "text")]
mod clipboard_pen;
#[cfg(feature = "text")]
mod clipboard_pen_line;
#[cfg(any(feature = "text", feature = "medical"))]
mod clipboard_plus;
#[cfg(feature = "text")]
mod clipboard_type;
#[cfg(feature = "text")]
mod clipboard_x;
#[cfg(feature = "time")]
mod clock;
#[cfg(feature = "time")]
mod clock_1;
#[cfg(feature = "time")]
mod clock_10;
#[cfg(feature = "time")]
mod clock_11;
#[cfg(feature = "time")]
mod clock_12;
#[cfg(feature = "time")]
mod clock_2;
#[cfg(feature = "time")]
mod clock_3;
#[cfg(feature = "time")]
mod clock_4;
#[cfg(feature = "time")]
mod clock_5;
#[cfg(feature = "time")]
mod clock_6;
#[cfg(feature = "time")]
mod clock_7;
#[cfg(feature = "time")]
mod clock_8;
#[cfg(feature = "time")]
mod clock_9;
#[cfg(feature = "time")]
mod clock_alert;
#[cfg(feature = "time")]
mod clock_arrow_down;
#[cfg(feature = "time")]
mod clock_arrow_up;
#[cfg(feature = "time")]
mod clock_fading;
#[cfg(feature = "time")]
mod clock_plus;
#[cfg(feature = "weather")]
mod cloud;
#[cfg(feature = "development")]
mod cloud_alert;
#[cfg(feature = "development")]
mod cloud_cog;
#[cfg(any(feature = "arrows", feature = "files"))]
mod cloud_download;
#[cfg(feature = "weather")]
mod cloud_drizzle;
#[cfg(feature = "weather")]
mod cloud_fog;
#[cfg(feature = "weather")]
mod cloud_hail;
#[cfg(feature = "weather")]
mod cloud_lightning;
#[cfg(feature = "weather")]
mod cloud_moon;
#[cfg(feature = "weather")]
mod cloud_moon_rain;
#[cfg(any(feature = "connectivity", feature = "weather"))]
mod cloud_off;
#[cfg(feature = "weather")]
mod cloud_rain;
#[cfg(feature = "weather")]
mod cloud_rain_wind;
#[cfg(feature = "weather")]
mod cloud_snow;
#[cfg(feature = "weather")]
mod cloud_sun;
#[cfg(feature = "weather")]
mod cloud_sun_rain;
#[cfg(any(feature = "arrows", feature = "files"))]
mod cloud_upload;
#[cfg(feature = "weather")]
mod cloudy;
#[cfg(feature = "gaming")]
mod clover;
#[cfg(any(feature = "shapes", feature = "gaming"))]
mod club;
#[cfg(any(feature = "text", feature = "development"))]
mod code;
#[cfg(any(feature = "text", feature = "development"))]
mod code_xml;
#[cfg(any(feature = "brands", feature = "development"))]
mod codepen;
#[cfg(any(feature = "brands", feature = "development"))]
mod codesandbox;
#[cfg(feature = "food-beverage")]
mod coffee;
#[cfg(feature = "account")]
mod cog;
#[cfg(feature = "gaming")]
mod coins;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
mod columns_2;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
mod columns_3;
#[cfg(any(feature = "layout", feature = "design"))]
mod columns_3_cog;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "security"
))]
mod columns_4;
#[cfg(any(feature = "development", feature = "files"))]
mod combine;
#[cfg(feature = "development")]
mod command;
#[cfg(any(feature = "navigation", feature = "travel"))]
mod compass;
#[cfg(any(feature = "design", feature = "development"))]
mod component;
#[cfg(any(feature = "devices", feature = "development", feature = "gaming"))]
mod computer;
#[cfg(feature = "travel")]
mod concierge_bell;
#[cfg(any(feature = "shapes", feature = "math"))]
mod cone;
#[cfg(feature = "development")]
mod construction;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
mod contact;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
mod contact_round;
#[cfg(any(feature = "development", feature = "transportation", feature = "mail"))]
mod container;
#[cfg(any(feature = "photography", feature = "accessibility", feature = "design"))]
mod contrast;
#[cfg(any(feature = "account", feature = "food-beverage"))]
mod cookie;
#[cfg(any(feature = "food-beverage", feature = "home"))]
mod cooking_pot;
#[cfg(feature = "text")]
mod copy;
#[cfg(any(feature = "text", feature = "notifications"))]
mod copy_check;
#[cfg(any(feature = "text", feature = "math"))]
mod copy_minus;
#[cfg(any(feature = "text", feature = "math"))]
mod copy_plus;
#[cfg(any(feature = "text", feature = "development", feature = "math"))]
mod copy_slash;
#[cfg(any(feature = "notifications", feature = "math"))]
mod copy_x;
#[cfg(feature = "text")]
mod copyleft;
#[cfg(feature = "text")]
mod copyright;
#[cfg(feature = "arrows")]
mod corner_down_left;
#[cfg(any(feature = "arrows", feature = "text", feature = "development"))]
mod corner_down_right;
#[cfg(feature = "arrows")]
mod corner_left_down;
#[cfg(feature = "arrows")]
mod corner_left_up;
#[cfg(feature = "arrows")]
mod corner_right_down;
#[cfg(feature = "arrows")]
mod corner_right_up;
#[cfg(feature = "arrows")]
mod corner_up_left;
#[cfg(feature = "arrows")]
mod corner_up_right;
#[cfg(feature = "devices")]
mod cpu;
#[cfg(feature = "text")]
mod creative_commons;
#[cfg(any(feature = "account", feature = "finance"))]
mod credit_card;
#[cfg(feature = "food-beverage")]
mod croissant;
#[cfg(any(feature = "photography", feature = "design"))]
mod crop;
#[cfg(feature = "shapes")]
mod cross;
#[cfg(feature = "photography")]
mod crosshair;
#[cfg(feature = "gaming")]
mod crown;
#[cfg(any(feature = "shapes", feature = "math", feature = "buildings"))]
mod cuboid;
#[cfg(feature = "food-beverage")]
mod cup_soda;
#[cfg(feature = "finance")]
mod currency;
#[cfg(any(feature = "shapes", feature = "design", feature = "math"))]
mod cylinder;
#[cfg(any(feature = "buildings", feature = "sustainability"))]
mod dam;
#[cfg(any(feature = "devices", feature = "development"))]
mod database;
#[cfg(any(
    feature = "devices",
    feature = "arrows",
    feature = "design",
    feature = "development",
    feature = "photography"
))]
mod database_backup;
#[cfg(any(feature = "devices", feature = "development"))]
mod database_zap;
#[cfg(any(
    feature = "design",
    feature = "text",
    feature = "arrows",
    feature = "math"
))]
mod decimals_arrow_left;
#[cfg(any(
    feature = "design",
    feature = "text",
    feature = "arrows",
    feature = "math"
))]
mod decimals_arrow_right;
#[cfg(any(feature = "text", feature = "arrows"))]
mod delete;
#[cfg(feature = "food-beverage")]
mod dessert;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
mod diameter;
#[cfg(any(feature = "shapes", feature = "gaming"))]
mod diamond;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "tools",
    feature = "devices"
))]
mod diamond_minus;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
mod diamond_percent;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "tools",
    feature = "devices"
))]
mod diamond_plus;
#[cfg(feature = "gaming")]
mod dice_1;
#[cfg(feature = "gaming")]
mod dice_2;
#[cfg(feature = "gaming")]
mod dice_3;
#[cfg(feature = "gaming")]
mod dice_4;
#[cfg(feature = "gaming")]
mod dice_5;
#[cfg(feature = "gaming")]
mod dice_6;
#[cfg(feature = "gaming")]
mod dices;
#[cfg(any(feature = "development", feature = "files"))]
mod diff;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod disc;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod disc_2;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod disc_3;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod disc_album;
#[cfg(any(feature = "math", feature = "development"))]
mod divide;
#[cfg(feature = "medical")]
mod dna;
#[cfg(any(feature = "medical", feature = "food-beverage"))]
mod dna_off;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
mod dock;
#[cfg(feature = "animals")]
mod dog;
#[cfg(feature = "finance")]
mod dollar_sign;
#[cfg(feature = "food-beverage")]
mod donut;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
mod door_closed;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
mod door_closed_locked;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
mod door_open;
#[cfg(any(feature = "shapes", feature = "text"))]
mod dot;
#[cfg(any(feature = "arrows", feature = "files"))]
mod download;
#[cfg(any(feature = "math", feature = "design", feature = "tools"))]
mod drafting_compass;
#[cfg(feature = "multimedia")]
mod drama;
#[cfg(any(feature = "brands", feature = "social", feature = "design"))]
mod dribbble;
#[cfg(any(feature = "tools", feature = "home", feature = "devices"))]
mod drill;
#[cfg(any(feature = "weather", feature = "gaming"))]
mod droplet;
#[cfg(any(feature = "weather", feature = "gaming"))]
mod droplet_off;
#[cfg(feature = "weather")]
mod droplets;
#[cfg(any(feature = "multimedia", feature = "devices"))]
mod drum;
#[cfg(feature = "food-beverage")]
mod drumstick;
#[cfg(any(feature = "navigation", feature = "sports"))]
mod dumbbell;
#[cfg(any(feature = "medical", feature = "accessibility"))]
mod ear;
#[cfg(any(feature = "medical", feature = "accessibility"))]
mod ear_off;
#[cfg(feature = "navigation")]
mod earth;
#[cfg(any(feature = "security", feature = "development", feature = "devices"))]
mod earth_lock;
#[cfg(any(
    feature = "science",
    feature = "design",
    feature = "development",
    feature = "accessibility",
    feature = "photography"
))]
mod eclipse;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
mod egg;
#[cfg(feature = "food-beverage")]
mod egg_fried;
#[cfg(feature = "food-beverage")]
mod egg_off;
#[cfg(any(feature = "layout", feature = "development"))]
mod ellipsis;
#[cfg(feature = "layout")]
mod ellipsis_vertical;
#[cfg(any(feature = "math", feature = "development"))]
mod equal;
#[cfg(feature = "math")]
mod equal_approximately;
#[cfg(any(feature = "math", feature = "development"))]
mod equal_not;
#[cfg(feature = "text")]
mod eraser;
#[cfg(any(
    feature = "communication",
    feature = "devices",
    feature = "multimedia",
    feature = "gaming"
))]
mod ethernet_port;
#[cfg(feature = "finance")]
mod euro;
#[cfg(any(feature = "text", feature = "arrows"))]
mod expand;
#[cfg(any(feature = "arrows", feature = "text", feature = "social"))]
mod external_link;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
mod eye;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
mod eye_closed;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
mod eye_off;
#[cfg(any(feature = "social", feature = "brands"))]
mod facebook;
#[cfg(feature = "buildings")]
mod factory;
#[cfg(feature = "home")]
mod fan;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod fast_forward;
#[cfg(feature = "gaming")]
mod feather;
#[cfg(any(feature = "home", feature = "buildings"))]
mod fence;
#[cfg(feature = "navigation")]
mod ferris_wheel;
#[cfg(any(feature = "brands", feature = "design"))]
mod figma;
#[cfg(feature = "files")]
mod file;
#[cfg(feature = "files")]
mod file_archive;
#[cfg(feature = "files")]
mod file_audio;
#[cfg(feature = "files")]
mod file_audio_2;
#[cfg(any(feature = "design", feature = "files"))]
mod file_axis_3_d;
#[cfg(feature = "files")]
mod file_badge;
#[cfg(feature = "files")]
mod file_badge_2;
#[cfg(feature = "files")]
mod file_box;
#[cfg(feature = "files")]
mod file_chart_column;
#[cfg(feature = "files")]
mod file_chart_column_increasing;
#[cfg(feature = "files")]
mod file_chart_line;
#[cfg(feature = "files")]
mod file_chart_pie;
#[cfg(feature = "files")]
mod file_check;
#[cfg(feature = "files")]
mod file_check_2;
#[cfg(any(feature = "files", feature = "time"))]
mod file_clock;
#[cfg(any(feature = "files", feature = "development"))]
mod file_code;
#[cfg(any(feature = "files", feature = "development"))]
mod file_code_2;
#[cfg(feature = "files")]
mod file_cog;
#[cfg(any(feature = "files", feature = "development"))]
mod file_diff;
#[cfg(any(feature = "files", feature = "development"))]
mod file_digit;
#[cfg(any(feature = "files", feature = "arrows"))]
mod file_down;
#[cfg(feature = "files")]
mod file_heart;
#[cfg(feature = "files")]
mod file_image;
#[cfg(any(feature = "files", feature = "arrows"))]
mod file_input;
#[cfg(any(feature = "files", feature = "development"))]
mod file_json;
#[cfg(any(feature = "files", feature = "development"))]
mod file_json_2;
#[cfg(any(feature = "files", feature = "security"))]
mod file_key;
#[cfg(any(feature = "files", feature = "security"))]
mod file_key_2;
#[cfg(any(feature = "files", feature = "security"))]
mod file_lock;
#[cfg(any(feature = "files", feature = "security"))]
mod file_lock_2;
#[cfg(feature = "files")]
mod file_minus;
#[cfg(feature = "files")]
mod file_minus_2;
#[cfg(any(feature = "files", feature = "multimedia"))]
mod file_music;
#[cfg(any(feature = "files", feature = "arrows"))]
mod file_output;
#[cfg(feature = "files")]
mod file_pen;
#[cfg(feature = "files")]
mod file_pen_line;
#[cfg(feature = "files")]
mod file_plus;
#[cfg(feature = "files")]
mod file_plus_2;
#[cfg(feature = "files")]
mod file_question;
#[cfg(feature = "files")]
mod file_scan;
#[cfg(feature = "files")]
mod file_search;
#[cfg(feature = "files")]
mod file_search_2;
#[cfg(any(feature = "files", feature = "development"))]
mod file_sliders;
#[cfg(feature = "files")]
mod file_spreadsheet;
#[cfg(any(feature = "files", feature = "development"))]
mod file_stack;
#[cfg(feature = "files")]
mod file_symlink;
#[cfg(any(feature = "files", feature = "development"))]
mod file_terminal;
#[cfg(any(feature = "files", feature = "text"))]
mod file_text;
#[cfg(any(feature = "files", feature = "text"))]
mod file_type;
#[cfg(any(feature = "files", feature = "text"))]
mod file_type_2;
#[cfg(any(feature = "files", feature = "arrows"))]
mod file_up;
#[cfg(any(feature = "account", feature = "files"))]
mod file_user;
#[cfg(feature = "files")]
mod file_video;
#[cfg(feature = "files")]
mod file_video_2;
#[cfg(feature = "files")]
mod file_volume;
#[cfg(feature = "files")]
mod file_volume_2;
#[cfg(any(feature = "files", feature = "notifications"))]
mod file_warning;
#[cfg(feature = "files")]
mod file_x;
#[cfg(feature = "files")]
mod file_x_2;
#[cfg(feature = "files")]
mod files;
#[cfg(any(feature = "photography", feature = "multimedia"))]
mod film;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "medical",
    feature = "devices"
))]
mod fingerprint;
#[cfg(any(feature = "home", feature = "tools", feature = "travel"))]
mod fire_extinguisher;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
mod fish;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
mod fish_off;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
mod fish_symbol;
#[cfg(any(feature = "account", feature = "social"))]
mod flag;
#[cfg(any(feature = "account", feature = "social"))]
mod flag_off;
#[cfg(feature = "development")]
mod flag_triangle_left;
#[cfg(feature = "development")]
mod flag_triangle_right;
#[cfg(any(feature = "weather", feature = "social", feature = "gaming"))]
mod flame;
#[cfg(any(feature = "nature", feature = "social", feature = "gaming"))]
mod flame_kindling;
#[cfg(any(feature = "photography", feature = "devices"))]
mod flashlight;
#[cfg(any(feature = "photography", feature = "devices"))]
mod flashlight_off;
#[cfg(any(feature = "science", feature = "gaming"))]
mod flask_conical;
#[cfg(any(feature = "science", feature = "gaming"))]
mod flask_conical_off;
#[cfg(any(feature = "science", feature = "gaming"))]
mod flask_round;
#[cfg(any(feature = "design", feature = "photography"))]
mod flip_horizontal;
#[cfg(any(feature = "design", feature = "photography"))]
mod flip_horizontal_2;
#[cfg(any(feature = "design", feature = "photography"))]
mod flip_vertical;
#[cfg(any(feature = "design", feature = "photography"))]
mod flip_vertical_2;
#[cfg(any(feature = "nature", feature = "gaming", feature = "sustainability"))]
mod flower;
#[cfg(any(feature = "nature", feature = "sustainability", feature = "seasons"))]
mod flower_2;
#[cfg(feature = "photography")]
mod focus;
#[cfg(any(feature = "arrows", feature = "layout"))]
mod fold_horizontal;
#[cfg(any(feature = "arrows", feature = "layout"))]
mod fold_vertical;
#[cfg(feature = "files")]
mod folder;
#[cfg(feature = "files")]
mod folder_archive;
#[cfg(feature = "files")]
mod folder_check;
#[cfg(any(feature = "files", feature = "time"))]
mod folder_clock;
#[cfg(feature = "files")]
mod folder_closed;
#[cfg(any(feature = "files", feature = "development"))]
mod folder_code;
#[cfg(feature = "files")]
mod folder_cog;
#[cfg(any(feature = "files", feature = "development"))]
mod folder_dot;
#[cfg(any(feature = "files", feature = "arrows"))]
mod folder_down;
#[cfg(feature = "files")]
mod folder_git;
#[cfg(feature = "files")]
mod folder_git_2;
#[cfg(feature = "files")]
mod folder_heart;
#[cfg(any(feature = "files", feature = "arrows"))]
mod folder_input;
#[cfg(any(
    feature = "charts",
    feature = "development",
    feature = "design",
    feature = "files"
))]
mod folder_kanban;
#[cfg(any(feature = "files", feature = "security"))]
mod folder_key;
#[cfg(any(feature = "files", feature = "security"))]
mod folder_lock;
#[cfg(feature = "files")]
mod folder_minus;
#[cfg(feature = "files")]
mod folder_open;
#[cfg(any(feature = "files", feature = "development"))]
mod folder_open_dot;
#[cfg(any(feature = "files", feature = "arrows"))]
mod folder_output;
#[cfg(feature = "files")]
mod folder_pen;
#[cfg(feature = "files")]
mod folder_plus;
#[cfg(any(feature = "files", feature = "development"))]
mod folder_root;
#[cfg(feature = "files")]
mod folder_search;
#[cfg(feature = "files")]
mod folder_search_2;
#[cfg(feature = "files")]
mod folder_symlink;
#[cfg(any(feature = "files", feature = "arrows"))]
mod folder_sync;
#[cfg(feature = "files")]
mod folder_tree;
#[cfg(any(feature = "files", feature = "arrows"))]
mod folder_up;
#[cfg(feature = "files")]
mod folder_x;
#[cfg(feature = "files")]
mod folders;
#[cfg(feature = "navigation")]
mod footprints;
#[cfg(feature = "transportation")]
mod forklift;
#[cfg(feature = "mail")]
mod forward;
#[cfg(any(feature = "design", feature = "photography"))]
mod frame;
#[cfg(any(feature = "brands", feature = "design"))]
mod framer;
#[cfg(any(feature = "emoji", feature = "account"))]
mod frown;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod fuel;
#[cfg(any(
    feature = "layout",
    feature = "multimedia",
    feature = "design",
    feature = "photography"
))]
mod fullscreen;
#[cfg(feature = "layout")]
mod funnel;
#[cfg(feature = "layout")]
mod funnel_plus;
#[cfg(feature = "layout")]
mod funnel_x;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
mod gallery_horizontal;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia",
    feature = "files"
))]
mod gallery_horizontal_end;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
mod gallery_thumbnails;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
mod gallery_vertical;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia",
    feature = "files"
))]
mod gallery_vertical_end;
#[cfg(any(feature = "gaming", feature = "devices"))]
mod gamepad;
#[cfg(any(feature = "gaming", feature = "devices"))]
mod gamepad_2;
#[cfg(any(feature = "transportation", feature = "sports", feature = "science"))]
mod gauge;
#[cfg(any(feature = "navigation", feature = "tools"))]
mod gavel;
#[cfg(any(feature = "gaming", feature = "development", feature = "finance"))]
mod gem;
#[cfg(feature = "gaming")]
mod ghost;
#[cfg(any(feature = "gaming", feature = "account"))]
mod gift;
#[cfg(feature = "development")]
mod git_branch;
#[cfg(feature = "development")]
mod git_branch_plus;
#[cfg(any(feature = "development", feature = "navigation"))]
mod git_commit_horizontal;
#[cfg(any(feature = "development", feature = "navigation"))]
mod git_commit_vertical;
#[cfg(feature = "development")]
mod git_compare;
#[cfg(any(feature = "development", feature = "arrows"))]
mod git_compare_arrows;
#[cfg(feature = "development")]
mod git_fork;
#[cfg(feature = "development")]
mod git_graph;
#[cfg(feature = "development")]
mod git_merge;
#[cfg(feature = "development")]
mod git_pull_request;
#[cfg(any(feature = "development", feature = "arrows"))]
mod git_pull_request_arrow;
#[cfg(feature = "development")]
mod git_pull_request_closed;
#[cfg(feature = "development")]
mod git_pull_request_create;
#[cfg(any(feature = "development", feature = "arrows"))]
mod git_pull_request_create_arrow;
#[cfg(feature = "development")]
mod git_pull_request_draft;
#[cfg(any(feature = "brands", feature = "development"))]
mod github;
#[cfg(any(feature = "brands", feature = "development"))]
mod gitlab;
#[cfg(feature = "food-beverage")]
mod glass_water;
#[cfg(feature = "accessibility")]
mod glasses;
#[cfg(feature = "navigation")]
mod globe;
#[cfg(any(feature = "security", feature = "development", feature = "devices"))]
mod globe_lock;
#[cfg(feature = "gaming")]
mod goal;
#[cfg(any(feature = "cursors", feature = "design", feature = "layout"))]
mod grab;
#[cfg(feature = "buildings")]
mod graduation_cap;
#[cfg(feature = "food-beverage")]
mod grape;
#[cfg(any(
    feature = "text",
    feature = "layout",
    feature = "design",
    feature = "math"
))]
mod grid_2_x_2;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
mod grid_2_x_2_check;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
mod grid_2_x_2_plus;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
mod grid_2_x_2_x;
#[cfg(any(feature = "text", feature = "layout", feature = "design"))]
mod grid_3_x_3;
#[cfg(feature = "layout")]
mod grip;
#[cfg(feature = "layout")]
mod grip_horizontal;
#[cfg(feature = "layout")]
mod grip_vertical;
#[cfg(feature = "files")]
mod group;
#[cfg(feature = "multimedia")]
mod guitar;
#[cfg(feature = "food-beverage")]
mod ham;
#[cfg(feature = "food-beverage")]
mod hamburger;
#[cfg(any(feature = "tools", feature = "home"))]
mod hammer;
#[cfg(any(feature = "cursors", feature = "accessibility"))]
mod hand;
#[cfg(any(feature = "finance", feature = "account"))]
mod hand_coins;
#[cfg(feature = "social")]
mod hand_heart;
#[cfg(feature = "emoji")]
mod hand_helping;
#[cfg(any(feature = "emoji", feature = "multimedia"))]
mod hand_metal;
#[cfg(any(feature = "food-beverage", feature = "people"))]
mod hand_platter;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "communication",
    feature = "finance",
    feature = "security"
))]
mod handshake;
#[cfg(any(feature = "development", feature = "devices"))]
mod hard_drive;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "arrows",
    feature = "files"
))]
mod hard_drive_download;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "arrows",
    feature = "files"
))]
mod hard_drive_upload;
#[cfg(feature = "tools")]
mod hard_hat;
#[cfg(any(feature = "text", feature = "social"))]
mod hash;
#[cfg(feature = "weather")]
mod haze;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "gaming"))]
mod hdmi_port;
#[cfg(feature = "text")]
mod heading;
#[cfg(feature = "text")]
mod heading_1;
#[cfg(feature = "text")]
mod heading_2;
#[cfg(feature = "text")]
mod heading_3;
#[cfg(feature = "text")]
mod heading_4;
#[cfg(feature = "text")]
mod heading_5;
#[cfg(feature = "text")]
mod heading_6;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "communication",
    feature = "devices",
    feature = "gaming"
))]
mod headphone_off;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "files",
    feature = "gaming"
))]
mod headphones;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "files",
    feature = "gaming"
))]
mod headset;
#[cfg(any(
    feature = "medical",
    feature = "social",
    feature = "multimedia",
    feature = "emoji",
    feature = "gaming",
    feature = "shapes"
))]
mod heart;
#[cfg(feature = "emoji")]
mod heart_crack;
#[cfg(any(feature = "emoji", feature = "account", feature = "security"))]
mod heart_handshake;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "multimedia",
    feature = "gaming",
    feature = "social"
))]
mod heart_minus;
#[cfg(any(feature = "social", feature = "multimedia"))]
mod heart_off;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "multimedia",
    feature = "gaming",
    feature = "social"
))]
mod heart_plus;
#[cfg(feature = "medical")]
mod heart_pulse;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
mod heater;
#[cfg(any(feature = "shapes", feature = "brands", feature = "development"))]
mod hexagon;
#[cfg(any(feature = "text", feature = "design"))]
mod highlighter;
#[cfg(any(feature = "arrows", feature = "time"))]
mod history;
#[cfg(feature = "food-beverage")]
mod hop;
#[cfg(feature = "food-beverage")]
mod hop_off;
#[cfg(any(
    feature = "medical",
    feature = "buildings",
    feature = "navigation",
    feature = "travel"
))]
mod hospital;
#[cfg(any(feature = "buildings", feature = "navigation", feature = "travel"))]
mod hotel;
#[cfg(any(feature = "time", feature = "gaming"))]
mod hourglass;
#[cfg(any(feature = "buildings", feature = "home"))]
mod house;
#[cfg(any(feature = "buildings", feature = "home", feature = "sustainability"))]
mod house_plug;
#[cfg(any(feature = "buildings", feature = "medical"))]
mod house_plus;
#[cfg(any(feature = "home", feature = "buildings", feature = "connectivity"))]
mod house_wifi;
#[cfg(feature = "food-beverage")]
mod ice_cream_bowl;
#[cfg(feature = "food-beverage")]
mod ice_cream_cone;
#[cfg(any(feature = "security", feature = "account"))]
mod id_card;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
mod image;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
mod image_down;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
mod image_minus;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
mod image_off;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
mod image_play;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
mod image_plus;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
mod image_up;
#[cfg(any(feature = "photography", feature = "multimedia"))]
mod image_upscale;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
mod images;
#[cfg(any(feature = "arrows", feature = "files"))]
mod import;
#[cfg(any(feature = "account", feature = "mail"))]
mod inbox;
#[cfg(any(feature = "text", feature = "development"))]
mod indent_decrease;
#[cfg(any(feature = "text", feature = "development"))]
mod indent_increase;
#[cfg(feature = "finance")]
mod indian_rupee;
#[cfg(feature = "multimedia")]
mod infinity;
#[cfg(any(feature = "accessibility", feature = "notifications"))]
mod info;
#[cfg(feature = "tools")]
mod inspection_panel;
#[cfg(any(feature = "brands", feature = "social", feature = "photography"))]
mod instagram;
#[cfg(feature = "text")]
mod italic;
#[cfg(any(feature = "arrows", feature = "design"))]
mod iteration_ccw;
#[cfg(any(feature = "arrows", feature = "design"))]
mod iteration_cw;
#[cfg(feature = "finance")]
mod japanese_yen;
#[cfg(any(feature = "gaming", feature = "devices"))]
mod joystick;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
mod kanban;
#[cfg(any(feature = "security", feature = "account"))]
mod key;
#[cfg(any(feature = "security", feature = "account"))]
mod key_round;
#[cfg(any(feature = "security", feature = "account"))]
mod key_square;
#[cfg(any(feature = "text", feature = "devices", feature = "development"))]
mod keyboard;
#[cfg(any(feature = "multimedia", feature = "devices"))]
mod keyboard_music;
#[cfg(any(feature = "devices", feature = "text", feature = "development"))]
mod keyboard_off;
#[cfg(feature = "home")]
mod lamp;
#[cfg(feature = "home")]
mod lamp_ceiling;
#[cfg(feature = "home")]
mod lamp_desk;
#[cfg(feature = "home")]
mod lamp_floor;
#[cfg(feature = "home")]
mod lamp_wall_down;
#[cfg(feature = "home")]
mod lamp_wall_up;
#[cfg(any(
    feature = "design",
    feature = "tools",
    feature = "math",
    feature = "sports",
    feature = "gaming"
))]
mod land_plot;
#[cfg(any(feature = "finance", feature = "navigation", feature = "buildings"))]
mod landmark;
#[cfg(feature = "text")]
mod languages;
#[cfg(feature = "devices")]
mod laptop;
#[cfg(feature = "devices")]
mod laptop_minimal;
#[cfg(any(feature = "devices", feature = "notifications"))]
mod laptop_minimal_check;
#[cfg(any(feature = "design", feature = "cursors"))]
mod lasso;
#[cfg(any(feature = "arrows", feature = "design", feature = "cursors"))]
mod lasso_select;
#[cfg(feature = "emoji")]
mod laugh;
#[cfg(any(feature = "design", feature = "layout"))]
mod layers;
#[cfg(any(feature = "design", feature = "layout"))]
mod layers_2;
#[cfg(any(feature = "design", feature = "layout"))]
mod layout_dashboard;
#[cfg(any(feature = "design", feature = "layout"))]
mod layout_grid;
#[cfg(any(
    feature = "design",
    feature = "layout",
    feature = "photography",
    feature = "text"
))]
mod layout_list;
#[cfg(any(feature = "design", feature = "layout"))]
mod layout_panel_left;
#[cfg(feature = "layout")]
mod layout_panel_top;
#[cfg(feature = "layout")]
mod layout_template;
#[cfg(any(feature = "nature", feature = "sustainability", feature = "seasons"))]
mod leaf;
#[cfg(any(
    feature = "food-beverage",
    feature = "emoji",
    feature = "sustainability"
))]
mod leafy_green;
#[cfg(any(feature = "communication", feature = "multimedia"))]
mod lectern;
#[cfg(feature = "text")]
mod letter_text;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
mod library;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
mod library_big;
#[cfg(any(feature = "accessibility", feature = "medical"))]
mod life_buoy;
#[cfg(feature = "text")]
mod ligature;
#[cfg(feature = "photography")]
mod lightbulb;
#[cfg(feature = "photography")]
mod lightbulb_off;
#[cfg(any(feature = "text", feature = "account"))]
mod link;
#[cfg(any(feature = "text", feature = "account"))]
mod link_2;
#[cfg(feature = "text")]
mod link_2_off;
#[cfg(any(feature = "social", feature = "brands"))]
mod linkedin;
#[cfg(feature = "text")]
mod list;
#[cfg(feature = "text")]
mod list_check;
#[cfg(feature = "text")]
mod list_checks;
#[cfg(feature = "text")]
mod list_collapse;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_end;
#[cfg(feature = "text")]
mod list_filter;
#[cfg(any(feature = "text", feature = "layout"))]
mod list_filter_plus;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_minus;
#[cfg(feature = "multimedia")]
mod list_music;
#[cfg(feature = "text")]
mod list_ordered;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_plus;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_restart;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_start;
#[cfg(feature = "text")]
mod list_todo;
#[cfg(any(feature = "files", feature = "text", feature = "layout"))]
mod list_tree;
#[cfg(feature = "multimedia")]
mod list_video;
#[cfg(any(feature = "multimedia", feature = "text"))]
mod list_x;
#[cfg(any(
    feature = "cursors",
    feature = "multimedia",
    feature = "layout",
    feature = "design"
))]
mod loader;
#[cfg(any(feature = "cursors", feature = "multimedia", feature = "layout"))]
mod loader_circle;
#[cfg(any(feature = "cursors", feature = "design"))]
mod loader_pinwheel;
#[cfg(feature = "navigation")]
mod locate;
#[cfg(feature = "navigation")]
mod locate_fixed;
#[cfg(feature = "navigation")]
mod locate_off;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod location_edit;
#[cfg(feature = "security")]
mod lock;
#[cfg(feature = "security")]
mod lock_keyhole;
#[cfg(feature = "security")]
mod lock_keyhole_open;
#[cfg(feature = "security")]
mod lock_open;
#[cfg(any(feature = "arrows", feature = "account"))]
mod log_in;
#[cfg(any(feature = "arrows", feature = "account"))]
mod log_out;
#[cfg(feature = "text")]
mod logs;
#[cfg(feature = "food-beverage")]
mod lollipop;
#[cfg(any(feature = "travel", feature = "transportation"))]
mod luggage;
#[cfg(feature = "design")]
mod magnet;
#[cfg(any(feature = "text", feature = "account", feature = "mail"))]
mod mail;
#[cfg(feature = "mail")]
mod mail_check;
#[cfg(feature = "mail")]
mod mail_minus;
#[cfg(feature = "mail")]
mod mail_open;
#[cfg(feature = "mail")]
mod mail_plus;
#[cfg(feature = "mail")]
mod mail_question;
#[cfg(feature = "mail")]
mod mail_search;
#[cfg(feature = "mail")]
mod mail_warning;
#[cfg(feature = "mail")]
mod mail_x;
#[cfg(feature = "mail")]
mod mailbox;
#[cfg(feature = "mail")]
mod mails;
#[cfg(any(feature = "text", feature = "navigation"))]
mod map;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_check;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_check_inside;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_house;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_minus;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_minus_inside;
#[cfg(any(feature = "navigation", feature = "travel"))]
mod map_pin_off;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_plus;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_plus_inside;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_x;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pin_x_inside;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
mod map_pinned;
#[cfg(feature = "navigation")]
mod map_plus;
#[cfg(feature = "medical")]
mod mars;
#[cfg(feature = "medical")]
mod mars_stroke;
#[cfg(feature = "food-beverage")]
mod martini;
#[cfg(any(feature = "layout", feature = "design"))]
mod maximize;
#[cfg(any(feature = "arrows", feature = "layout", feature = "design"))]
mod maximize_2;
#[cfg(any(feature = "sports", feature = "gaming"))]
mod medal;
#[cfg(any(feature = "multimedia", feature = "notifications"))]
mod megaphone;
#[cfg(any(feature = "multimedia", feature = "notifications"))]
mod megaphone_off;
#[cfg(feature = "emoji")]
mod meh;
#[cfg(any(feature = "devices", feature = "gaming"))]
mod memory_stick;
#[cfg(any(feature = "layout", feature = "account"))]
mod menu;
#[cfg(any(feature = "development", feature = "arrows"))]
mod merge;
#[cfg(feature = "social")]
mod message_circle;
#[cfg(any(feature = "development", feature = "social"))]
mod message_circle_code;
#[cfg(feature = "social")]
mod message_circle_dashed;
#[cfg(feature = "social")]
mod message_circle_heart;
#[cfg(feature = "social")]
mod message_circle_more;
#[cfg(feature = "social")]
mod message_circle_off;
#[cfg(feature = "social")]
mod message_circle_plus;
#[cfg(feature = "social")]
mod message_circle_question;
#[cfg(feature = "social")]
mod message_circle_reply;
#[cfg(any(feature = "social", feature = "notifications"))]
mod message_circle_warning;
#[cfg(any(feature = "account", feature = "social"))]
mod message_circle_x;
#[cfg(feature = "social")]
mod message_square;
#[cfg(any(feature = "development", feature = "social"))]
mod message_square_code;
#[cfg(feature = "social")]
mod message_square_dashed;
#[cfg(any(feature = "development", feature = "files", feature = "social"))]
mod message_square_diff;
#[cfg(any(feature = "social", feature = "notifications"))]
mod message_square_dot;
#[cfg(feature = "social")]
mod message_square_heart;
#[cfg(feature = "social")]
mod message_square_lock;
#[cfg(feature = "social")]
mod message_square_more;
#[cfg(feature = "social")]
mod message_square_off;
#[cfg(feature = "social")]
mod message_square_plus;
#[cfg(any(feature = "social", feature = "text"))]
mod message_square_quote;
#[cfg(feature = "social")]
mod message_square_reply;
#[cfg(feature = "social")]
mod message_square_share;
#[cfg(feature = "social")]
mod message_square_text;
#[cfg(any(feature = "social", feature = "notifications"))]
mod message_square_warning;
#[cfg(feature = "social")]
mod message_square_x;
#[cfg(feature = "social")]
mod messages_square;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "multimedia"
))]
mod mic;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "multimedia"
))]
mod mic_off;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod mic_vocal;
#[cfg(feature = "devices")]
mod microchip;
#[cfg(any(feature = "science", feature = "medical"))]
mod microscope;
#[cfg(any(feature = "food-beverage", feature = "home"))]
mod microwave;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
mod milestone;
#[cfg(feature = "food-beverage")]
mod milk;
#[cfg(feature = "food-beverage")]
mod milk_off;
#[cfg(any(feature = "layout", feature = "design"))]
mod minimize;
#[cfg(any(feature = "arrows", feature = "layout", feature = "design"))]
mod minimize_2;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "tools"
))]
mod minus;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_check;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_cog;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_dot;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_down;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_off;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
mod monitor_pause;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
mod monitor_play;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_smartphone;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_speaker;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
mod monitor_stop;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_up;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod monitor_x;
#[cfg(feature = "accessibility")]
mod moon;
#[cfg(any(feature = "accessibility", feature = "weather"))]
mod moon_star;
#[cfg(any(feature = "nature", feature = "gaming"))]
mod mountain;
#[cfg(feature = "nature")]
mod mountain_snow;
#[cfg(feature = "devices")]
mod mouse;
#[cfg(feature = "devices")]
mod mouse_off;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod mouse_pointer;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod mouse_pointer_2;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod mouse_pointer_ban;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod mouse_pointer_click;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod r#move;
#[cfg(feature = "design")]
mod move_3_d;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod move_diagonal;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod move_diagonal_2;
#[cfg(feature = "arrows")]
mod move_down;
#[cfg(feature = "arrows")]
mod move_down_left;
#[cfg(feature = "arrows")]
mod move_down_right;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod move_horizontal;
#[cfg(feature = "arrows")]
mod move_left;
#[cfg(feature = "arrows")]
mod move_right;
#[cfg(feature = "arrows")]
mod move_up;
#[cfg(feature = "arrows")]
mod move_up_left;
#[cfg(feature = "arrows")]
mod move_up_right;
#[cfg(any(feature = "arrows", feature = "cursors"))]
mod move_vertical;
#[cfg(any(feature = "multimedia", feature = "files"))]
mod music;
#[cfg(any(feature = "multimedia", feature = "files"))]
mod music_2;
#[cfg(any(feature = "multimedia", feature = "files"))]
mod music_3;
#[cfg(any(feature = "multimedia", feature = "files"))]
mod music_4;
#[cfg(feature = "navigation")]
mod navigation;
#[cfg(feature = "navigation")]
mod navigation_2;
#[cfg(feature = "navigation")]
mod navigation_2_off;
#[cfg(feature = "navigation")]
mod navigation_off;
#[cfg(feature = "development")]
mod network;
#[cfg(any(feature = "multimedia", feature = "communication"))]
mod newspaper;
#[cfg(any(feature = "communication", feature = "finance", feature = "devices"))]
mod nfc;
#[cfg(feature = "medical")]
mod non_binary;
#[cfg(any(
    feature = "text",
    feature = "communication",
    feature = "social",
    feature = "design"
))]
mod notebook;
#[cfg(any(feature = "text", feature = "social"))]
mod notebook_pen;
#[cfg(any(feature = "account", feature = "communication", feature = "social"))]
mod notebook_tabs;
#[cfg(any(feature = "text", feature = "social"))]
mod notebook_text;
#[cfg(any(feature = "text", feature = "social"))]
mod notepad_text;
#[cfg(any(feature = "text", feature = "social"))]
mod notepad_text_dashed;
#[cfg(feature = "food-beverage")]
mod nut;
#[cfg(feature = "food-beverage")]
mod nut_off;
#[cfg(feature = "shapes")]
mod octagon;
#[cfg(any(feature = "notifications", feature = "shapes"))]
mod octagon_alert;
#[cfg(feature = "transportation")]
mod octagon_minus;
#[cfg(any(feature = "multimedia", feature = "shapes"))]
mod octagon_pause;
#[cfg(any(feature = "math", feature = "notifications"))]
mod octagon_x;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "science"
))]
mod omega;
#[cfg(feature = "development")]
mod option;
#[cfg(feature = "science")]
mod orbit;
#[cfg(any(feature = "animals", feature = "design"))]
mod origami;
#[cfg(any(feature = "files", feature = "development"))]
mod package;
#[cfg(any(feature = "files", feature = "development"))]
mod package_2;
#[cfg(feature = "development")]
mod package_check;
#[cfg(feature = "development")]
mod package_minus;
#[cfg(any(feature = "files", feature = "development"))]
mod package_open;
#[cfg(feature = "development")]
mod package_plus;
#[cfg(any(feature = "files", feature = "development"))]
mod package_search;
#[cfg(feature = "development")]
mod package_x;
#[cfg(any(feature = "design", feature = "tools"))]
mod paint_bucket;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "home",
    feature = "tools"
))]
mod paint_roller;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "photography",
    feature = "home",
    feature = "tools"
))]
mod paintbrush;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "photography",
    feature = "home",
    feature = "tools"
))]
mod paintbrush_vertical;
#[cfg(any(feature = "text", feature = "design", feature = "photography"))]
mod palette;
#[cfg(feature = "animals")]
mod panda;
#[cfg(feature = "layout")]
mod panel_bottom;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_bottom_close;
#[cfg(feature = "layout")]
mod panel_bottom_dashed;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_bottom_open;
#[cfg(feature = "layout")]
mod panel_left;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_left_close;
#[cfg(feature = "layout")]
mod panel_left_dashed;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_left_open;
#[cfg(feature = "layout")]
mod panel_right;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_right_close;
#[cfg(feature = "layout")]
mod panel_right_dashed;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_right_open;
#[cfg(any(feature = "layout", feature = "design", feature = "development"))]
mod panel_top;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_top_close;
#[cfg(feature = "layout")]
mod panel_top_dashed;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod panel_top_open;
#[cfg(feature = "layout")]
mod panels_left_bottom;
#[cfg(feature = "layout")]
mod panels_right_bottom;
#[cfg(any(feature = "layout", feature = "design", feature = "development"))]
mod panels_top_left;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "files",
    feature = "mail"
))]
mod paperclip;
#[cfg(any(feature = "development", feature = "files", feature = "math"))]
mod parentheses;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod parking_meter;
#[cfg(feature = "emoji")]
mod party_popper;
#[cfg(feature = "multimedia")]
mod pause;
#[cfg(feature = "animals")]
mod paw_print;
#[cfg(any(feature = "devices", feature = "gaming"))]
mod pc_case;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod pen;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod pen_line;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod pen_off;
#[cfg(any(feature = "text", feature = "design", feature = "cursors"))]
mod pen_tool;
#[cfg(any(
    feature = "design",
    feature = "cursors",
    feature = "tools",
    feature = "text"
))]
mod pencil;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod pencil_line;
#[cfg(any(
    feature = "design",
    feature = "cursors",
    feature = "tools",
    feature = "text"
))]
mod pencil_off;
#[cfg(any(
    feature = "tools",
    feature = "design",
    feature = "layout",
    feature = "text"
))]
mod pencil_ruler;
#[cfg(feature = "shapes")]
mod pentagon;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "finance",
    feature = "shopping"
))]
mod percent;
#[cfg(any(feature = "accessibility", feature = "people"))]
mod person_standing;
#[cfg(feature = "finance")]
mod philippine_peso;
#[cfg(any(
    feature = "text",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_call;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_forwarded;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_incoming;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_missed;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_off;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod phone_outgoing;
#[cfg(any(feature = "development", feature = "math"))]
mod pi;
#[cfg(any(feature = "multimedia", feature = "devices"))]
mod piano;
#[cfg(any(feature = "tools", feature = "gaming"))]
mod pickaxe;
#[cfg(feature = "multimedia")]
mod picture_in_picture;
#[cfg(feature = "multimedia")]
mod picture_in_picture_2;
#[cfg(feature = "finance")]
mod piggy_bank;
#[cfg(feature = "text")]
mod pilcrow;
#[cfg(feature = "text")]
mod pilcrow_left;
#[cfg(feature = "text")]
mod pilcrow_right;
#[cfg(feature = "medical")]
mod pill;
#[cfg(feature = "medical")]
mod pill_bottle;
#[cfg(any(feature = "navigation", feature = "account"))]
mod pin;
#[cfg(feature = "navigation")]
mod pin_off;
#[cfg(any(feature = "text", feature = "design", feature = "science"))]
mod pipette;
#[cfg(feature = "food-beverage")]
mod pizza;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod plane;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod plane_landing;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod plane_takeoff;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod play;
#[cfg(any(feature = "devices", feature = "development"))]
mod plug;
#[cfg(any(feature = "devices", feature = "development"))]
mod plug_2;
#[cfg(feature = "devices")]
mod plug_zap;
#[cfg(any(
    feature = "math",
    feature = "tools",
    feature = "development",
    feature = "text",
    feature = "cursors",
    feature = "gaming"
))]
mod plus;
#[cfg(feature = "brands")]
mod pocket;
#[cfg(feature = "tools")]
mod pocket_knife;
#[cfg(any(feature = "multimedia", feature = "social"))]
mod podcast;
#[cfg(feature = "cursors")]
mod pointer;
#[cfg(feature = "cursors")]
mod pointer_off;
#[cfg(any(feature = "food-beverage", feature = "multimedia"))]
mod popcorn;
#[cfg(feature = "food-beverage")]
mod popsicle;
#[cfg(feature = "finance")]
mod pound_sterling;
#[cfg(feature = "connectivity")]
mod power;
#[cfg(feature = "connectivity")]
mod power_off;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "devices",
    feature = "communication",
    feature = "design"
))]
mod presentation;
#[cfg(feature = "devices")]
mod printer;
#[cfg(feature = "devices")]
mod printer_check;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
mod projector;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "devices"
))]
mod proportions;
#[cfg(any(feature = "development", feature = "gaming"))]
mod puzzle;
#[cfg(any(feature = "shapes", feature = "math", feature = "travel"))]
mod pyramid;
#[cfg(any(feature = "development", feature = "social"))]
mod qr_code;
#[cfg(feature = "text")]
mod quote;
#[cfg(feature = "animals")]
mod rabbit;
#[cfg(any(
    feature = "navigation",
    feature = "security",
    feature = "communication"
))]
mod radar;
#[cfg(feature = "science")]
mod radiation;
#[cfg(any(feature = "development", feature = "math"))]
mod radical;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
mod radio;
#[cfg(feature = "devices")]
mod radio_receiver;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
mod radio_tower;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
mod radius;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod rail_symbol;
#[cfg(feature = "weather")]
mod rainbow;
#[cfg(feature = "animals")]
mod rat;
#[cfg(any(feature = "layout", feature = "design", feature = "photography"))]
mod ratio;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_cent;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_euro;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_indian_rupee;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_japanese_yen;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_pound_sterling;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_russian_ruble;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_swiss_franc;
#[cfg(any(feature = "finance", feature = "travel"))]
mod receipt_text;
#[cfg(any(feature = "text", feature = "development"))]
mod rectangle_ellipsis;
#[cfg(any(
    feature = "devices",
    feature = "gaming",
    feature = "multimedia",
    feature = "connectivity"
))]
mod rectangle_goggles;
#[cfg(any(feature = "shapes", feature = "design"))]
mod rectangle_horizontal;
#[cfg(any(feature = "shapes", feature = "design"))]
mod rectangle_vertical;
#[cfg(feature = "sustainability")]
mod recycle;
#[cfg(any(feature = "text", feature = "arrows"))]
mod redo;
#[cfg(any(feature = "text", feature = "arrows"))]
mod redo_2;
#[cfg(any(feature = "text", feature = "arrows"))]
mod redo_dot;
#[cfg(feature = "arrows")]
mod refresh_ccw;
#[cfg(any(feature = "arrows", feature = "development"))]
mod refresh_ccw_dot;
#[cfg(feature = "arrows")]
mod refresh_cw;
#[cfg(feature = "arrows")]
mod refresh_cw_off;
#[cfg(any(feature = "food-beverage", feature = "home"))]
mod refrigerator;
#[cfg(any(feature = "text", feature = "development"))]
mod regex;
#[cfg(feature = "text")]
mod remove_formatting;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod repeat;
#[cfg(feature = "multimedia")]
mod repeat_1;
#[cfg(any(feature = "arrows", feature = "social", feature = "multimedia"))]
mod repeat_2;
#[cfg(feature = "text")]
mod replace;
#[cfg(feature = "text")]
mod replace_all;
#[cfg(feature = "mail")]
mod reply;
#[cfg(feature = "mail")]
mod reply_all;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod rewind;
#[cfg(any(feature = "social", feature = "medical", feature = "emoji"))]
mod ribbon;
#[cfg(any(feature = "gaming", feature = "development"))]
mod rocket;
#[cfg(feature = "home")]
mod rocking_chair;
#[cfg(feature = "navigation")]
mod roller_coaster;
#[cfg(feature = "design")]
mod rotate_3_d;
#[cfg(any(feature = "arrows", feature = "design", feature = "photography"))]
mod rotate_ccw;
#[cfg(any(feature = "security", feature = "account"))]
mod rotate_ccw_key;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "arrows"
))]
mod rotate_ccw_square;
#[cfg(any(feature = "arrows", feature = "design", feature = "photography"))]
mod rotate_cw;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "arrows"
))]
mod rotate_cw_square;
#[cfg(feature = "navigation")]
mod route;
#[cfg(feature = "navigation")]
mod route_off;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "connectivity",
    feature = "home"
))]
mod router;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
mod rows_2;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
mod rows_3;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
mod rows_4;
#[cfg(any(feature = "development", feature = "social"))]
mod rss;
#[cfg(any(feature = "tools", feature = "design", feature = "layout"))]
mod ruler;
#[cfg(any(feature = "tools", feature = "design", feature = "layout"))]
mod ruler_dimension_line;
#[cfg(feature = "finance")]
mod russian_ruble;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod sailboat;
#[cfg(any(feature = "food-beverage", feature = "emoji"))]
mod salad;
#[cfg(feature = "food-beverage")]
mod sandwich;
#[cfg(any(feature = "connectivity", feature = "science"))]
mod satellite;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
mod satellite_dish;
#[cfg(feature = "finance")]
mod saudi_riyal;
#[cfg(any(feature = "text", feature = "files"))]
mod save;
#[cfg(any(feature = "text", feature = "files"))]
mod save_all;
#[cfg(any(feature = "text", feature = "files"))]
mod save_off;
#[cfg(feature = "navigation")]
mod scale;
#[cfg(feature = "design")]
mod scale_3_d;
#[cfg(feature = "design")]
mod scaling;
#[cfg(any(
    feature = "devices",
    feature = "shopping",
    feature = "security",
    feature = "social",
    feature = "gaming"
))]
mod scan;
#[cfg(any(feature = "shopping", feature = "devices"))]
mod scan_barcode;
#[cfg(any(
    feature = "photography",
    feature = "multimedia",
    feature = "accessibility",
    feature = "security",
    feature = "devices",
    feature = "account"
))]
mod scan_eye;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "devices",
    feature = "social"
))]
mod scan_face;
#[cfg(feature = "medical")]
mod scan_heart;
#[cfg(any(feature = "devices", feature = "shopping"))]
mod scan_line;
#[cfg(any(
    feature = "account",
    feature = "shopping",
    feature = "devices",
    feature = "security"
))]
mod scan_qr_code;
#[cfg(any(
    feature = "photography",
    feature = "multimedia",
    feature = "accessibility"
))]
mod scan_search;
#[cfg(any(feature = "text", feature = "devices"))]
mod scan_text;
#[cfg(any(feature = "buildings", feature = "navigation"))]
mod school;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
mod scissors;
#[cfg(any(feature = "design", feature = "tools"))]
mod scissors_line_dashed;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod screen_share;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod screen_share_off;
#[cfg(any(feature = "gaming", feature = "development", feature = "text"))]
mod scroll;
#[cfg(any(feature = "gaming", feature = "development", feature = "text"))]
mod scroll_text;
#[cfg(any(feature = "text", feature = "social"))]
mod search;
#[cfg(any(feature = "text", feature = "social"))]
mod search_check;
#[cfg(any(feature = "text", feature = "social", feature = "development"))]
mod search_code;
#[cfg(any(feature = "text", feature = "social"))]
mod search_slash;
#[cfg(any(feature = "text", feature = "social"))]
mod search_x;
#[cfg(feature = "text")]
mod section;
#[cfg(any(feature = "mail", feature = "communication", feature = "connectivity"))]
mod send;
#[cfg(any(feature = "mail", feature = "communication", feature = "connectivity"))]
mod send_horizontal;
#[cfg(any(feature = "design", feature = "layout"))]
mod send_to_back;
#[cfg(any(feature = "text", feature = "arrows", feature = "layout"))]
mod separator_horizontal;
#[cfg(any(feature = "text", feature = "arrows", feature = "layout"))]
mod separator_vertical;
#[cfg(any(feature = "development", feature = "devices"))]
mod server;
#[cfg(any(feature = "development", feature = "devices"))]
mod server_cog;
#[cfg(any(feature = "development", feature = "devices"))]
mod server_crash;
#[cfg(any(feature = "development", feature = "devices"))]
mod server_off;
#[cfg(feature = "account")]
mod settings;
#[cfg(feature = "account")]
mod settings_2;
#[cfg(any(feature = "shapes", feature = "gaming"))]
mod shapes;
#[cfg(any(feature = "account", feature = "social"))]
mod share;
#[cfg(any(feature = "account", feature = "social"))]
mod share_2;
#[cfg(any(feature = "text", feature = "files"))]
mod sheet;
#[cfg(any(
    feature = "animals",
    feature = "development",
    feature = "nature",
    feature = "science",
    feature = "travel",
    feature = "food-beverage",
    feature = "home"
))]
mod shell;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming",
    feature = "shapes"
))]
mod shield;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "notifications",
    feature = "gaming"
))]
mod shield_alert;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_ban;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_check;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_ellipsis;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_half;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_minus;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_off;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming",
    feature = "medical"
))]
mod shield_plus;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_question;
#[cfg(any(feature = "account", feature = "security", feature = "development"))]
mod shield_user;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
mod shield_x;
#[cfg(any(feature = "transportation", feature = "navigation", feature = "travel"))]
mod ship;
#[cfg(any(feature = "transportation", feature = "navigation", feature = "travel"))]
mod ship_wheel;
#[cfg(feature = "shopping")]
mod shirt;
#[cfg(feature = "shopping")]
mod shopping_bag;
#[cfg(feature = "shopping")]
mod shopping_basket;
#[cfg(feature = "shopping")]
mod shopping_cart;
#[cfg(any(feature = "nature", feature = "tools", feature = "gaming"))]
mod shovel;
#[cfg(any(feature = "home", feature = "travel"))]
mod shower_head;
#[cfg(any(feature = "mail", feature = "files"))]
mod shredder;
#[cfg(feature = "animals")]
mod shrimp;
#[cfg(any(feature = "layout", feature = "arrows"))]
mod shrink;
#[cfg(feature = "nature")]
mod shrub;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod shuffle;
#[cfg(any(feature = "text", feature = "math", feature = "science"))]
mod sigma;
#[cfg(feature = "connectivity")]
mod signal;
#[cfg(feature = "connectivity")]
mod signal_high;
#[cfg(feature = "connectivity")]
mod signal_low;
#[cfg(feature = "connectivity")]
mod signal_medium;
#[cfg(feature = "connectivity")]
mod signal_zero;
#[cfg(feature = "text")]
mod signature;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
mod signpost;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
mod signpost_big;
#[cfg(feature = "medical")]
mod siren;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod skip_back;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod skip_forward;
#[cfg(feature = "gaming")]
mod skull;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "brands",
    feature = "development"
))]
mod slack;
#[cfg(any(feature = "development", feature = "math"))]
mod slash;
#[cfg(feature = "design")]
mod slice;
#[cfg(feature = "account")]
mod sliders_horizontal;
#[cfg(feature = "account")]
mod sliders_vertical;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod smartphone;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod smartphone_charging;
#[cfg(any(feature = "communication", feature = "finance", feature = "devices"))]
mod smartphone_nfc;
#[cfg(any(feature = "emoji", feature = "account"))]
mod smile;
#[cfg(any(
    feature = "emoji",
    feature = "social",
    feature = "notifications",
    feature = "communication"
))]
mod smile_plus;
#[cfg(any(feature = "animals", feature = "food-beverage"))]
mod snail;
#[cfg(any(feature = "weather", feature = "seasons"))]
mod snowflake;
#[cfg(any(feature = "home", feature = "travel"))]
mod soap_dispenser_droplet;
#[cfg(feature = "home")]
mod sofa;
#[cfg(feature = "food-beverage")]
mod soup;
#[cfg(feature = "text")]
mod space;
#[cfg(any(feature = "shapes", feature = "gaming"))]
mod spade;
#[cfg(feature = "shapes")]
mod sparkle;
#[cfg(any(
    feature = "cursors",
    feature = "multimedia",
    feature = "gaming",
    feature = "weather"
))]
mod sparkles;
#[cfg(any(feature = "multimedia", feature = "devices"))]
mod speaker;
#[cfg(any(feature = "accessibility", feature = "communication"))]
mod speech;
#[cfg(any(feature = "text", feature = "development"))]
mod spell_check;
#[cfg(any(feature = "text", feature = "development"))]
mod spell_check_2;
#[cfg(feature = "design")]
mod spline;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "design",
    feature = "tools"
))]
mod spline_pointer;
#[cfg(any(feature = "development", feature = "arrows"))]
mod split;
#[cfg(any(feature = "design", feature = "tools"))]
mod spray_can;
#[cfg(any(feature = "nature", feature = "gaming", feature = "sustainability"))]
mod sprout;
#[cfg(feature = "shapes")]
mod square;
#[cfg(any(
    feature = "medical",
    feature = "social",
    feature = "science",
    feature = "multimedia"
))]
mod square_activity;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod square_arrow_down;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod square_arrow_down_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
mod square_arrow_down_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_out_down_left;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_out_down_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_out_up_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "social"))]
mod square_arrow_out_up_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_right;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_up;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_arrow_up_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "social"))]
mod square_arrow_up_right;
#[cfg(any(
    feature = "text",
    feature = "security",
    feature = "math",
    feature = "development"
))]
mod square_asterisk;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "tools",
    feature = "files",
    feature = "development"
))]
mod square_bottom_dashed_scissors;
#[cfg(any(
    feature = "charts",
    feature = "time",
    feature = "development",
    feature = "design"
))]
mod square_chart_gantt;
#[cfg(feature = "notifications")]
mod square_check;
#[cfg(feature = "notifications")]
mod square_check_big;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_chevron_down;
#[cfg(any(feature = "arrows", feature = "navigation"))]
mod square_chevron_left;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
mod square_chevron_right;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "math"))]
mod square_chevron_up;
#[cfg(any(feature = "text", feature = "development"))]
mod square_code;
#[cfg(any(feature = "text", feature = "design"))]
mod square_dashed;
#[cfg(any(feature = "development", feature = "files"))]
mod square_dashed_bottom;
#[cfg(any(feature = "development", feature = "files"))]
mod square_dashed_bottom_code;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
mod square_dashed_kanban;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "development",
    feature = "tools"
))]
mod square_dashed_mouse_pointer;
#[cfg(feature = "math")]
mod square_divide;
#[cfg(feature = "development")]
mod square_dot;
#[cfg(feature = "math")]
mod square_equal;
#[cfg(any(feature = "development", feature = "math"))]
mod square_function;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
mod square_kanban;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
mod square_library;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod square_m;
#[cfg(feature = "layout")]
mod square_menu;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "tools",
    feature = "devices"
))]
mod square_minus;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "development",
    feature = "tools"
))]
mod square_mouse_pointer;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod square_parking;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod square_parking_off;
#[cfg(feature = "text")]
mod square_pen;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
mod square_percent;
#[cfg(any(feature = "development", feature = "math"))]
mod square_pi;
#[cfg(feature = "text")]
mod square_pilcrow;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
mod square_play;
#[cfg(any(
    feature = "math",
    feature = "tools",
    feature = "development",
    feature = "text"
))]
mod square_plus;
#[cfg(feature = "connectivity")]
mod square_power;
#[cfg(any(feature = "development", feature = "math"))]
mod square_radical;
#[cfg(any(feature = "design", feature = "development", feature = "layout"))]
mod square_round_corner;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "tools",
    feature = "files",
    feature = "development"
))]
mod square_scissors;
#[cfg(any(feature = "text", feature = "math"))]
mod square_sigma;
#[cfg(any(feature = "development", feature = "math"))]
mod square_slash;
#[cfg(feature = "layout")]
mod square_split_horizontal;
#[cfg(feature = "layout")]
mod square_split_vertical;
#[cfg(feature = "layout")]
mod square_square;
#[cfg(any(feature = "text", feature = "files", feature = "development"))]
mod square_stack;
#[cfg(feature = "development")]
mod square_terminal;
#[cfg(feature = "account")]
mod square_user;
#[cfg(feature = "account")]
mod square_user_round;
#[cfg(any(feature = "math", feature = "notifications"))]
mod square_x;
#[cfg(feature = "design")]
mod squares_exclude;
#[cfg(feature = "design")]
mod squares_intersect;
#[cfg(feature = "design")]
mod squares_subtract;
#[cfg(feature = "design")]
mod squares_unite;
#[cfg(feature = "shapes")]
mod squircle;
#[cfg(feature = "animals")]
mod squirrel;
#[cfg(any(feature = "design", feature = "cursors", feature = "tools"))]
mod stamp;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "shapes",
    feature = "multimedia",
    feature = "weather",
    feature = "emoji",
    feature = "gaming"
))]
mod star;
#[cfg(any(feature = "social", feature = "multimedia"))]
mod star_half;
#[cfg(any(feature = "multimedia", feature = "social"))]
mod star_off;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod step_back;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
mod step_forward;
#[cfg(any(feature = "science", feature = "medical"))]
mod stethoscope;
#[cfg(feature = "social")]
mod sticker;
#[cfg(any(feature = "text", feature = "social"))]
mod sticky_note;
#[cfg(any(feature = "buildings", feature = "navigation", feature = "shopping"))]
mod store;
#[cfg(feature = "layout")]
mod stretch_horizontal;
#[cfg(feature = "layout")]
mod stretch_vertical;
#[cfg(feature = "text")]
mod strikethrough;
#[cfg(feature = "text")]
mod subscript;
#[cfg(any(
    feature = "accessibility",
    feature = "weather",
    feature = "seasons",
    feature = "sustainability"
))]
mod sun;
#[cfg(any(feature = "accessibility", feature = "weather"))]
mod sun_dim;
#[cfg(any(feature = "accessibility", feature = "weather"))]
mod sun_medium;
#[cfg(feature = "accessibility")]
mod sun_moon;
#[cfg(feature = "weather")]
mod sun_snow;
#[cfg(any(feature = "arrows", feature = "weather", feature = "time"))]
mod sunrise;
#[cfg(any(feature = "arrows", feature = "weather"))]
mod sunset;
#[cfg(feature = "text")]
mod superscript;
#[cfg(any(feature = "design", feature = "home", feature = "photography"))]
mod swatch_book;
#[cfg(feature = "finance")]
mod swiss_franc;
#[cfg(any(feature = "communication", feature = "devices"))]
mod switch_camera;
#[cfg(any(feature = "gaming", feature = "tools"))]
mod sword;
#[cfg(any(feature = "gaming", feature = "tools"))]
mod swords;
#[cfg(any(feature = "science", feature = "medical"))]
mod syringe;
#[cfg(any(feature = "text", feature = "files"))]
mod table;
#[cfg(any(feature = "text", feature = "files"))]
mod table_2;
#[cfg(any(feature = "text", feature = "files"))]
mod table_cells_merge;
#[cfg(any(feature = "text", feature = "files"))]
mod table_cells_split;
#[cfg(any(feature = "text", feature = "files"))]
mod table_columns_split;
#[cfg(feature = "text")]
mod table_of_contents;
#[cfg(any(feature = "text", feature = "development", feature = "files"))]
mod table_properties;
#[cfg(any(feature = "text", feature = "files"))]
mod table_rows_split;
#[cfg(feature = "devices")]
mod tablet;
#[cfg(any(
    feature = "devices",
    feature = "design",
    feature = "development",
    feature = "tools"
))]
mod tablet_smartphone;
#[cfg(feature = "medical")]
mod tablets;
#[cfg(feature = "account")]
mod tag;
#[cfg(feature = "account")]
mod tags;
#[cfg(any(feature = "math", feature = "gaming"))]
mod tally_1;
#[cfg(any(feature = "math", feature = "gaming"))]
mod tally_2;
#[cfg(any(feature = "math", feature = "gaming"))]
mod tally_3;
#[cfg(any(feature = "math", feature = "gaming"))]
mod tally_4;
#[cfg(any(feature = "math", feature = "gaming"))]
mod tally_5;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
mod tangent;
#[cfg(any(feature = "brands", feature = "gaming"))]
mod target;
#[cfg(any(feature = "science", feature = "development", feature = "tools"))]
mod telescope;
#[cfg(any(feature = "travel", feature = "nature", feature = "sustainability"))]
mod tent;
#[cfg(any(feature = "travel", feature = "nature"))]
mod tent_tree;
#[cfg(feature = "development")]
mod terminal;
#[cfg(feature = "science")]
mod test_tube;
#[cfg(feature = "science")]
mod test_tube_diagonal;
#[cfg(feature = "science")]
mod test_tubes;
#[cfg(any(feature = "text", feature = "files", feature = "cursors"))]
mod text;
#[cfg(any(feature = "text", feature = "cursors"))]
mod text_cursor;
#[cfg(any(feature = "text", feature = "layout"))]
mod text_cursor_input;
#[cfg(feature = "text")]
mod text_quote;
#[cfg(feature = "text")]
mod text_search;
#[cfg(any(feature = "text", feature = "cursors"))]
mod text_select;
#[cfg(any(feature = "buildings", feature = "social"))]
mod theater;
#[cfg(feature = "weather")]
mod thermometer;
#[cfg(feature = "weather")]
mod thermometer_snowflake;
#[cfg(feature = "weather")]
mod thermometer_sun;
#[cfg(any(feature = "account", feature = "social", feature = "emoji"))]
mod thumbs_down;
#[cfg(any(feature = "account", feature = "social", feature = "emoji"))]
mod thumbs_up;
#[cfg(any(feature = "account", feature = "transportation"))]
mod ticket;
#[cfg(feature = "transportation")]
mod ticket_check;
#[cfg(feature = "transportation")]
mod ticket_minus;
#[cfg(any(feature = "transportation", feature = "shopping"))]
mod ticket_percent;
#[cfg(feature = "transportation")]
mod ticket_plus;
#[cfg(feature = "transportation")]
mod ticket_slash;
#[cfg(feature = "transportation")]
mod ticket_x;
#[cfg(any(feature = "travel", feature = "account", feature = "transportation"))]
mod tickets;
#[cfg(any(feature = "transportation", feature = "travel"))]
mod tickets_plane;
#[cfg(feature = "time")]
mod timer;
#[cfg(feature = "time")]
mod timer_off;
#[cfg(feature = "time")]
mod timer_reset;
#[cfg(any(feature = "layout", feature = "account", feature = "development"))]
mod toggle_left;
#[cfg(any(feature = "layout", feature = "account", feature = "development"))]
mod toggle_right;
#[cfg(any(feature = "devices", feature = "home"))]
mod toilet;
#[cfg(feature = "weather")]
mod tornado;
#[cfg(any(
    feature = "shapes",
    feature = "design",
    feature = "tools",
    feature = "food-beverage"
))]
mod torus;
#[cfg(feature = "devices")]
mod touchpad;
#[cfg(feature = "devices")]
mod touchpad_off;
#[cfg(any(feature = "travel", feature = "transportation"))]
mod tower_control;
#[cfg(any(feature = "gaming", feature = "development"))]
mod toy_brick;
#[cfg(any(
    feature = "transportation",
    feature = "sustainability",
    feature = "food-beverage"
))]
mod tractor;
#[cfg(feature = "transportation")]
mod traffic_cone;
#[cfg(feature = "transportation")]
mod train_front;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod train_front_tunnel;
#[cfg(any(feature = "transportation", feature = "navigation"))]
mod train_track;
#[cfg(feature = "transportation")]
mod tram_front;
#[cfg(any(feature = "medical", feature = "accessibility"))]
mod transgender;
#[cfg(any(feature = "files", feature = "mail"))]
mod trash;
#[cfg(any(feature = "files", feature = "mail"))]
mod trash_2;
#[cfg(any(feature = "nature", feature = "sustainability"))]
mod tree_deciduous;
#[cfg(any(feature = "nature", feature = "sustainability"))]
mod tree_palm;
#[cfg(any(feature = "nature", feature = "sustainability"))]
mod tree_pine;
#[cfg(any(feature = "nature", feature = "sustainability"))]
mod trees;
#[cfg(any(feature = "account", feature = "brands", feature = "development"))]
mod trello;
#[cfg(any(feature = "charts", feature = "arrows"))]
mod trending_down;
#[cfg(any(feature = "charts", feature = "arrows"))]
mod trending_up;
#[cfg(any(feature = "charts", feature = "arrows"))]
mod trending_up_down;
#[cfg(feature = "shapes")]
mod triangle;
#[cfg(any(feature = "notifications", feature = "shapes", feature = "development"))]
mod triangle_alert;
#[cfg(feature = "shapes")]
mod triangle_dashed;
#[cfg(any(feature = "shapes", feature = "math"))]
mod triangle_right;
#[cfg(any(feature = "sports", feature = "gaming"))]
mod trophy;
#[cfg(feature = "transportation")]
mod truck;
#[cfg(feature = "transportation")]
mod truck_electric;
#[cfg(feature = "animals")]
mod turtle;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "communication"))]
mod tv;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod tv_minimal;
#[cfg(any(feature = "devices", feature = "multimedia"))]
mod tv_minimal_play;
#[cfg(any(
    feature = "brands",
    feature = "social",
    feature = "account",
    feature = "gaming"
))]
mod twitch;
#[cfg(any(feature = "brands", feature = "social", feature = "account"))]
mod twitter;
#[cfg(feature = "text")]
mod r#type;
#[cfg(feature = "text")]
mod type_outline;
#[cfg(feature = "weather")]
mod umbrella;
#[cfg(feature = "weather")]
mod umbrella_off;
#[cfg(feature = "text")]
mod underline;
#[cfg(any(feature = "text", feature = "arrows"))]
mod undo;
#[cfg(any(feature = "text", feature = "arrows"))]
mod undo_2;
#[cfg(any(feature = "text", feature = "arrows"))]
mod undo_dot;
#[cfg(any(feature = "arrows", feature = "layout"))]
mod unfold_horizontal;
#[cfg(any(feature = "arrows", feature = "layout"))]
mod unfold_vertical;
#[cfg(any(feature = "shapes", feature = "files"))]
mod ungroup;
#[cfg(any(feature = "buildings", feature = "navigation"))]
mod university;
#[cfg(feature = "text")]
mod unlink;
#[cfg(feature = "text")]
mod unlink_2;
#[cfg(any(feature = "devices", feature = "development"))]
mod unplug;
#[cfg(any(feature = "arrows", feature = "files"))]
mod upload;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "home"))]
mod usb;
#[cfg(feature = "account")]
mod user;
#[cfg(feature = "account")]
mod user_check;
#[cfg(feature = "account")]
mod user_cog;
#[cfg(any(feature = "account", feature = "security"))]
mod user_lock;
#[cfg(feature = "account")]
mod user_minus;
#[cfg(feature = "account")]
mod user_pen;
#[cfg(feature = "account")]
mod user_plus;
#[cfg(feature = "account")]
mod user_round;
#[cfg(feature = "account")]
mod user_round_check;
#[cfg(feature = "account")]
mod user_round_cog;
#[cfg(feature = "account")]
mod user_round_minus;
#[cfg(feature = "account")]
mod user_round_pen;
#[cfg(feature = "account")]
mod user_round_plus;
#[cfg(any(feature = "account", feature = "social"))]
mod user_round_search;
#[cfg(feature = "account")]
mod user_round_x;
#[cfg(any(feature = "account", feature = "social"))]
mod user_search;
#[cfg(feature = "account")]
mod user_x;
#[cfg(feature = "account")]
mod users;
#[cfg(feature = "account")]
mod users_round;
#[cfg(any(feature = "food-beverage", feature = "travel", feature = "navigation"))]
mod utensils;
#[cfg(any(feature = "food-beverage", feature = "travel", feature = "navigation"))]
mod utensils_crossed;
#[cfg(any(feature = "buildings", feature = "home", feature = "sustainability"))]
mod utility_pole;
#[cfg(any(feature = "development", feature = "math"))]
mod variable;
#[cfg(any(feature = "security", feature = "travel", feature = "home"))]
mod vault;
#[cfg(any(feature = "food-beverage", feature = "sustainability"))]
mod vegan;
#[cfg(any(feature = "account", feature = "gaming"))]
mod venetian_mask;
#[cfg(feature = "medical")]
mod venus;
#[cfg(feature = "medical")]
mod venus_and_mars;
#[cfg(any(
    feature = "devices",
    feature = "connectivity",
    feature = "account",
    feature = "notifications"
))]
mod vibrate;
#[cfg(any(feature = "devices", feature = "connectivity", feature = "account"))]
mod vibrate_off;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
mod video;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
mod video_off;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography",
    feature = "files"
))]
mod videotape;
#[cfg(any(feature = "design", feature = "photography"))]
mod view;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "social"))]
mod voicemail;
#[cfg(any(feature = "sports", feature = "gaming", feature = "travel"))]
mod volleyball;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
mod volume;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
mod volume_1;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
mod volume_2;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
mod volume_off;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
mod volume_x;
#[cfg(feature = "social")]
mod vote;
#[cfg(any(feature = "account", feature = "finance"))]
mod wallet;
#[cfg(any(feature = "account", feature = "finance"))]
mod wallet_cards;
#[cfg(any(feature = "account", feature = "finance"))]
mod wallet_minimal;
#[cfg(any(feature = "account", feature = "devices"))]
mod wallpaper;
#[cfg(any(
    feature = "design",
    feature = "gaming",
    feature = "cursors",
    feature = "photography"
))]
mod wand;
#[cfg(any(
    feature = "design",
    feature = "gaming",
    feature = "cursors",
    feature = "photography"
))]
mod wand_sparkles;
#[cfg(any(feature = "buildings", feature = "navigation"))]
mod warehouse;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
mod washing_machine;
#[cfg(feature = "time")]
mod watch;
#[cfg(any(
    feature = "weather",
    feature = "navigation",
    feature = "multimedia",
    feature = "sustainability"
))]
mod waves;
#[cfg(any(feature = "sports", feature = "home"))]
mod waves_ladder;
#[cfg(any(
    feature = "security",
    feature = "account",
    feature = "navigation",
    feature = "development",
    feature = "social"
))]
mod waypoints;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
mod webcam;
#[cfg(any(feature = "development", feature = "social", feature = "account"))]
mod webhook;
#[cfg(any(feature = "development", feature = "social", feature = "account"))]
mod webhook_off;
#[cfg(feature = "math")]
mod weight;
#[cfg(feature = "food-beverage")]
mod wheat;
#[cfg(feature = "food-beverage")]
mod wheat_off;
#[cfg(feature = "text")]
mod whole_word;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi_high;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi_low;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi_off;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi_pen;
#[cfg(any(feature = "connectivity", feature = "devices"))]
mod wifi_zero;
#[cfg(any(feature = "weather", feature = "sustainability"))]
mod wind;
#[cfg(any(feature = "weather", feature = "sustainability"))]
mod wind_arrow_down;
#[cfg(feature = "food-beverage")]
mod wine;
#[cfg(feature = "food-beverage")]
mod wine_off;
#[cfg(feature = "development")]
mod workflow;
#[cfg(any(feature = "animals", feature = "security"))]
mod worm;
#[cfg(any(feature = "text", feature = "arrows"))]
mod wrap_text;
#[cfg(any(feature = "account", feature = "development", feature = "tools"))]
mod wrench;
#[cfg(any(feature = "notifications", feature = "math"))]
mod x;
#[cfg(any(feature = "multimedia", feature = "social", feature = "brands"))]
mod youtube;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "photography",
    feature = "weather"
))]
mod zap;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "photography",
    feature = "weather"
))]
mod zap_off;
#[cfg(any(
    feature = "accessibility",
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "photography"
))]
mod zoom_in;
#[cfg(any(
    feature = "accessibility",
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "photography"
))]
mod zoom_out;

#[cfg(any(feature = "text", feature = "design"))]
pub use a_arrow_down::*;
#[cfg(any(feature = "text", feature = "design"))]
pub use a_arrow_up::*;
#[cfg(any(feature = "text", feature = "design"))]
pub use a_large_small::*;
#[cfg(any(feature = "accessibility", feature = "medical"))]
pub use accessibility::*;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "social",
    feature = "science",
    feature = "multimedia"
))]
pub use activity::*;
#[cfg(feature = "home")]
pub use air_vent::*;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "brands"
))]
pub use airplay::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
pub use alarm_clock::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
pub use alarm_clock_check::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
pub use alarm_clock_minus::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
pub use alarm_clock_off::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "time"))]
pub use alarm_clock_plus::*;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
pub use alarm_smoke::*;
#[cfg(any(feature = "photography", feature = "multimedia"))]
pub use album::*;
#[cfg(feature = "text")]
pub use align_center::*;
#[cfg(feature = "layout")]
pub use align_center_horizontal::*;
#[cfg(feature = "layout")]
pub use align_center_vertical::*;
#[cfg(feature = "layout")]
pub use align_end_horizontal::*;
#[cfg(feature = "layout")]
pub use align_end_vertical::*;
#[cfg(feature = "layout")]
pub use align_horizontal_distribute_center::*;
#[cfg(feature = "layout")]
pub use align_horizontal_distribute_end::*;
#[cfg(feature = "layout")]
pub use align_horizontal_distribute_start::*;
#[cfg(feature = "layout")]
pub use align_horizontal_justify_center::*;
#[cfg(feature = "layout")]
pub use align_horizontal_justify_end::*;
#[cfg(feature = "layout")]
pub use align_horizontal_justify_start::*;
#[cfg(feature = "layout")]
pub use align_horizontal_space_around::*;
#[cfg(feature = "layout")]
pub use align_horizontal_space_between::*;
#[cfg(feature = "text")]
pub use align_justify::*;
#[cfg(feature = "text")]
pub use align_left::*;
#[cfg(feature = "text")]
pub use align_right::*;
#[cfg(feature = "layout")]
pub use align_start_horizontal::*;
#[cfg(feature = "layout")]
pub use align_start_vertical::*;
#[cfg(feature = "layout")]
pub use align_vertical_distribute_center::*;
#[cfg(feature = "layout")]
pub use align_vertical_distribute_end::*;
#[cfg(feature = "layout")]
pub use align_vertical_distribute_start::*;
#[cfg(feature = "layout")]
pub use align_vertical_justify_center::*;
#[cfg(feature = "layout")]
pub use align_vertical_justify_end::*;
#[cfg(feature = "layout")]
pub use align_vertical_justify_start::*;
#[cfg(feature = "layout")]
pub use align_vertical_space_around::*;
#[cfg(feature = "layout")]
pub use align_vertical_space_between::*;
#[cfg(any(feature = "medical", feature = "transportation"))]
pub use ambulance::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use ampersand::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use ampersands::*;
#[cfg(any(feature = "food-beverage", feature = "gaming"))]
pub use amphora::*;
#[cfg(any(feature = "transportation", feature = "text"))]
pub use anchor::*;
#[cfg(feature = "emoji")]
pub use angry::*;
#[cfg(feature = "emoji")]
pub use annoyed::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "communication"))]
pub use antenna::*;
#[cfg(any(feature = "buildings", feature = "tools", feature = "gaming"))]
pub use anvil::*;
#[cfg(feature = "photography")]
pub use aperture::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
pub use app_window::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
pub use app_window_mac::*;
#[cfg(feature = "food-beverage")]
pub use apple::*;
#[cfg(any(feature = "files", feature = "mail"))]
pub use archive::*;
#[cfg(any(feature = "files", feature = "mail"))]
pub use archive_restore::*;
#[cfg(any(feature = "files", feature = "mail"))]
pub use archive_x::*;
#[cfg(feature = "home")]
pub use armchair::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use arrow_big_down::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "gaming",
    feature = "files"
))]
pub use arrow_big_down_dash::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use arrow_big_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use arrow_big_left_dash::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use arrow_big_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use arrow_big_right_dash::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "text",
    feature = "development",
    feature = "gaming"
))]
pub use arrow_big_up::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "text",
    feature = "development",
    feature = "gaming"
))]
pub use arrow_big_up_dash::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_down::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_0_1::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_1_0::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_a_z::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "files"))]
pub use arrow_down_from_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_down_left::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_narrow_wide::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_down_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_down_to_dot::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "files",
    feature = "development"
))]
pub use arrow_down_to_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_down_up::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_wide_narrow::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_down_z_a::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_left_from_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_left_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_left_to_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_right_from_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_right_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
pub use arrow_right_to_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_up::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_0_1::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_1_0::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_a_z::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_up_down::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_up_from_dot::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "files",
    feature = "development"
))]
pub use arrow_up_from_line::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_up_left::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_narrow_wide::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use arrow_up_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "files"))]
pub use arrow_up_to_line::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_wide_narrow::*;
#[cfg(any(feature = "text", feature = "layout", feature = "arrows"))]
pub use arrow_up_z_a::*;
#[cfg(any(feature = "arrows", feature = "transportation", feature = "mail"))]
pub use arrows_up_from_line::*;
#[cfg(any(feature = "text", feature = "math", feature = "development"))]
pub use asterisk::*;
#[cfg(any(feature = "text", feature = "account"))]
pub use at_sign::*;
#[cfg(feature = "science")]
pub use atom::*;
#[cfg(any(feature = "multimedia", feature = "communication"))]
pub use audio_lines::*;
#[cfg(any(feature = "multimedia", feature = "communication"))]
pub use audio_waveform::*;
#[cfg(any(feature = "account", feature = "sports", feature = "gaming"))]
pub use award::*;
#[cfg(any(feature = "tools", feature = "gaming"))]
pub use axe::*;
#[cfg(feature = "design")]
pub use axis_3_d::*;
#[cfg(any(feature = "accessibility", feature = "people"))]
pub use baby::*;
#[cfg(any(feature = "gaming", feature = "photography", feature = "travel"))]
pub use backpack::*;
#[cfg(any(feature = "account", feature = "social", feature = "shapes"))]
pub use badge::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use badge_alert::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_cent::*;
#[cfg(feature = "social")]
pub use badge_check::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_dollar_sign::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_euro::*;
#[cfg(any(feature = "accessibility", feature = "social"))]
pub use badge_help::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_indian_rupee::*;
#[cfg(any(feature = "account", feature = "accessibility", feature = "social"))]
pub use badge_info::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_japanese_yen::*;
#[cfg(feature = "social")]
pub use badge_minus::*;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
pub use badge_percent::*;
#[cfg(feature = "social")]
pub use badge_plus::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_pound_sterling::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_russian_ruble::*;
#[cfg(any(feature = "shopping", feature = "finance"))]
pub use badge_swiss_franc::*;
#[cfg(feature = "social")]
pub use badge_x::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use baggage_claim::*;
#[cfg(feature = "account")]
pub use ban::*;
#[cfg(feature = "food-beverage")]
pub use banana::*;
#[cfg(feature = "medical")]
pub use bandage::*;
#[cfg(feature = "finance")]
pub use banknote::*;
#[cfg(feature = "finance")]
pub use banknote_arrow_down::*;
#[cfg(feature = "finance")]
pub use banknote_arrow_up::*;
#[cfg(feature = "finance")]
pub use banknote_x::*;
#[cfg(feature = "shopping")]
pub use barcode::*;
#[cfg(feature = "text")]
pub use baseline::*;
#[cfg(feature = "travel")]
pub use bath::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery_charging::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery_full::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery_low::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery_medium::*;
#[cfg(feature = "devices")]
pub use battery_plus::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use battery_warning::*;
#[cfg(any(feature = "science", feature = "gaming"))]
pub use beaker::*;
#[cfg(feature = "food-beverage")]
pub use bean::*;
#[cfg(feature = "food-beverage")]
pub use bean_off::*;
#[cfg(feature = "home")]
pub use bed::*;
#[cfg(feature = "home")]
pub use bed_double::*;
#[cfg(feature = "home")]
pub use bed_single::*;
#[cfg(feature = "food-beverage")]
pub use beef::*;
#[cfg(feature = "food-beverage")]
pub use beer::*;
#[cfg(feature = "food-beverage")]
pub use beer_off::*;
#[cfg(any(feature = "account", feature = "notifications"))]
pub use bell::*;
#[cfg(any(feature = "account", feature = "notifications"))]
pub use bell_dot::*;
#[cfg(any(feature = "devices", feature = "notifications", feature = "home"))]
pub use bell_electric::*;
#[cfg(feature = "notifications")]
pub use bell_minus::*;
#[cfg(feature = "notifications")]
pub use bell_off::*;
#[cfg(feature = "notifications")]
pub use bell_plus::*;
#[cfg(feature = "notifications")]
pub use bell_ring::*;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
pub use between_horizontal_end::*;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
pub use between_horizontal_start::*;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
pub use between_vertical_end::*;
#[cfg(any(feature = "layout", feature = "design", feature = "tools"))]
pub use between_vertical_start::*;
#[cfg(feature = "emoji")]
pub use biceps_flexed::*;
#[cfg(feature = "transportation")]
pub use bike::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use binary::*;
#[cfg(any(
    feature = "navigation",
    feature = "nature",
    feature = "photography",
    feature = "science",
    feature = "travel",
    feature = "development"
))]
pub use binoculars::*;
#[cfg(feature = "science")]
pub use biohazard::*;
#[cfg(feature = "animals")]
pub use bird::*;
#[cfg(any(feature = "brands", feature = "development", feature = "finance"))]
pub use bitcoin::*;
#[cfg(any(
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "development"
))]
pub use blend::*;
#[cfg(feature = "home")]
pub use blinds::*;
#[cfg(any(feature = "development", feature = "shapes"))]
pub use blocks::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use bluetooth::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use bluetooth_connected::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use bluetooth_off::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use bluetooth_searching::*;
#[cfg(feature = "text")]
pub use bold::*;
#[cfg(any(feature = "tools", feature = "home"))]
pub use bolt::*;
#[cfg(any(feature = "security", feature = "tools"))]
pub use bomb::*;
#[cfg(any(feature = "animals", feature = "medical", feature = "gaming"))]
pub use bone::*;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
pub use book::*;
#[cfg(any(feature = "text", feature = "gaming"))]
pub use book_a::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use book_audio::*;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
pub use book_check::*;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
pub use book_copy::*;
#[cfg(feature = "development")]
pub use book_dashed::*;
#[cfg(feature = "development")]
pub use book_down::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use book_headphones::*;
#[cfg(any(feature = "social", feature = "text", feature = "gaming"))]
pub use book_heart::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files",
    feature = "social",
    feature = "shopping",
    feature = "travel"
))]
pub use book_image::*;
#[cfg(any(feature = "development", feature = "security", feature = "gaming"))]
pub use book_key::*;
#[cfg(any(feature = "development", feature = "security", feature = "gaming"))]
pub use book_lock::*;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
pub use book_marked::*;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
pub use book_minus::*;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
pub use book_open::*;
#[cfg(any(feature = "text", feature = "development", feature = "gaming"))]
pub use book_open_check::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use book_open_text::*;
#[cfg(any(feature = "development", feature = "text", feature = "gaming"))]
pub use book_plus::*;
#[cfg(any(feature = "text", feature = "gaming"))]
pub use book_text::*;
#[cfg(any(feature = "text", feature = "design", feature = "gaming"))]
pub use book_type::*;
#[cfg(feature = "development")]
pub use book_up::*;
#[cfg(feature = "development")]
pub use book_up_2::*;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
pub use book_user::*;
#[cfg(any(feature = "text", feature = "gaming"))]
pub use book_x::*;
#[cfg(feature = "account")]
pub use bookmark::*;
#[cfg(feature = "account")]
pub use bookmark_check::*;
#[cfg(feature = "account")]
pub use bookmark_minus::*;
#[cfg(feature = "account")]
pub use bookmark_plus::*;
#[cfg(feature = "account")]
pub use bookmark_x::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
pub use boom_box::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use bot::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use bot_message_square::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use bot_off::*;
#[cfg(any(feature = "gaming", feature = "tools"))]
pub use bow_arrow::*;
#[cfg(any(
    feature = "shapes",
    feature = "gaming",
    feature = "development",
    feature = "math"
))]
pub use r#box::*;
#[cfg(any(feature = "shapes", feature = "gaming", feature = "development"))]
pub use boxes::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use braces::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use brackets::*;
#[cfg(any(feature = "medical", feature = "science"))]
pub use brain::*;
#[cfg(any(feature = "science", feature = "development"))]
pub use brain_circuit::*;
#[cfg(any(feature = "science", feature = "development"))]
pub use brain_cog::*;
#[cfg(any(feature = "buildings", feature = "home"))]
pub use brick_wall::*;
#[cfg(any(feature = "security", feature = "home", feature = "connectivity"))]
pub use brick_wall_fire::*;
#[cfg(feature = "transportation")]
pub use briefcase::*;
#[cfg(feature = "transportation")]
pub use briefcase_business::*;
#[cfg(any(feature = "travel", feature = "transportation"))]
pub use briefcase_conveyor_belt::*;
#[cfg(any(feature = "medical", feature = "transportation"))]
pub use briefcase_medical::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use bring_to_front::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use brush::*;
#[cfg(any(feature = "home", feature = "tools", feature = "design"))]
pub use brush_cleaning::*;
#[cfg(feature = "weather")]
pub use bubbles::*;
#[cfg(any(feature = "development", feature = "animals"))]
pub use bug::*;
#[cfg(any(feature = "development", feature = "animals"))]
pub use bug_off::*;
#[cfg(any(feature = "development", feature = "animals"))]
pub use bug_play::*;
#[cfg(any(feature = "account", feature = "buildings"))]
pub use building::*;
#[cfg(any(feature = "account", feature = "buildings"))]
pub use building_2::*;
#[cfg(feature = "transportation")]
pub use bus::*;
#[cfg(feature = "transportation")]
pub use bus_front::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
pub use cable::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use cable_car::*;
#[cfg(any(feature = "food-beverage", feature = "social", feature = "account"))]
pub use cake::*;
#[cfg(any(feature = "food-beverage", feature = "social"))]
pub use cake_slice::*;
#[cfg(any(feature = "math", feature = "devices"))]
pub use calculator::*;
#[cfg(feature = "time")]
pub use calendar::*;
#[cfg(feature = "time")]
pub use calendar_1::*;
#[cfg(feature = "time")]
pub use calendar_arrow_down::*;
#[cfg(feature = "time")]
pub use calendar_arrow_up::*;
#[cfg(feature = "time")]
pub use calendar_check::*;
#[cfg(feature = "time")]
pub use calendar_check_2::*;
#[cfg(feature = "time")]
pub use calendar_clock::*;
#[cfg(feature = "time")]
pub use calendar_cog::*;
#[cfg(feature = "time")]
pub use calendar_days::*;
#[cfg(any(feature = "time", feature = "files"))]
pub use calendar_fold::*;
#[cfg(feature = "time")]
pub use calendar_heart::*;
#[cfg(feature = "time")]
pub use calendar_minus::*;
#[cfg(feature = "time")]
pub use calendar_minus_2::*;
#[cfg(feature = "time")]
pub use calendar_off::*;
#[cfg(feature = "time")]
pub use calendar_plus::*;
#[cfg(feature = "time")]
pub use calendar_plus_2::*;
#[cfg(feature = "time")]
pub use calendar_range::*;
#[cfg(feature = "time")]
pub use calendar_search::*;
#[cfg(any(feature = "arrows", feature = "time"))]
pub use calendar_sync::*;
#[cfg(feature = "time")]
pub use calendar_x::*;
#[cfg(feature = "time")]
pub use calendar_x_2::*;
#[cfg(any(
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
pub use camera::*;
#[cfg(any(
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
pub use camera_off::*;
#[cfg(feature = "food-beverage")]
pub use candy::*;
#[cfg(feature = "food-beverage")]
pub use candy_cane::*;
#[cfg(feature = "food-beverage")]
pub use candy_off::*;
#[cfg(feature = "nature")]
pub use cannabis::*;
#[cfg(feature = "multimedia")]
pub use captions::*;
#[cfg(feature = "multimedia")]
pub use captions_off::*;
#[cfg(feature = "transportation")]
pub use car::*;
#[cfg(feature = "transportation")]
pub use car_front::*;
#[cfg(feature = "transportation")]
pub use car_taxi_front::*;
#[cfg(any(feature = "transportation", feature = "travel", feature = "nature"))]
pub use caravan::*;
#[cfg(feature = "food-beverage")]
pub use carrot::*;
#[cfg(feature = "text")]
pub use case_lower::*;
#[cfg(feature = "text")]
pub use case_sensitive::*;
#[cfg(feature = "text")]
pub use case_upper::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "multimedia",
    feature = "communication",
    feature = "files"
))]
pub use cassette_tape::*;
#[cfg(any(feature = "devices", feature = "connectivity"))]
pub use cast::*;
#[cfg(any(feature = "buildings", feature = "gaming"))]
pub use castle::*;
#[cfg(feature = "animals")]
pub use cat::*;
#[cfg(any(
    feature = "security",
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
pub use cctv::*;
#[cfg(feature = "charts")]
pub use chart_area::*;
#[cfg(feature = "charts")]
pub use chart_bar::*;
#[cfg(feature = "charts")]
pub use chart_bar_big::*;
#[cfg(feature = "charts")]
pub use chart_bar_decreasing::*;
#[cfg(feature = "charts")]
pub use chart_bar_increasing::*;
#[cfg(feature = "charts")]
pub use chart_bar_stacked::*;
#[cfg(any(feature = "charts", feature = "finance"))]
pub use chart_candlestick::*;
#[cfg(feature = "charts")]
pub use chart_column::*;
#[cfg(feature = "charts")]
pub use chart_column_big::*;
#[cfg(feature = "charts")]
pub use chart_column_decreasing::*;
#[cfg(feature = "charts")]
pub use chart_column_increasing::*;
#[cfg(feature = "charts")]
pub use chart_column_stacked::*;
#[cfg(feature = "charts")]
pub use chart_gantt::*;
#[cfg(feature = "charts")]
pub use chart_line::*;
#[cfg(feature = "charts")]
pub use chart_network::*;
#[cfg(feature = "charts")]
pub use chart_no_axes_column::*;
#[cfg(feature = "charts")]
pub use chart_no_axes_column_decreasing::*;
#[cfg(feature = "charts")]
pub use chart_no_axes_column_increasing::*;
#[cfg(feature = "charts")]
pub use chart_no_axes_combined::*;
#[cfg(any(
    feature = "charts",
    feature = "time",
    feature = "development",
    feature = "design"
))]
pub use chart_no_axes_gantt::*;
#[cfg(any(feature = "charts", feature = "files"))]
pub use chart_pie::*;
#[cfg(feature = "charts")]
pub use chart_scatter::*;
#[cfg(feature = "charts")]
pub use chart_spline::*;
#[cfg(feature = "notifications")]
pub use check::*;
#[cfg(feature = "notifications")]
pub use check_check::*;
#[cfg(feature = "food-beverage")]
pub use chef_hat::*;
#[cfg(feature = "food-beverage")]
pub use cherry::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use chevron_down::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use chevron_first::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use chevron_last::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use chevron_left::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "math",
    feature = "development"
))]
pub use chevron_right::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "math",
    feature = "gaming"
))]
pub use chevron_up::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use chevrons_down::*;
#[cfg(feature = "arrows")]
pub use chevrons_down_up::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use chevrons_left::*;
#[cfg(feature = "arrows")]
pub use chevrons_left_right::*;
#[cfg(any(
    feature = "communication",
    feature = "devices",
    feature = "multimedia",
    feature = "gaming"
))]
pub use chevrons_left_right_ellipsis::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use chevrons_right::*;
#[cfg(feature = "arrows")]
pub use chevrons_right_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use chevrons_up::*;
#[cfg(feature = "arrows")]
pub use chevrons_up_down::*;
#[cfg(feature = "brands")]
pub use chrome::*;
#[cfg(any(feature = "buildings", feature = "navigation"))]
pub use church::*;
#[cfg(any(feature = "travel", feature = "transportation", feature = "medical"))]
pub use cigarette::*;
#[cfg(any(feature = "travel", feature = "transportation", feature = "medical"))]
pub use cigarette_off::*;
#[cfg(feature = "shapes")]
pub use circle::*;
#[cfg(feature = "notifications")]
pub use circle_alert::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use circle_arrow_down::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use circle_arrow_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_arrow_out_down_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_arrow_out_down_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
pub use circle_arrow_out_up_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_arrow_out_up_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use circle_arrow_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use circle_arrow_up::*;
#[cfg(feature = "notifications")]
pub use circle_check::*;
#[cfg(feature = "notifications")]
pub use circle_check_big::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_chevron_down::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_chevron_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_chevron_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use circle_chevron_up::*;
#[cfg(any(feature = "development", feature = "shapes"))]
pub use circle_dashed::*;
#[cfg(feature = "math")]
pub use circle_divide::*;
#[cfg(feature = "finance")]
pub use circle_dollar_sign::*;
#[cfg(any(feature = "development", feature = "shapes"))]
pub use circle_dot::*;
#[cfg(any(feature = "development", feature = "shapes"))]
pub use circle_dot_dashed::*;
#[cfg(any(feature = "layout", feature = "development"))]
pub use circle_ellipsis::*;
#[cfg(feature = "math")]
pub use circle_equal::*;
#[cfg(any(feature = "arrows", feature = "development"))]
pub use circle_fading_arrow_up::*;
#[cfg(any(feature = "communication", feature = "social"))]
pub use circle_fading_plus::*;
#[cfg(any(feature = "transportation", feature = "sports", feature = "science"))]
pub use circle_gauge::*;
#[cfg(any(feature = "accessibility", feature = "text", feature = "notifications"))]
pub use circle_help::*;
#[cfg(feature = "math")]
pub use circle_minus::*;
#[cfg(feature = "shapes")]
pub use circle_off::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use circle_parking::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use circle_parking_off::*;
#[cfg(feature = "multimedia")]
pub use circle_pause::*;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
pub use circle_percent::*;
#[cfg(feature = "multimedia")]
pub use circle_play::*;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "cursors",
    feature = "gaming"
))]
pub use circle_plus::*;
#[cfg(feature = "connectivity")]
pub use circle_power::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use circle_slash::*;
#[cfg(any(feature = "shapes", feature = "math", feature = "development"))]
pub use circle_slash_2::*;
#[cfg(any(feature = "shapes", feature = "medical"))]
pub use circle_small::*;
#[cfg(feature = "multimedia")]
pub use circle_stop::*;
#[cfg(feature = "account")]
pub use circle_user::*;
#[cfg(feature = "account")]
pub use circle_user_round::*;
#[cfg(any(feature = "math", feature = "development"))]
pub use circle_x::*;
#[cfg(any(feature = "science", feature = "development"))]
pub use circuit_board::*;
#[cfg(feature = "food-beverage")]
pub use citrus::*;
#[cfg(feature = "multimedia")]
pub use clapperboard::*;
#[cfg(feature = "text")]
pub use clipboard::*;
#[cfg(feature = "text")]
pub use clipboard_check::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use clipboard_copy::*;
#[cfg(feature = "text")]
pub use clipboard_list::*;
#[cfg(any(feature = "text", feature = "medical"))]
pub use clipboard_minus::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use clipboard_paste::*;
#[cfg(feature = "text")]
pub use clipboard_pen::*;
#[cfg(feature = "text")]
pub use clipboard_pen_line::*;
#[cfg(any(feature = "text", feature = "medical"))]
pub use clipboard_plus::*;
#[cfg(feature = "text")]
pub use clipboard_type::*;
#[cfg(feature = "text")]
pub use clipboard_x::*;
#[cfg(feature = "time")]
pub use clock::*;
#[cfg(feature = "time")]
pub use clock_1::*;
#[cfg(feature = "time")]
pub use clock_2::*;
#[cfg(feature = "time")]
pub use clock_3::*;
#[cfg(feature = "time")]
pub use clock_4::*;
#[cfg(feature = "time")]
pub use clock_5::*;
#[cfg(feature = "time")]
pub use clock_6::*;
#[cfg(feature = "time")]
pub use clock_7::*;
#[cfg(feature = "time")]
pub use clock_8::*;
#[cfg(feature = "time")]
pub use clock_9::*;
#[cfg(feature = "time")]
pub use clock_10::*;
#[cfg(feature = "time")]
pub use clock_11::*;
#[cfg(feature = "time")]
pub use clock_12::*;
#[cfg(feature = "time")]
pub use clock_alert::*;
#[cfg(feature = "time")]
pub use clock_arrow_down::*;
#[cfg(feature = "time")]
pub use clock_arrow_up::*;
#[cfg(feature = "time")]
pub use clock_fading::*;
#[cfg(feature = "time")]
pub use clock_plus::*;
#[cfg(feature = "weather")]
pub use cloud::*;
#[cfg(feature = "development")]
pub use cloud_alert::*;
#[cfg(feature = "development")]
pub use cloud_cog::*;
#[cfg(any(feature = "arrows", feature = "files"))]
pub use cloud_download::*;
#[cfg(feature = "weather")]
pub use cloud_drizzle::*;
#[cfg(feature = "weather")]
pub use cloud_fog::*;
#[cfg(feature = "weather")]
pub use cloud_hail::*;
#[cfg(feature = "weather")]
pub use cloud_lightning::*;
#[cfg(feature = "weather")]
pub use cloud_moon::*;
#[cfg(feature = "weather")]
pub use cloud_moon_rain::*;
#[cfg(any(feature = "connectivity", feature = "weather"))]
pub use cloud_off::*;
#[cfg(feature = "weather")]
pub use cloud_rain::*;
#[cfg(feature = "weather")]
pub use cloud_rain_wind::*;
#[cfg(feature = "weather")]
pub use cloud_snow::*;
#[cfg(feature = "weather")]
pub use cloud_sun::*;
#[cfg(feature = "weather")]
pub use cloud_sun_rain::*;
#[cfg(any(feature = "arrows", feature = "files"))]
pub use cloud_upload::*;
#[cfg(feature = "weather")]
pub use cloudy::*;
#[cfg(feature = "gaming")]
pub use clover::*;
#[cfg(any(feature = "shapes", feature = "gaming"))]
pub use club::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use code::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use code_xml::*;
#[cfg(any(feature = "brands", feature = "development"))]
pub use codepen::*;
#[cfg(any(feature = "brands", feature = "development"))]
pub use codesandbox::*;
#[cfg(feature = "food-beverage")]
pub use coffee::*;
#[cfg(feature = "account")]
pub use cog::*;
#[cfg(feature = "gaming")]
pub use coins::*;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
pub use columns_2::*;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
pub use columns_3::*;
#[cfg(any(feature = "layout", feature = "design"))]
pub use columns_3_cog::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "security"
))]
pub use columns_4::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use combine::*;
#[cfg(feature = "development")]
pub use command::*;
#[cfg(any(feature = "navigation", feature = "travel"))]
pub use compass::*;
#[cfg(any(feature = "design", feature = "development"))]
pub use component::*;
#[cfg(any(feature = "devices", feature = "development", feature = "gaming"))]
pub use computer::*;
#[cfg(feature = "travel")]
pub use concierge_bell::*;
#[cfg(any(feature = "shapes", feature = "math"))]
pub use cone::*;
#[cfg(feature = "development")]
pub use construction::*;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
pub use contact::*;
#[cfg(any(
    feature = "account",
    feature = "connectivity",
    feature = "communication",
    feature = "social"
))]
pub use contact_round::*;
#[cfg(any(feature = "development", feature = "transportation", feature = "mail"))]
pub use container::*;
#[cfg(any(feature = "photography", feature = "accessibility", feature = "design"))]
pub use contrast::*;
#[cfg(any(feature = "account", feature = "food-beverage"))]
pub use cookie::*;
#[cfg(any(feature = "food-beverage", feature = "home"))]
pub use cooking_pot::*;
#[cfg(feature = "text")]
pub use copy::*;
#[cfg(any(feature = "text", feature = "notifications"))]
pub use copy_check::*;
#[cfg(any(feature = "text", feature = "math"))]
pub use copy_minus::*;
#[cfg(any(feature = "text", feature = "math"))]
pub use copy_plus::*;
#[cfg(any(feature = "text", feature = "development", feature = "math"))]
pub use copy_slash::*;
#[cfg(any(feature = "notifications", feature = "math"))]
pub use copy_x::*;
#[cfg(feature = "text")]
pub use copyleft::*;
#[cfg(feature = "text")]
pub use copyright::*;
#[cfg(feature = "arrows")]
pub use corner_down_left::*;
#[cfg(any(feature = "arrows", feature = "text", feature = "development"))]
pub use corner_down_right::*;
#[cfg(feature = "arrows")]
pub use corner_left_down::*;
#[cfg(feature = "arrows")]
pub use corner_left_up::*;
#[cfg(feature = "arrows")]
pub use corner_right_down::*;
#[cfg(feature = "arrows")]
pub use corner_right_up::*;
#[cfg(feature = "arrows")]
pub use corner_up_left::*;
#[cfg(feature = "arrows")]
pub use corner_up_right::*;
#[cfg(feature = "devices")]
pub use cpu::*;
#[cfg(feature = "text")]
pub use creative_commons::*;
#[cfg(any(feature = "account", feature = "finance"))]
pub use credit_card::*;
#[cfg(feature = "food-beverage")]
pub use croissant::*;
#[cfg(any(feature = "photography", feature = "design"))]
pub use crop::*;
#[cfg(feature = "shapes")]
pub use cross::*;
#[cfg(feature = "photography")]
pub use crosshair::*;
#[cfg(feature = "gaming")]
pub use crown::*;
#[cfg(any(feature = "shapes", feature = "math", feature = "buildings"))]
pub use cuboid::*;
#[cfg(feature = "food-beverage")]
pub use cup_soda::*;
#[cfg(feature = "finance")]
pub use currency::*;
#[cfg(any(feature = "shapes", feature = "design", feature = "math"))]
pub use cylinder::*;
#[cfg(any(feature = "buildings", feature = "sustainability"))]
pub use dam::*;
#[cfg(any(feature = "devices", feature = "development"))]
pub use database::*;
#[cfg(any(
    feature = "devices",
    feature = "arrows",
    feature = "design",
    feature = "development",
    feature = "photography"
))]
pub use database_backup::*;
#[cfg(any(feature = "devices", feature = "development"))]
pub use database_zap::*;
#[cfg(any(
    feature = "design",
    feature = "text",
    feature = "arrows",
    feature = "math"
))]
pub use decimals_arrow_left::*;
#[cfg(any(
    feature = "design",
    feature = "text",
    feature = "arrows",
    feature = "math"
))]
pub use decimals_arrow_right::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use delete::*;
#[cfg(feature = "food-beverage")]
pub use dessert::*;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
pub use diameter::*;
#[cfg(any(feature = "shapes", feature = "gaming"))]
pub use diamond::*;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "tools",
    feature = "devices"
))]
pub use diamond_minus::*;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
pub use diamond_percent::*;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "tools",
    feature = "devices"
))]
pub use diamond_plus::*;
#[cfg(feature = "gaming")]
pub use dice_1::*;
#[cfg(feature = "gaming")]
pub use dice_2::*;
#[cfg(feature = "gaming")]
pub use dice_3::*;
#[cfg(feature = "gaming")]
pub use dice_4::*;
#[cfg(feature = "gaming")]
pub use dice_5::*;
#[cfg(feature = "gaming")]
pub use dice_6::*;
#[cfg(feature = "gaming")]
pub use dices::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use diff::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use disc::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use disc_2::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use disc_3::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use disc_album::*;
#[cfg(any(feature = "math", feature = "development"))]
pub use divide::*;
#[cfg(feature = "medical")]
pub use dna::*;
#[cfg(any(feature = "medical", feature = "food-beverage"))]
pub use dna_off::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "files"
))]
pub use dock::*;
#[cfg(feature = "animals")]
pub use dog::*;
#[cfg(feature = "finance")]
pub use dollar_sign::*;
#[cfg(feature = "food-beverage")]
pub use donut::*;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
pub use door_closed::*;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
pub use door_closed_locked::*;
#[cfg(any(feature = "home", feature = "travel", feature = "security"))]
pub use door_open::*;
#[cfg(any(feature = "shapes", feature = "text"))]
pub use dot::*;
#[cfg(any(feature = "arrows", feature = "files"))]
pub use download::*;
#[cfg(any(feature = "math", feature = "design", feature = "tools"))]
pub use drafting_compass::*;
#[cfg(feature = "multimedia")]
pub use drama::*;
#[cfg(any(feature = "brands", feature = "social", feature = "design"))]
pub use dribbble::*;
#[cfg(any(feature = "tools", feature = "home", feature = "devices"))]
pub use drill::*;
#[cfg(any(feature = "weather", feature = "gaming"))]
pub use droplet::*;
#[cfg(any(feature = "weather", feature = "gaming"))]
pub use droplet_off::*;
#[cfg(feature = "weather")]
pub use droplets::*;
#[cfg(any(feature = "multimedia", feature = "devices"))]
pub use drum::*;
#[cfg(feature = "food-beverage")]
pub use drumstick::*;
#[cfg(any(feature = "navigation", feature = "sports"))]
pub use dumbbell::*;
#[cfg(any(feature = "medical", feature = "accessibility"))]
pub use ear::*;
#[cfg(any(feature = "medical", feature = "accessibility"))]
pub use ear_off::*;
#[cfg(feature = "navigation")]
pub use earth::*;
#[cfg(any(feature = "security", feature = "development", feature = "devices"))]
pub use earth_lock::*;
#[cfg(any(
    feature = "science",
    feature = "design",
    feature = "development",
    feature = "accessibility",
    feature = "photography"
))]
pub use eclipse::*;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
pub use egg::*;
#[cfg(feature = "food-beverage")]
pub use egg_fried::*;
#[cfg(feature = "food-beverage")]
pub use egg_off::*;
#[cfg(any(feature = "layout", feature = "development"))]
pub use ellipsis::*;
#[cfg(feature = "layout")]
pub use ellipsis_vertical::*;
#[cfg(any(feature = "math", feature = "development"))]
pub use equal::*;
#[cfg(feature = "math")]
pub use equal_approximately::*;
#[cfg(any(feature = "math", feature = "development"))]
pub use equal_not::*;
#[cfg(feature = "text")]
pub use eraser::*;
#[cfg(any(
    feature = "communication",
    feature = "devices",
    feature = "multimedia",
    feature = "gaming"
))]
pub use ethernet_port::*;
#[cfg(feature = "finance")]
pub use euro::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use expand::*;
#[cfg(any(feature = "arrows", feature = "text", feature = "social"))]
pub use external_link::*;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
pub use eye::*;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
pub use eye_closed::*;
#[cfg(any(
    feature = "accessibility",
    feature = "photography",
    feature = "design",
    feature = "security"
))]
pub use eye_off::*;
#[cfg(any(feature = "social", feature = "brands"))]
pub use facebook::*;
#[cfg(feature = "buildings")]
pub use factory::*;
#[cfg(feature = "home")]
pub use fan::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use fast_forward::*;
#[cfg(feature = "gaming")]
pub use feather::*;
#[cfg(any(feature = "home", feature = "buildings"))]
pub use fence::*;
#[cfg(feature = "navigation")]
pub use ferris_wheel::*;
#[cfg(any(feature = "brands", feature = "design"))]
pub use figma::*;
#[cfg(feature = "files")]
pub use file::*;
#[cfg(feature = "files")]
pub use file_archive::*;
#[cfg(feature = "files")]
pub use file_audio::*;
#[cfg(feature = "files")]
pub use file_audio_2::*;
#[cfg(any(feature = "design", feature = "files"))]
pub use file_axis_3_d::*;
#[cfg(feature = "files")]
pub use file_badge::*;
#[cfg(feature = "files")]
pub use file_badge_2::*;
#[cfg(feature = "files")]
pub use file_box::*;
#[cfg(feature = "files")]
pub use file_chart_column::*;
#[cfg(feature = "files")]
pub use file_chart_column_increasing::*;
#[cfg(feature = "files")]
pub use file_chart_line::*;
#[cfg(feature = "files")]
pub use file_chart_pie::*;
#[cfg(feature = "files")]
pub use file_check::*;
#[cfg(feature = "files")]
pub use file_check_2::*;
#[cfg(any(feature = "files", feature = "time"))]
pub use file_clock::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_code::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_code_2::*;
#[cfg(feature = "files")]
pub use file_cog::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_diff::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_digit::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use file_down::*;
#[cfg(feature = "files")]
pub use file_heart::*;
#[cfg(feature = "files")]
pub use file_image::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use file_input::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_json::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_json_2::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use file_key::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use file_key_2::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use file_lock::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use file_lock_2::*;
#[cfg(feature = "files")]
pub use file_minus::*;
#[cfg(feature = "files")]
pub use file_minus_2::*;
#[cfg(any(feature = "files", feature = "multimedia"))]
pub use file_music::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use file_output::*;
#[cfg(feature = "files")]
pub use file_pen::*;
#[cfg(feature = "files")]
pub use file_pen_line::*;
#[cfg(feature = "files")]
pub use file_plus::*;
#[cfg(feature = "files")]
pub use file_plus_2::*;
#[cfg(feature = "files")]
pub use file_question::*;
#[cfg(feature = "files")]
pub use file_scan::*;
#[cfg(feature = "files")]
pub use file_search::*;
#[cfg(feature = "files")]
pub use file_search_2::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_sliders::*;
#[cfg(feature = "files")]
pub use file_spreadsheet::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_stack::*;
#[cfg(feature = "files")]
pub use file_symlink::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use file_terminal::*;
#[cfg(any(feature = "files", feature = "text"))]
pub use file_text::*;
#[cfg(any(feature = "files", feature = "text"))]
pub use file_type::*;
#[cfg(any(feature = "files", feature = "text"))]
pub use file_type_2::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use file_up::*;
#[cfg(any(feature = "account", feature = "files"))]
pub use file_user::*;
#[cfg(feature = "files")]
pub use file_video::*;
#[cfg(feature = "files")]
pub use file_video_2::*;
#[cfg(feature = "files")]
pub use file_volume::*;
#[cfg(feature = "files")]
pub use file_volume_2::*;
#[cfg(any(feature = "files", feature = "notifications"))]
pub use file_warning::*;
#[cfg(feature = "files")]
pub use file_x::*;
#[cfg(feature = "files")]
pub use file_x_2::*;
#[cfg(feature = "files")]
pub use files::*;
#[cfg(any(feature = "photography", feature = "multimedia"))]
pub use film::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "medical",
    feature = "devices"
))]
pub use fingerprint::*;
#[cfg(any(feature = "home", feature = "tools", feature = "travel"))]
pub use fire_extinguisher::*;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
pub use fish::*;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
pub use fish_off::*;
#[cfg(any(feature = "food-beverage", feature = "animals"))]
pub use fish_symbol::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use flag::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use flag_off::*;
#[cfg(feature = "development")]
pub use flag_triangle_left::*;
#[cfg(feature = "development")]
pub use flag_triangle_right::*;
#[cfg(any(feature = "weather", feature = "social", feature = "gaming"))]
pub use flame::*;
#[cfg(any(feature = "nature", feature = "social", feature = "gaming"))]
pub use flame_kindling::*;
#[cfg(any(feature = "photography", feature = "devices"))]
pub use flashlight::*;
#[cfg(any(feature = "photography", feature = "devices"))]
pub use flashlight_off::*;
#[cfg(any(feature = "science", feature = "gaming"))]
pub use flask_conical::*;
#[cfg(any(feature = "science", feature = "gaming"))]
pub use flask_conical_off::*;
#[cfg(any(feature = "science", feature = "gaming"))]
pub use flask_round::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use flip_horizontal::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use flip_horizontal_2::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use flip_vertical::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use flip_vertical_2::*;
#[cfg(any(feature = "nature", feature = "gaming", feature = "sustainability"))]
pub use flower::*;
#[cfg(any(feature = "nature", feature = "sustainability", feature = "seasons"))]
pub use flower_2::*;
#[cfg(feature = "photography")]
pub use focus::*;
#[cfg(any(feature = "arrows", feature = "layout"))]
pub use fold_horizontal::*;
#[cfg(any(feature = "arrows", feature = "layout"))]
pub use fold_vertical::*;
#[cfg(feature = "files")]
pub use folder::*;
#[cfg(feature = "files")]
pub use folder_archive::*;
#[cfg(feature = "files")]
pub use folder_check::*;
#[cfg(any(feature = "files", feature = "time"))]
pub use folder_clock::*;
#[cfg(feature = "files")]
pub use folder_closed::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use folder_code::*;
#[cfg(feature = "files")]
pub use folder_cog::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use folder_dot::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use folder_down::*;
#[cfg(feature = "files")]
pub use folder_git::*;
#[cfg(feature = "files")]
pub use folder_git_2::*;
#[cfg(feature = "files")]
pub use folder_heart::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use folder_input::*;
#[cfg(any(
    feature = "charts",
    feature = "development",
    feature = "design",
    feature = "files"
))]
pub use folder_kanban::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use folder_key::*;
#[cfg(any(feature = "files", feature = "security"))]
pub use folder_lock::*;
#[cfg(feature = "files")]
pub use folder_minus::*;
#[cfg(feature = "files")]
pub use folder_open::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use folder_open_dot::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use folder_output::*;
#[cfg(feature = "files")]
pub use folder_pen::*;
#[cfg(feature = "files")]
pub use folder_plus::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use folder_root::*;
#[cfg(feature = "files")]
pub use folder_search::*;
#[cfg(feature = "files")]
pub use folder_search_2::*;
#[cfg(feature = "files")]
pub use folder_symlink::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use folder_sync::*;
#[cfg(feature = "files")]
pub use folder_tree::*;
#[cfg(any(feature = "files", feature = "arrows"))]
pub use folder_up::*;
#[cfg(feature = "files")]
pub use folder_x::*;
#[cfg(feature = "files")]
pub use folders::*;
#[cfg(feature = "navigation")]
pub use footprints::*;
#[cfg(feature = "transportation")]
pub use forklift::*;
#[cfg(feature = "mail")]
pub use forward::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use frame::*;
#[cfg(any(feature = "brands", feature = "design"))]
pub use framer::*;
#[cfg(any(feature = "emoji", feature = "account"))]
pub use frown::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use fuel::*;
#[cfg(any(
    feature = "layout",
    feature = "multimedia",
    feature = "design",
    feature = "photography"
))]
pub use fullscreen::*;
#[cfg(feature = "layout")]
pub use funnel::*;
#[cfg(feature = "layout")]
pub use funnel_plus::*;
#[cfg(feature = "layout")]
pub use funnel_x::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
pub use gallery_horizontal::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia",
    feature = "files"
))]
pub use gallery_horizontal_end::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
pub use gallery_thumbnails::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia"
))]
pub use gallery_vertical::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "development",
    feature = "photography",
    feature = "multimedia",
    feature = "files"
))]
pub use gallery_vertical_end::*;
#[cfg(any(feature = "gaming", feature = "devices"))]
pub use gamepad::*;
#[cfg(any(feature = "gaming", feature = "devices"))]
pub use gamepad_2::*;
#[cfg(any(feature = "transportation", feature = "sports", feature = "science"))]
pub use gauge::*;
#[cfg(any(feature = "navigation", feature = "tools"))]
pub use gavel::*;
#[cfg(any(feature = "gaming", feature = "development", feature = "finance"))]
pub use gem::*;
#[cfg(feature = "gaming")]
pub use ghost::*;
#[cfg(any(feature = "gaming", feature = "account"))]
pub use gift::*;
#[cfg(feature = "development")]
pub use git_branch::*;
#[cfg(feature = "development")]
pub use git_branch_plus::*;
#[cfg(any(feature = "development", feature = "navigation"))]
pub use git_commit_horizontal::*;
#[cfg(any(feature = "development", feature = "navigation"))]
pub use git_commit_vertical::*;
#[cfg(feature = "development")]
pub use git_compare::*;
#[cfg(any(feature = "development", feature = "arrows"))]
pub use git_compare_arrows::*;
#[cfg(feature = "development")]
pub use git_fork::*;
#[cfg(feature = "development")]
pub use git_graph::*;
#[cfg(feature = "development")]
pub use git_merge::*;
#[cfg(feature = "development")]
pub use git_pull_request::*;
#[cfg(any(feature = "development", feature = "arrows"))]
pub use git_pull_request_arrow::*;
#[cfg(feature = "development")]
pub use git_pull_request_closed::*;
#[cfg(feature = "development")]
pub use git_pull_request_create::*;
#[cfg(any(feature = "development", feature = "arrows"))]
pub use git_pull_request_create_arrow::*;
#[cfg(feature = "development")]
pub use git_pull_request_draft::*;
#[cfg(any(feature = "brands", feature = "development"))]
pub use github::*;
#[cfg(any(feature = "brands", feature = "development"))]
pub use gitlab::*;
#[cfg(feature = "food-beverage")]
pub use glass_water::*;
#[cfg(feature = "accessibility")]
pub use glasses::*;
#[cfg(feature = "navigation")]
pub use globe::*;
#[cfg(any(feature = "security", feature = "development", feature = "devices"))]
pub use globe_lock::*;
#[cfg(feature = "gaming")]
pub use goal::*;
#[cfg(any(feature = "cursors", feature = "design", feature = "layout"))]
pub use grab::*;
#[cfg(feature = "buildings")]
pub use graduation_cap::*;
#[cfg(feature = "food-beverage")]
pub use grape::*;
#[cfg(any(
    feature = "text",
    feature = "layout",
    feature = "design",
    feature = "math"
))]
pub use grid_2_x_2::*;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
pub use grid_2_x_2_check::*;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
pub use grid_2_x_2_plus::*;
#[cfg(any(feature = "text", feature = "layout", feature = "math"))]
pub use grid_2_x_2_x::*;
#[cfg(any(feature = "text", feature = "layout", feature = "design"))]
pub use grid_3_x_3::*;
#[cfg(feature = "layout")]
pub use grip::*;
#[cfg(feature = "layout")]
pub use grip_horizontal::*;
#[cfg(feature = "layout")]
pub use grip_vertical::*;
#[cfg(feature = "files")]
pub use group::*;
#[cfg(feature = "multimedia")]
pub use guitar::*;
#[cfg(feature = "food-beverage")]
pub use ham::*;
#[cfg(feature = "food-beverage")]
pub use hamburger::*;
#[cfg(any(feature = "tools", feature = "home"))]
pub use hammer::*;
#[cfg(any(feature = "cursors", feature = "accessibility"))]
pub use hand::*;
#[cfg(any(feature = "finance", feature = "account"))]
pub use hand_coins::*;
#[cfg(feature = "social")]
pub use hand_heart::*;
#[cfg(feature = "emoji")]
pub use hand_helping::*;
#[cfg(any(feature = "emoji", feature = "multimedia"))]
pub use hand_metal::*;
#[cfg(any(feature = "food-beverage", feature = "people"))]
pub use hand_platter::*;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "communication",
    feature = "finance",
    feature = "security"
))]
pub use handshake::*;
#[cfg(any(feature = "development", feature = "devices"))]
pub use hard_drive::*;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "arrows",
    feature = "files"
))]
pub use hard_drive_download::*;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "arrows",
    feature = "files"
))]
pub use hard_drive_upload::*;
#[cfg(feature = "tools")]
pub use hard_hat::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use hash::*;
#[cfg(feature = "weather")]
pub use haze::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "gaming"))]
pub use hdmi_port::*;
#[cfg(feature = "text")]
pub use heading::*;
#[cfg(feature = "text")]
pub use heading_1::*;
#[cfg(feature = "text")]
pub use heading_2::*;
#[cfg(feature = "text")]
pub use heading_3::*;
#[cfg(feature = "text")]
pub use heading_4::*;
#[cfg(feature = "text")]
pub use heading_5::*;
#[cfg(feature = "text")]
pub use heading_6::*;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "communication",
    feature = "devices",
    feature = "gaming"
))]
pub use headphone_off::*;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "files",
    feature = "gaming"
))]
pub use headphones::*;
#[cfg(any(
    feature = "multimedia",
    feature = "connectivity",
    feature = "devices",
    feature = "files",
    feature = "gaming"
))]
pub use headset::*;
#[cfg(any(
    feature = "medical",
    feature = "social",
    feature = "multimedia",
    feature = "emoji",
    feature = "gaming",
    feature = "shapes"
))]
pub use heart::*;
#[cfg(feature = "emoji")]
pub use heart_crack::*;
#[cfg(any(feature = "emoji", feature = "account", feature = "security"))]
pub use heart_handshake::*;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "multimedia",
    feature = "gaming",
    feature = "social"
))]
pub use heart_minus::*;
#[cfg(any(feature = "social", feature = "multimedia"))]
pub use heart_off::*;
#[cfg(any(
    feature = "medical",
    feature = "account",
    feature = "multimedia",
    feature = "gaming",
    feature = "social"
))]
pub use heart_plus::*;
#[cfg(feature = "medical")]
pub use heart_pulse::*;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
pub use heater::*;
#[cfg(any(feature = "shapes", feature = "brands", feature = "development"))]
pub use hexagon::*;
#[cfg(any(feature = "text", feature = "design"))]
pub use highlighter::*;
#[cfg(any(feature = "arrows", feature = "time"))]
pub use history::*;
#[cfg(feature = "food-beverage")]
pub use hop::*;
#[cfg(feature = "food-beverage")]
pub use hop_off::*;
#[cfg(any(
    feature = "medical",
    feature = "buildings",
    feature = "navigation",
    feature = "travel"
))]
pub use hospital::*;
#[cfg(any(feature = "buildings", feature = "navigation", feature = "travel"))]
pub use hotel::*;
#[cfg(any(feature = "time", feature = "gaming"))]
pub use hourglass::*;
#[cfg(any(feature = "buildings", feature = "home"))]
pub use house::*;
#[cfg(any(feature = "buildings", feature = "home", feature = "sustainability"))]
pub use house_plug::*;
#[cfg(any(feature = "buildings", feature = "medical"))]
pub use house_plus::*;
#[cfg(any(feature = "home", feature = "buildings", feature = "connectivity"))]
pub use house_wifi::*;
#[cfg(feature = "food-beverage")]
pub use ice_cream_bowl::*;
#[cfg(feature = "food-beverage")]
pub use ice_cream_cone::*;
#[cfg(any(feature = "security", feature = "account"))]
pub use id_card::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
pub use image::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
pub use image_down::*;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
pub use image_minus::*;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
pub use image_off::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
pub use image_play::*;
#[cfg(any(feature = "photography", feature = "multimedia", feature = "files"))]
pub use image_plus::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
pub use image_up::*;
#[cfg(any(feature = "photography", feature = "multimedia"))]
pub use image_upscale::*;
#[cfg(any(
    feature = "photography",
    feature = "text",
    feature = "multimedia",
    feature = "files"
))]
pub use images::*;
#[cfg(any(feature = "arrows", feature = "files"))]
pub use import::*;
#[cfg(any(feature = "account", feature = "mail"))]
pub use inbox::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use indent_decrease::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use indent_increase::*;
#[cfg(feature = "finance")]
pub use indian_rupee::*;
#[cfg(feature = "multimedia")]
pub use infinity::*;
#[cfg(any(feature = "accessibility", feature = "notifications"))]
pub use info::*;
#[cfg(feature = "tools")]
pub use inspection_panel::*;
#[cfg(any(feature = "brands", feature = "social", feature = "photography"))]
pub use instagram::*;
#[cfg(feature = "text")]
pub use italic::*;
#[cfg(any(feature = "arrows", feature = "design"))]
pub use iteration_ccw::*;
#[cfg(any(feature = "arrows", feature = "design"))]
pub use iteration_cw::*;
#[cfg(feature = "finance")]
pub use japanese_yen::*;
#[cfg(any(feature = "gaming", feature = "devices"))]
pub use joystick::*;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
pub use kanban::*;
#[cfg(any(feature = "security", feature = "account"))]
pub use key::*;
#[cfg(any(feature = "security", feature = "account"))]
pub use key_round::*;
#[cfg(any(feature = "security", feature = "account"))]
pub use key_square::*;
#[cfg(any(feature = "text", feature = "devices", feature = "development"))]
pub use keyboard::*;
#[cfg(any(feature = "multimedia", feature = "devices"))]
pub use keyboard_music::*;
#[cfg(any(feature = "devices", feature = "text", feature = "development"))]
pub use keyboard_off::*;
#[cfg(feature = "home")]
pub use lamp::*;
#[cfg(feature = "home")]
pub use lamp_ceiling::*;
#[cfg(feature = "home")]
pub use lamp_desk::*;
#[cfg(feature = "home")]
pub use lamp_floor::*;
#[cfg(feature = "home")]
pub use lamp_wall_down::*;
#[cfg(feature = "home")]
pub use lamp_wall_up::*;
#[cfg(any(
    feature = "design",
    feature = "tools",
    feature = "math",
    feature = "sports",
    feature = "gaming"
))]
pub use land_plot::*;
#[cfg(any(feature = "finance", feature = "navigation", feature = "buildings"))]
pub use landmark::*;
#[cfg(feature = "text")]
pub use languages::*;
#[cfg(feature = "devices")]
pub use laptop::*;
#[cfg(feature = "devices")]
pub use laptop_minimal::*;
#[cfg(any(feature = "devices", feature = "notifications"))]
pub use laptop_minimal_check::*;
#[cfg(any(feature = "design", feature = "cursors"))]
pub use lasso::*;
#[cfg(any(feature = "arrows", feature = "design", feature = "cursors"))]
pub use lasso_select::*;
#[cfg(feature = "emoji")]
pub use laugh::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use layers::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use layers_2::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use layout_dashboard::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use layout_grid::*;
#[cfg(any(
    feature = "design",
    feature = "layout",
    feature = "photography",
    feature = "text"
))]
pub use layout_list::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use layout_panel_left::*;
#[cfg(feature = "layout")]
pub use layout_panel_top::*;
#[cfg(feature = "layout")]
pub use layout_template::*;
#[cfg(any(feature = "nature", feature = "sustainability", feature = "seasons"))]
pub use leaf::*;
#[cfg(any(
    feature = "food-beverage",
    feature = "emoji",
    feature = "sustainability"
))]
pub use leafy_green::*;
#[cfg(any(feature = "communication", feature = "multimedia"))]
pub use lectern::*;
#[cfg(feature = "text")]
pub use letter_text::*;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
pub use library::*;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
pub use library_big::*;
#[cfg(any(feature = "accessibility", feature = "medical"))]
pub use life_buoy::*;
#[cfg(feature = "text")]
pub use ligature::*;
#[cfg(feature = "photography")]
pub use lightbulb::*;
#[cfg(feature = "photography")]
pub use lightbulb_off::*;
#[cfg(any(feature = "text", feature = "account"))]
pub use link::*;
#[cfg(any(feature = "text", feature = "account"))]
pub use link_2::*;
#[cfg(feature = "text")]
pub use link_2_off::*;
#[cfg(any(feature = "social", feature = "brands"))]
pub use linkedin::*;
#[cfg(feature = "text")]
pub use list::*;
#[cfg(feature = "text")]
pub use list_check::*;
#[cfg(feature = "text")]
pub use list_checks::*;
#[cfg(feature = "text")]
pub use list_collapse::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_end::*;
#[cfg(feature = "text")]
pub use list_filter::*;
#[cfg(any(feature = "text", feature = "layout"))]
pub use list_filter_plus::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_minus::*;
#[cfg(feature = "multimedia")]
pub use list_music::*;
#[cfg(feature = "text")]
pub use list_ordered::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_plus::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_restart::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_start::*;
#[cfg(feature = "text")]
pub use list_todo::*;
#[cfg(any(feature = "files", feature = "text", feature = "layout"))]
pub use list_tree::*;
#[cfg(feature = "multimedia")]
pub use list_video::*;
#[cfg(any(feature = "multimedia", feature = "text"))]
pub use list_x::*;
#[cfg(any(
    feature = "cursors",
    feature = "multimedia",
    feature = "layout",
    feature = "design"
))]
pub use loader::*;
#[cfg(any(feature = "cursors", feature = "multimedia", feature = "layout"))]
pub use loader_circle::*;
#[cfg(any(feature = "cursors", feature = "design"))]
pub use loader_pinwheel::*;
#[cfg(feature = "navigation")]
pub use locate::*;
#[cfg(feature = "navigation")]
pub use locate_fixed::*;
#[cfg(feature = "navigation")]
pub use locate_off::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use location_edit::*;
#[cfg(feature = "security")]
pub use lock::*;
#[cfg(feature = "security")]
pub use lock_keyhole::*;
#[cfg(feature = "security")]
pub use lock_keyhole_open::*;
#[cfg(feature = "security")]
pub use lock_open::*;
#[cfg(any(feature = "arrows", feature = "account"))]
pub use log_in::*;
#[cfg(any(feature = "arrows", feature = "account"))]
pub use log_out::*;
#[cfg(feature = "text")]
pub use logs::*;
#[cfg(feature = "food-beverage")]
pub use lollipop::*;
#[cfg(any(feature = "travel", feature = "transportation"))]
pub use luggage::*;
#[cfg(feature = "design")]
pub use magnet::*;
#[cfg(any(feature = "text", feature = "account", feature = "mail"))]
pub use mail::*;
#[cfg(feature = "mail")]
pub use mail_check::*;
#[cfg(feature = "mail")]
pub use mail_minus::*;
#[cfg(feature = "mail")]
pub use mail_open::*;
#[cfg(feature = "mail")]
pub use mail_plus::*;
#[cfg(feature = "mail")]
pub use mail_question::*;
#[cfg(feature = "mail")]
pub use mail_search::*;
#[cfg(feature = "mail")]
pub use mail_warning::*;
#[cfg(feature = "mail")]
pub use mail_x::*;
#[cfg(feature = "mail")]
pub use mailbox::*;
#[cfg(feature = "mail")]
pub use mails::*;
#[cfg(any(feature = "text", feature = "navigation"))]
pub use map::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_check::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_check_inside::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_house::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_minus::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_minus_inside::*;
#[cfg(any(feature = "navigation", feature = "travel"))]
pub use map_pin_off::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_plus::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_plus_inside::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_x::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pin_x_inside::*;
#[cfg(any(feature = "navigation", feature = "travel", feature = "account"))]
pub use map_pinned::*;
#[cfg(feature = "navigation")]
pub use map_plus::*;
#[cfg(feature = "medical")]
pub use mars::*;
#[cfg(feature = "medical")]
pub use mars_stroke::*;
#[cfg(feature = "food-beverage")]
pub use martini::*;
#[cfg(any(feature = "layout", feature = "design"))]
pub use maximize::*;
#[cfg(any(feature = "arrows", feature = "layout", feature = "design"))]
pub use maximize_2::*;
#[cfg(any(feature = "sports", feature = "gaming"))]
pub use medal::*;
#[cfg(any(feature = "multimedia", feature = "notifications"))]
pub use megaphone::*;
#[cfg(any(feature = "multimedia", feature = "notifications"))]
pub use megaphone_off::*;
#[cfg(feature = "emoji")]
pub use meh::*;
#[cfg(any(feature = "devices", feature = "gaming"))]
pub use memory_stick::*;
#[cfg(any(feature = "layout", feature = "account"))]
pub use menu::*;
#[cfg(any(feature = "development", feature = "arrows"))]
pub use merge::*;
#[cfg(feature = "social")]
pub use message_circle::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use message_circle_code::*;
#[cfg(feature = "social")]
pub use message_circle_dashed::*;
#[cfg(feature = "social")]
pub use message_circle_heart::*;
#[cfg(feature = "social")]
pub use message_circle_more::*;
#[cfg(feature = "social")]
pub use message_circle_off::*;
#[cfg(feature = "social")]
pub use message_circle_plus::*;
#[cfg(feature = "social")]
pub use message_circle_question::*;
#[cfg(feature = "social")]
pub use message_circle_reply::*;
#[cfg(any(feature = "social", feature = "notifications"))]
pub use message_circle_warning::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use message_circle_x::*;
#[cfg(feature = "social")]
pub use message_square::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use message_square_code::*;
#[cfg(feature = "social")]
pub use message_square_dashed::*;
#[cfg(any(feature = "development", feature = "files", feature = "social"))]
pub use message_square_diff::*;
#[cfg(any(feature = "social", feature = "notifications"))]
pub use message_square_dot::*;
#[cfg(feature = "social")]
pub use message_square_heart::*;
#[cfg(feature = "social")]
pub use message_square_lock::*;
#[cfg(feature = "social")]
pub use message_square_more::*;
#[cfg(feature = "social")]
pub use message_square_off::*;
#[cfg(feature = "social")]
pub use message_square_plus::*;
#[cfg(any(feature = "social", feature = "text"))]
pub use message_square_quote::*;
#[cfg(feature = "social")]
pub use message_square_reply::*;
#[cfg(feature = "social")]
pub use message_square_share::*;
#[cfg(feature = "social")]
pub use message_square_text::*;
#[cfg(any(feature = "social", feature = "notifications"))]
pub use message_square_warning::*;
#[cfg(feature = "social")]
pub use message_square_x::*;
#[cfg(feature = "social")]
pub use messages_square::*;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "multimedia"
))]
pub use mic::*;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "multimedia"
))]
pub use mic_off::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use mic_vocal::*;
#[cfg(feature = "devices")]
pub use microchip::*;
#[cfg(any(feature = "science", feature = "medical"))]
pub use microscope::*;
#[cfg(any(feature = "food-beverage", feature = "home"))]
pub use microwave::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
pub use milestone::*;
#[cfg(feature = "food-beverage")]
pub use milk::*;
#[cfg(feature = "food-beverage")]
pub use milk_off::*;
#[cfg(any(feature = "layout", feature = "design"))]
pub use minimize::*;
#[cfg(any(feature = "arrows", feature = "layout", feature = "design"))]
pub use minimize_2::*;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "tools"
))]
pub use minus::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_check::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_cog::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_dot::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_down::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_off::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
pub use monitor_pause::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
pub use monitor_play::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_smartphone::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_speaker::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
pub use monitor_stop::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_up::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use monitor_x::*;
#[cfg(feature = "accessibility")]
pub use moon::*;
#[cfg(any(feature = "accessibility", feature = "weather"))]
pub use moon_star::*;
#[cfg(any(feature = "nature", feature = "gaming"))]
pub use mountain::*;
#[cfg(feature = "nature")]
pub use mountain_snow::*;
#[cfg(feature = "devices")]
pub use mouse::*;
#[cfg(feature = "devices")]
pub use mouse_off::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use mouse_pointer::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use mouse_pointer_2::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use mouse_pointer_ban::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use mouse_pointer_click::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use r#move::*;
#[cfg(feature = "design")]
pub use move_3_d::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use move_diagonal::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use move_diagonal_2::*;
#[cfg(feature = "arrows")]
pub use move_down::*;
#[cfg(feature = "arrows")]
pub use move_down_left::*;
#[cfg(feature = "arrows")]
pub use move_down_right::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use move_horizontal::*;
#[cfg(feature = "arrows")]
pub use move_left::*;
#[cfg(feature = "arrows")]
pub use move_right::*;
#[cfg(feature = "arrows")]
pub use move_up::*;
#[cfg(feature = "arrows")]
pub use move_up_left::*;
#[cfg(feature = "arrows")]
pub use move_up_right::*;
#[cfg(any(feature = "arrows", feature = "cursors"))]
pub use move_vertical::*;
#[cfg(any(feature = "multimedia", feature = "files"))]
pub use music::*;
#[cfg(any(feature = "multimedia", feature = "files"))]
pub use music_2::*;
#[cfg(any(feature = "multimedia", feature = "files"))]
pub use music_3::*;
#[cfg(any(feature = "multimedia", feature = "files"))]
pub use music_4::*;
#[cfg(feature = "navigation")]
pub use navigation::*;
#[cfg(feature = "navigation")]
pub use navigation_2::*;
#[cfg(feature = "navigation")]
pub use navigation_2_off::*;
#[cfg(feature = "navigation")]
pub use navigation_off::*;
#[cfg(feature = "development")]
pub use network::*;
#[cfg(any(feature = "multimedia", feature = "communication"))]
pub use newspaper::*;
#[cfg(any(feature = "communication", feature = "finance", feature = "devices"))]
pub use nfc::*;
#[cfg(feature = "medical")]
pub use non_binary::*;
#[cfg(any(
    feature = "text",
    feature = "communication",
    feature = "social",
    feature = "design"
))]
pub use notebook::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use notebook_pen::*;
#[cfg(any(feature = "account", feature = "communication", feature = "social"))]
pub use notebook_tabs::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use notebook_text::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use notepad_text::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use notepad_text_dashed::*;
#[cfg(feature = "food-beverage")]
pub use nut::*;
#[cfg(feature = "food-beverage")]
pub use nut_off::*;
#[cfg(feature = "shapes")]
pub use octagon::*;
#[cfg(any(feature = "notifications", feature = "shapes"))]
pub use octagon_alert::*;
#[cfg(feature = "transportation")]
pub use octagon_minus::*;
#[cfg(any(feature = "multimedia", feature = "shapes"))]
pub use octagon_pause::*;
#[cfg(any(feature = "math", feature = "notifications"))]
pub use octagon_x::*;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "science"
))]
pub use omega::*;
#[cfg(feature = "development")]
pub use option::*;
#[cfg(feature = "science")]
pub use orbit::*;
#[cfg(any(feature = "animals", feature = "design"))]
pub use origami::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use package::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use package_2::*;
#[cfg(feature = "development")]
pub use package_check::*;
#[cfg(feature = "development")]
pub use package_minus::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use package_open::*;
#[cfg(feature = "development")]
pub use package_plus::*;
#[cfg(any(feature = "files", feature = "development"))]
pub use package_search::*;
#[cfg(feature = "development")]
pub use package_x::*;
#[cfg(any(feature = "design", feature = "tools"))]
pub use paint_bucket::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "home",
    feature = "tools"
))]
pub use paint_roller::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "photography",
    feature = "home",
    feature = "tools"
))]
pub use paintbrush::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "photography",
    feature = "home",
    feature = "tools"
))]
pub use paintbrush_vertical::*;
#[cfg(any(feature = "text", feature = "design", feature = "photography"))]
pub use palette::*;
#[cfg(feature = "animals")]
pub use panda::*;
#[cfg(feature = "layout")]
pub use panel_bottom::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_bottom_close::*;
#[cfg(feature = "layout")]
pub use panel_bottom_dashed::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_bottom_open::*;
#[cfg(feature = "layout")]
pub use panel_left::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_left_close::*;
#[cfg(feature = "layout")]
pub use panel_left_dashed::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_left_open::*;
#[cfg(feature = "layout")]
pub use panel_right::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_right_close::*;
#[cfg(feature = "layout")]
pub use panel_right_dashed::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_right_open::*;
#[cfg(any(feature = "layout", feature = "design", feature = "development"))]
pub use panel_top::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_top_close::*;
#[cfg(feature = "layout")]
pub use panel_top_dashed::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use panel_top_open::*;
#[cfg(feature = "layout")]
pub use panels_left_bottom::*;
#[cfg(feature = "layout")]
pub use panels_right_bottom::*;
#[cfg(any(feature = "layout", feature = "design", feature = "development"))]
pub use panels_top_left::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "files",
    feature = "mail"
))]
pub use paperclip::*;
#[cfg(any(feature = "development", feature = "files", feature = "math"))]
pub use parentheses::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use parking_meter::*;
#[cfg(feature = "emoji")]
pub use party_popper::*;
#[cfg(feature = "multimedia")]
pub use pause::*;
#[cfg(feature = "animals")]
pub use paw_print::*;
#[cfg(any(feature = "devices", feature = "gaming"))]
pub use pc_case::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use pen::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use pen_line::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use pen_off::*;
#[cfg(any(feature = "text", feature = "design", feature = "cursors"))]
pub use pen_tool::*;
#[cfg(any(
    feature = "design",
    feature = "cursors",
    feature = "tools",
    feature = "text"
))]
pub use pencil::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use pencil_line::*;
#[cfg(any(
    feature = "design",
    feature = "cursors",
    feature = "tools",
    feature = "text"
))]
pub use pencil_off::*;
#[cfg(any(
    feature = "tools",
    feature = "design",
    feature = "layout",
    feature = "text"
))]
pub use pencil_ruler::*;
#[cfg(feature = "shapes")]
pub use pentagon::*;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "finance",
    feature = "shopping"
))]
pub use percent::*;
#[cfg(any(feature = "accessibility", feature = "people"))]
pub use person_standing::*;
#[cfg(feature = "finance")]
pub use philippine_peso::*;
#[cfg(any(
    feature = "text",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_call::*;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_forwarded::*;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_incoming::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_missed::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_off::*;
#[cfg(any(
    feature = "arrows",
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use phone_outgoing::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use pi::*;
#[cfg(any(feature = "multimedia", feature = "devices"))]
pub use piano::*;
#[cfg(any(feature = "tools", feature = "gaming"))]
pub use pickaxe::*;
#[cfg(feature = "multimedia")]
pub use picture_in_picture::*;
#[cfg(feature = "multimedia")]
pub use picture_in_picture_2::*;
#[cfg(feature = "finance")]
pub use piggy_bank::*;
#[cfg(feature = "text")]
pub use pilcrow::*;
#[cfg(feature = "text")]
pub use pilcrow_left::*;
#[cfg(feature = "text")]
pub use pilcrow_right::*;
#[cfg(feature = "medical")]
pub use pill::*;
#[cfg(feature = "medical")]
pub use pill_bottle::*;
#[cfg(any(feature = "navigation", feature = "account"))]
pub use pin::*;
#[cfg(feature = "navigation")]
pub use pin_off::*;
#[cfg(any(feature = "text", feature = "design", feature = "science"))]
pub use pipette::*;
#[cfg(feature = "food-beverage")]
pub use pizza::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use plane::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use plane_landing::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use plane_takeoff::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use play::*;
#[cfg(any(feature = "devices", feature = "development"))]
pub use plug::*;
#[cfg(any(feature = "devices", feature = "development"))]
pub use plug_2::*;
#[cfg(feature = "devices")]
pub use plug_zap::*;
#[cfg(any(
    feature = "math",
    feature = "tools",
    feature = "development",
    feature = "text",
    feature = "cursors",
    feature = "gaming"
))]
pub use plus::*;
#[cfg(feature = "brands")]
pub use pocket::*;
#[cfg(feature = "tools")]
pub use pocket_knife::*;
#[cfg(any(feature = "multimedia", feature = "social"))]
pub use podcast::*;
#[cfg(feature = "cursors")]
pub use pointer::*;
#[cfg(feature = "cursors")]
pub use pointer_off::*;
#[cfg(any(feature = "food-beverage", feature = "multimedia"))]
pub use popcorn::*;
#[cfg(feature = "food-beverage")]
pub use popsicle::*;
#[cfg(feature = "finance")]
pub use pound_sterling::*;
#[cfg(feature = "connectivity")]
pub use power::*;
#[cfg(feature = "connectivity")]
pub use power_off::*;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "devices",
    feature = "communication",
    feature = "design"
))]
pub use presentation::*;
#[cfg(feature = "devices")]
pub use printer::*;
#[cfg(feature = "devices")]
pub use printer_check::*;
#[cfg(any(
    feature = "multimedia",
    feature = "photography",
    feature = "devices",
    feature = "communication"
))]
pub use projector::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "devices"
))]
pub use proportions::*;
#[cfg(any(feature = "development", feature = "gaming"))]
pub use puzzle::*;
#[cfg(any(feature = "shapes", feature = "math", feature = "travel"))]
pub use pyramid::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use qr_code::*;
#[cfg(feature = "text")]
pub use quote::*;
#[cfg(feature = "animals")]
pub use rabbit::*;
#[cfg(any(
    feature = "navigation",
    feature = "security",
    feature = "communication"
))]
pub use radar::*;
#[cfg(feature = "science")]
pub use radiation::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use radical::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
pub use radio::*;
#[cfg(feature = "devices")]
pub use radio_receiver::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "social"))]
pub use radio_tower::*;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
pub use radius::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use rail_symbol::*;
#[cfg(feature = "weather")]
pub use rainbow::*;
#[cfg(feature = "animals")]
pub use rat::*;
#[cfg(any(feature = "layout", feature = "design", feature = "photography"))]
pub use ratio::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_cent::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_euro::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_indian_rupee::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_japanese_yen::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_pound_sterling::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_russian_ruble::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_swiss_franc::*;
#[cfg(any(feature = "finance", feature = "travel"))]
pub use receipt_text::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use rectangle_ellipsis::*;
#[cfg(any(
    feature = "devices",
    feature = "gaming",
    feature = "multimedia",
    feature = "connectivity"
))]
pub use rectangle_goggles::*;
#[cfg(any(feature = "shapes", feature = "design"))]
pub use rectangle_horizontal::*;
#[cfg(any(feature = "shapes", feature = "design"))]
pub use rectangle_vertical::*;
#[cfg(feature = "sustainability")]
pub use recycle::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use redo::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use redo_2::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use redo_dot::*;
#[cfg(feature = "arrows")]
pub use refresh_ccw::*;
#[cfg(any(feature = "arrows", feature = "development"))]
pub use refresh_ccw_dot::*;
#[cfg(feature = "arrows")]
pub use refresh_cw::*;
#[cfg(feature = "arrows")]
pub use refresh_cw_off::*;
#[cfg(any(feature = "food-beverage", feature = "home"))]
pub use refrigerator::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use regex::*;
#[cfg(feature = "text")]
pub use remove_formatting::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use repeat::*;
#[cfg(feature = "multimedia")]
pub use repeat_1::*;
#[cfg(any(feature = "arrows", feature = "social", feature = "multimedia"))]
pub use repeat_2::*;
#[cfg(feature = "text")]
pub use replace::*;
#[cfg(feature = "text")]
pub use replace_all::*;
#[cfg(feature = "mail")]
pub use reply::*;
#[cfg(feature = "mail")]
pub use reply_all::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use rewind::*;
#[cfg(any(feature = "social", feature = "medical", feature = "emoji"))]
pub use ribbon::*;
#[cfg(any(feature = "gaming", feature = "development"))]
pub use rocket::*;
#[cfg(feature = "home")]
pub use rocking_chair::*;
#[cfg(feature = "navigation")]
pub use roller_coaster::*;
#[cfg(feature = "design")]
pub use rotate_3_d::*;
#[cfg(any(feature = "arrows", feature = "design", feature = "photography"))]
pub use rotate_ccw::*;
#[cfg(any(feature = "security", feature = "account"))]
pub use rotate_ccw_key::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "arrows"
))]
pub use rotate_ccw_square::*;
#[cfg(any(feature = "arrows", feature = "design", feature = "photography"))]
pub use rotate_cw::*;
#[cfg(any(
    feature = "layout",
    feature = "design",
    feature = "photography",
    feature = "tools",
    feature = "arrows"
))]
pub use rotate_cw_square::*;
#[cfg(feature = "navigation")]
pub use route::*;
#[cfg(feature = "navigation")]
pub use route_off::*;
#[cfg(any(
    feature = "development",
    feature = "devices",
    feature = "connectivity",
    feature = "home"
))]
pub use router::*;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
pub use rows_2::*;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
pub use rows_3::*;
#[cfg(any(feature = "layout", feature = "design", feature = "text"))]
pub use rows_4::*;
#[cfg(any(feature = "development", feature = "social"))]
pub use rss::*;
#[cfg(any(feature = "tools", feature = "design", feature = "layout"))]
pub use ruler::*;
#[cfg(any(feature = "tools", feature = "design", feature = "layout"))]
pub use ruler_dimension_line::*;
#[cfg(feature = "finance")]
pub use russian_ruble::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use sailboat::*;
#[cfg(any(feature = "food-beverage", feature = "emoji"))]
pub use salad::*;
#[cfg(feature = "food-beverage")]
pub use sandwich::*;
#[cfg(any(feature = "connectivity", feature = "science"))]
pub use satellite::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "multimedia"))]
pub use satellite_dish::*;
#[cfg(feature = "finance")]
pub use saudi_riyal::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use save::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use save_all::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use save_off::*;
#[cfg(feature = "navigation")]
pub use scale::*;
#[cfg(feature = "design")]
pub use scale_3_d::*;
#[cfg(feature = "design")]
pub use scaling::*;
#[cfg(any(
    feature = "devices",
    feature = "shopping",
    feature = "security",
    feature = "social",
    feature = "gaming"
))]
pub use scan::*;
#[cfg(any(feature = "shopping", feature = "devices"))]
pub use scan_barcode::*;
#[cfg(any(
    feature = "photography",
    feature = "multimedia",
    feature = "accessibility",
    feature = "security",
    feature = "devices",
    feature = "account"
))]
pub use scan_eye::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "devices",
    feature = "social"
))]
pub use scan_face::*;
#[cfg(feature = "medical")]
pub use scan_heart::*;
#[cfg(any(feature = "devices", feature = "shopping"))]
pub use scan_line::*;
#[cfg(any(
    feature = "account",
    feature = "shopping",
    feature = "devices",
    feature = "security"
))]
pub use scan_qr_code::*;
#[cfg(any(
    feature = "photography",
    feature = "multimedia",
    feature = "accessibility"
))]
pub use scan_search::*;
#[cfg(any(feature = "text", feature = "devices"))]
pub use scan_text::*;
#[cfg(any(feature = "buildings", feature = "navigation"))]
pub use school::*;
#[cfg(any(feature = "text", feature = "design", feature = "tools"))]
pub use scissors::*;
#[cfg(any(feature = "design", feature = "tools"))]
pub use scissors_line_dashed::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use screen_share::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use screen_share_off::*;
#[cfg(any(feature = "gaming", feature = "development", feature = "text"))]
pub use scroll::*;
#[cfg(any(feature = "gaming", feature = "development", feature = "text"))]
pub use scroll_text::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use search::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use search_check::*;
#[cfg(any(feature = "text", feature = "social", feature = "development"))]
pub use search_code::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use search_slash::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use search_x::*;
#[cfg(feature = "text")]
pub use section::*;
#[cfg(any(feature = "mail", feature = "communication", feature = "connectivity"))]
pub use send::*;
#[cfg(any(feature = "mail", feature = "communication", feature = "connectivity"))]
pub use send_horizontal::*;
#[cfg(any(feature = "design", feature = "layout"))]
pub use send_to_back::*;
#[cfg(any(feature = "text", feature = "arrows", feature = "layout"))]
pub use separator_horizontal::*;
#[cfg(any(feature = "text", feature = "arrows", feature = "layout"))]
pub use separator_vertical::*;
#[cfg(any(feature = "development", feature = "devices"))]
pub use server::*;
#[cfg(any(feature = "development", feature = "devices"))]
pub use server_cog::*;
#[cfg(any(feature = "development", feature = "devices"))]
pub use server_crash::*;
#[cfg(any(feature = "development", feature = "devices"))]
pub use server_off::*;
#[cfg(feature = "account")]
pub use settings::*;
#[cfg(feature = "account")]
pub use settings_2::*;
#[cfg(any(feature = "shapes", feature = "gaming"))]
pub use shapes::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use share::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use share_2::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use sheet::*;
#[cfg(any(
    feature = "animals",
    feature = "development",
    feature = "nature",
    feature = "science",
    feature = "travel",
    feature = "food-beverage",
    feature = "home"
))]
pub use shell::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming",
    feature = "shapes"
))]
pub use shield::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "notifications",
    feature = "gaming"
))]
pub use shield_alert::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_ban::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_check::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_ellipsis::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_half::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_minus::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_off::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming",
    feature = "medical"
))]
pub use shield_plus::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_question::*;
#[cfg(any(feature = "account", feature = "security", feature = "development"))]
pub use shield_user::*;
#[cfg(any(
    feature = "account",
    feature = "security",
    feature = "development",
    feature = "gaming"
))]
pub use shield_x::*;
#[cfg(any(feature = "transportation", feature = "navigation", feature = "travel"))]
pub use ship::*;
#[cfg(any(feature = "transportation", feature = "navigation", feature = "travel"))]
pub use ship_wheel::*;
#[cfg(feature = "shopping")]
pub use shirt::*;
#[cfg(feature = "shopping")]
pub use shopping_bag::*;
#[cfg(feature = "shopping")]
pub use shopping_basket::*;
#[cfg(feature = "shopping")]
pub use shopping_cart::*;
#[cfg(any(feature = "nature", feature = "tools", feature = "gaming"))]
pub use shovel::*;
#[cfg(any(feature = "home", feature = "travel"))]
pub use shower_head::*;
#[cfg(any(feature = "mail", feature = "files"))]
pub use shredder::*;
#[cfg(feature = "animals")]
pub use shrimp::*;
#[cfg(any(feature = "layout", feature = "arrows"))]
pub use shrink::*;
#[cfg(feature = "nature")]
pub use shrub::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use shuffle::*;
#[cfg(any(feature = "text", feature = "math", feature = "science"))]
pub use sigma::*;
#[cfg(feature = "connectivity")]
pub use signal::*;
#[cfg(feature = "connectivity")]
pub use signal_high::*;
#[cfg(feature = "connectivity")]
pub use signal_low::*;
#[cfg(feature = "connectivity")]
pub use signal_medium::*;
#[cfg(feature = "connectivity")]
pub use signal_zero::*;
#[cfg(feature = "text")]
pub use signature::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
pub use signpost::*;
#[cfg(any(
    feature = "arrows",
    feature = "navigation",
    feature = "development",
    feature = "gaming"
))]
pub use signpost_big::*;
#[cfg(feature = "medical")]
pub use siren::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use skip_back::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use skip_forward::*;
#[cfg(feature = "gaming")]
pub use skull::*;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "brands",
    feature = "development"
))]
pub use slack::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use slash::*;
#[cfg(feature = "design")]
pub use slice::*;
#[cfg(feature = "account")]
pub use sliders_horizontal::*;
#[cfg(feature = "account")]
pub use sliders_vertical::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use smartphone::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use smartphone_charging::*;
#[cfg(any(feature = "communication", feature = "finance", feature = "devices"))]
pub use smartphone_nfc::*;
#[cfg(any(feature = "emoji", feature = "account"))]
pub use smile::*;
#[cfg(any(
    feature = "emoji",
    feature = "social",
    feature = "notifications",
    feature = "communication"
))]
pub use smile_plus::*;
#[cfg(any(feature = "animals", feature = "food-beverage"))]
pub use snail::*;
#[cfg(any(feature = "weather", feature = "seasons"))]
pub use snowflake::*;
#[cfg(any(feature = "home", feature = "travel"))]
pub use soap_dispenser_droplet::*;
#[cfg(feature = "home")]
pub use sofa::*;
#[cfg(feature = "food-beverage")]
pub use soup::*;
#[cfg(feature = "text")]
pub use space::*;
#[cfg(any(feature = "shapes", feature = "gaming"))]
pub use spade::*;
#[cfg(feature = "shapes")]
pub use sparkle::*;
#[cfg(any(
    feature = "cursors",
    feature = "multimedia",
    feature = "gaming",
    feature = "weather"
))]
pub use sparkles::*;
#[cfg(any(feature = "multimedia", feature = "devices"))]
pub use speaker::*;
#[cfg(any(feature = "accessibility", feature = "communication"))]
pub use speech::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use spell_check::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use spell_check_2::*;
#[cfg(feature = "design")]
pub use spline::*;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "design",
    feature = "tools"
))]
pub use spline_pointer::*;
#[cfg(any(feature = "development", feature = "arrows"))]
pub use split::*;
#[cfg(any(feature = "design", feature = "tools"))]
pub use spray_can::*;
#[cfg(any(feature = "nature", feature = "gaming", feature = "sustainability"))]
pub use sprout::*;
#[cfg(feature = "shapes")]
pub use square::*;
#[cfg(any(
    feature = "medical",
    feature = "social",
    feature = "science",
    feature = "multimedia"
))]
pub use square_activity::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use square_arrow_down::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use square_arrow_down_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "gaming"))]
pub use square_arrow_down_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_out_down_left::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_out_down_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_out_up_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "social"))]
pub use square_arrow_out_up_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_right::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_up::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_arrow_up_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "social"))]
pub use square_arrow_up_right::*;
#[cfg(any(
    feature = "text",
    feature = "security",
    feature = "math",
    feature = "development"
))]
pub use square_asterisk::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "tools",
    feature = "files",
    feature = "development"
))]
pub use square_bottom_dashed_scissors::*;
#[cfg(any(
    feature = "charts",
    feature = "time",
    feature = "development",
    feature = "design"
))]
pub use square_chart_gantt::*;
#[cfg(feature = "notifications")]
pub use square_check::*;
#[cfg(feature = "notifications")]
pub use square_check_big::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_chevron_down::*;
#[cfg(any(feature = "arrows", feature = "navigation"))]
pub use square_chevron_left::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "development"))]
pub use square_chevron_right::*;
#[cfg(any(feature = "arrows", feature = "navigation", feature = "math"))]
pub use square_chevron_up::*;
#[cfg(any(feature = "text", feature = "development"))]
pub use square_code::*;
#[cfg(any(feature = "text", feature = "design"))]
pub use square_dashed::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use square_dashed_bottom::*;
#[cfg(any(feature = "development", feature = "files"))]
pub use square_dashed_bottom_code::*;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
pub use square_dashed_kanban::*;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "development",
    feature = "tools"
))]
pub use square_dashed_mouse_pointer::*;
#[cfg(feature = "math")]
pub use square_divide::*;
#[cfg(feature = "development")]
pub use square_dot::*;
#[cfg(feature = "math")]
pub use square_equal::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use square_function::*;
#[cfg(any(feature = "charts", feature = "development", feature = "design"))]
pub use square_kanban::*;
#[cfg(any(
    feature = "text",
    feature = "photography",
    feature = "multimedia",
    feature = "navigation",
    feature = "development"
))]
pub use square_library::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use square_m::*;
#[cfg(feature = "layout")]
pub use square_menu::*;
#[cfg(any(
    feature = "math",
    feature = "development",
    feature = "text",
    feature = "tools",
    feature = "devices"
))]
pub use square_minus::*;
#[cfg(any(
    feature = "arrows",
    feature = "cursors",
    feature = "development",
    feature = "tools"
))]
pub use square_mouse_pointer::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use square_parking::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use square_parking_off::*;
#[cfg(feature = "text")]
pub use square_pen::*;
#[cfg(any(
    feature = "social",
    feature = "finance",
    feature = "shopping",
    feature = "math"
))]
pub use square_percent::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use square_pi::*;
#[cfg(feature = "text")]
pub use square_pilcrow::*;
#[cfg(any(feature = "arrows", feature = "multimedia"))]
pub use square_play::*;
#[cfg(any(
    feature = "math",
    feature = "tools",
    feature = "development",
    feature = "text"
))]
pub use square_plus::*;
#[cfg(feature = "connectivity")]
pub use square_power::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use square_radical::*;
#[cfg(any(feature = "design", feature = "development", feature = "layout"))]
pub use square_round_corner::*;
#[cfg(any(
    feature = "text",
    feature = "design",
    feature = "tools",
    feature = "files",
    feature = "development"
))]
pub use square_scissors::*;
#[cfg(any(feature = "text", feature = "math"))]
pub use square_sigma::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use square_slash::*;
#[cfg(feature = "layout")]
pub use square_split_horizontal::*;
#[cfg(feature = "layout")]
pub use square_split_vertical::*;
#[cfg(feature = "layout")]
pub use square_square::*;
#[cfg(any(feature = "text", feature = "files", feature = "development"))]
pub use square_stack::*;
#[cfg(feature = "development")]
pub use square_terminal::*;
#[cfg(feature = "account")]
pub use square_user::*;
#[cfg(feature = "account")]
pub use square_user_round::*;
#[cfg(any(feature = "math", feature = "notifications"))]
pub use square_x::*;
#[cfg(feature = "design")]
pub use squares_exclude::*;
#[cfg(feature = "design")]
pub use squares_intersect::*;
#[cfg(feature = "design")]
pub use squares_subtract::*;
#[cfg(feature = "design")]
pub use squares_unite::*;
#[cfg(feature = "shapes")]
pub use squircle::*;
#[cfg(feature = "animals")]
pub use squirrel::*;
#[cfg(any(feature = "design", feature = "cursors", feature = "tools"))]
pub use stamp::*;
#[cfg(any(
    feature = "account",
    feature = "social",
    feature = "shapes",
    feature = "multimedia",
    feature = "weather",
    feature = "emoji",
    feature = "gaming"
))]
pub use star::*;
#[cfg(any(feature = "social", feature = "multimedia"))]
pub use star_half::*;
#[cfg(any(feature = "multimedia", feature = "social"))]
pub use star_off::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use step_back::*;
#[cfg(any(feature = "multimedia", feature = "arrows"))]
pub use step_forward::*;
#[cfg(any(feature = "science", feature = "medical"))]
pub use stethoscope::*;
#[cfg(feature = "social")]
pub use sticker::*;
#[cfg(any(feature = "text", feature = "social"))]
pub use sticky_note::*;
#[cfg(any(feature = "buildings", feature = "navigation", feature = "shopping"))]
pub use store::*;
#[cfg(feature = "layout")]
pub use stretch_horizontal::*;
#[cfg(feature = "layout")]
pub use stretch_vertical::*;
#[cfg(feature = "text")]
pub use strikethrough::*;
#[cfg(feature = "text")]
pub use subscript::*;
#[cfg(any(
    feature = "accessibility",
    feature = "weather",
    feature = "seasons",
    feature = "sustainability"
))]
pub use sun::*;
#[cfg(any(feature = "accessibility", feature = "weather"))]
pub use sun_dim::*;
#[cfg(any(feature = "accessibility", feature = "weather"))]
pub use sun_medium::*;
#[cfg(feature = "accessibility")]
pub use sun_moon::*;
#[cfg(feature = "weather")]
pub use sun_snow::*;
#[cfg(any(feature = "arrows", feature = "weather", feature = "time"))]
pub use sunrise::*;
#[cfg(any(feature = "arrows", feature = "weather"))]
pub use sunset::*;
#[cfg(feature = "text")]
pub use superscript::*;
#[cfg(any(feature = "design", feature = "home", feature = "photography"))]
pub use swatch_book::*;
#[cfg(feature = "finance")]
pub use swiss_franc::*;
#[cfg(any(feature = "communication", feature = "devices"))]
pub use switch_camera::*;
#[cfg(any(feature = "gaming", feature = "tools"))]
pub use sword::*;
#[cfg(any(feature = "gaming", feature = "tools"))]
pub use swords::*;
#[cfg(any(feature = "science", feature = "medical"))]
pub use syringe::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table_2::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table_cells_merge::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table_cells_split::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table_columns_split::*;
#[cfg(feature = "text")]
pub use table_of_contents::*;
#[cfg(any(feature = "text", feature = "development", feature = "files"))]
pub use table_properties::*;
#[cfg(any(feature = "text", feature = "files"))]
pub use table_rows_split::*;
#[cfg(feature = "devices")]
pub use tablet::*;
#[cfg(any(
    feature = "devices",
    feature = "design",
    feature = "development",
    feature = "tools"
))]
pub use tablet_smartphone::*;
#[cfg(feature = "medical")]
pub use tablets::*;
#[cfg(feature = "account")]
pub use tag::*;
#[cfg(feature = "account")]
pub use tags::*;
#[cfg(any(feature = "math", feature = "gaming"))]
pub use tally_1::*;
#[cfg(any(feature = "math", feature = "gaming"))]
pub use tally_2::*;
#[cfg(any(feature = "math", feature = "gaming"))]
pub use tally_3::*;
#[cfg(any(feature = "math", feature = "gaming"))]
pub use tally_4::*;
#[cfg(any(feature = "math", feature = "gaming"))]
pub use tally_5::*;
#[cfg(any(
    feature = "shapes",
    feature = "math",
    feature = "design",
    feature = "tools"
))]
pub use tangent::*;
#[cfg(any(feature = "brands", feature = "gaming"))]
pub use target::*;
#[cfg(any(feature = "science", feature = "development", feature = "tools"))]
pub use telescope::*;
#[cfg(any(feature = "travel", feature = "nature", feature = "sustainability"))]
pub use tent::*;
#[cfg(any(feature = "travel", feature = "nature"))]
pub use tent_tree::*;
#[cfg(feature = "development")]
pub use terminal::*;
#[cfg(feature = "science")]
pub use test_tube::*;
#[cfg(feature = "science")]
pub use test_tube_diagonal::*;
#[cfg(feature = "science")]
pub use test_tubes::*;
#[cfg(any(feature = "text", feature = "files", feature = "cursors"))]
pub use text::*;
#[cfg(any(feature = "text", feature = "cursors"))]
pub use text_cursor::*;
#[cfg(any(feature = "text", feature = "layout"))]
pub use text_cursor_input::*;
#[cfg(feature = "text")]
pub use text_quote::*;
#[cfg(feature = "text")]
pub use text_search::*;
#[cfg(any(feature = "text", feature = "cursors"))]
pub use text_select::*;
#[cfg(any(feature = "buildings", feature = "social"))]
pub use theater::*;
#[cfg(feature = "weather")]
pub use thermometer::*;
#[cfg(feature = "weather")]
pub use thermometer_snowflake::*;
#[cfg(feature = "weather")]
pub use thermometer_sun::*;
#[cfg(any(feature = "account", feature = "social", feature = "emoji"))]
pub use thumbs_down::*;
#[cfg(any(feature = "account", feature = "social", feature = "emoji"))]
pub use thumbs_up::*;
#[cfg(any(feature = "account", feature = "transportation"))]
pub use ticket::*;
#[cfg(feature = "transportation")]
pub use ticket_check::*;
#[cfg(feature = "transportation")]
pub use ticket_minus::*;
#[cfg(any(feature = "transportation", feature = "shopping"))]
pub use ticket_percent::*;
#[cfg(feature = "transportation")]
pub use ticket_plus::*;
#[cfg(feature = "transportation")]
pub use ticket_slash::*;
#[cfg(feature = "transportation")]
pub use ticket_x::*;
#[cfg(any(feature = "travel", feature = "account", feature = "transportation"))]
pub use tickets::*;
#[cfg(any(feature = "transportation", feature = "travel"))]
pub use tickets_plane::*;
#[cfg(feature = "time")]
pub use timer::*;
#[cfg(feature = "time")]
pub use timer_off::*;
#[cfg(feature = "time")]
pub use timer_reset::*;
#[cfg(any(feature = "layout", feature = "account", feature = "development"))]
pub use toggle_left::*;
#[cfg(any(feature = "layout", feature = "account", feature = "development"))]
pub use toggle_right::*;
#[cfg(any(feature = "devices", feature = "home"))]
pub use toilet::*;
#[cfg(feature = "weather")]
pub use tornado::*;
#[cfg(any(
    feature = "shapes",
    feature = "design",
    feature = "tools",
    feature = "food-beverage"
))]
pub use torus::*;
#[cfg(feature = "devices")]
pub use touchpad::*;
#[cfg(feature = "devices")]
pub use touchpad_off::*;
#[cfg(any(feature = "travel", feature = "transportation"))]
pub use tower_control::*;
#[cfg(any(feature = "gaming", feature = "development"))]
pub use toy_brick::*;
#[cfg(any(
    feature = "transportation",
    feature = "sustainability",
    feature = "food-beverage"
))]
pub use tractor::*;
#[cfg(feature = "transportation")]
pub use traffic_cone::*;
#[cfg(feature = "transportation")]
pub use train_front::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use train_front_tunnel::*;
#[cfg(any(feature = "transportation", feature = "navigation"))]
pub use train_track::*;
#[cfg(feature = "transportation")]
pub use tram_front::*;
#[cfg(any(feature = "medical", feature = "accessibility"))]
pub use transgender::*;
#[cfg(any(feature = "files", feature = "mail"))]
pub use trash::*;
#[cfg(any(feature = "files", feature = "mail"))]
pub use trash_2::*;
#[cfg(any(feature = "nature", feature = "sustainability"))]
pub use tree_deciduous::*;
#[cfg(any(feature = "nature", feature = "sustainability"))]
pub use tree_palm::*;
#[cfg(any(feature = "nature", feature = "sustainability"))]
pub use tree_pine::*;
#[cfg(any(feature = "nature", feature = "sustainability"))]
pub use trees::*;
#[cfg(any(feature = "account", feature = "brands", feature = "development"))]
pub use trello::*;
#[cfg(any(feature = "charts", feature = "arrows"))]
pub use trending_down::*;
#[cfg(any(feature = "charts", feature = "arrows"))]
pub use trending_up::*;
#[cfg(any(feature = "charts", feature = "arrows"))]
pub use trending_up_down::*;
#[cfg(feature = "shapes")]
pub use triangle::*;
#[cfg(any(feature = "notifications", feature = "shapes", feature = "development"))]
pub use triangle_alert::*;
#[cfg(feature = "shapes")]
pub use triangle_dashed::*;
#[cfg(any(feature = "shapes", feature = "math"))]
pub use triangle_right::*;
#[cfg(any(feature = "sports", feature = "gaming"))]
pub use trophy::*;
#[cfg(feature = "transportation")]
pub use truck::*;
#[cfg(feature = "transportation")]
pub use truck_electric::*;
#[cfg(feature = "animals")]
pub use turtle::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "communication"))]
pub use tv::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use tv_minimal::*;
#[cfg(any(feature = "devices", feature = "multimedia"))]
pub use tv_minimal_play::*;
#[cfg(any(
    feature = "brands",
    feature = "social",
    feature = "account",
    feature = "gaming"
))]
pub use twitch::*;
#[cfg(any(feature = "brands", feature = "social", feature = "account"))]
pub use twitter::*;
#[cfg(feature = "text")]
pub use r#type::*;
#[cfg(feature = "text")]
pub use type_outline::*;
#[cfg(feature = "weather")]
pub use umbrella::*;
#[cfg(feature = "weather")]
pub use umbrella_off::*;
#[cfg(feature = "text")]
pub use underline::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use undo::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use undo_2::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use undo_dot::*;
#[cfg(any(feature = "arrows", feature = "layout"))]
pub use unfold_horizontal::*;
#[cfg(any(feature = "arrows", feature = "layout"))]
pub use unfold_vertical::*;
#[cfg(any(feature = "shapes", feature = "files"))]
pub use ungroup::*;
#[cfg(any(feature = "buildings", feature = "navigation"))]
pub use university::*;
#[cfg(feature = "text")]
pub use unlink::*;
#[cfg(feature = "text")]
pub use unlink_2::*;
#[cfg(any(feature = "devices", feature = "development"))]
pub use unplug::*;
#[cfg(any(feature = "arrows", feature = "files"))]
pub use upload::*;
#[cfg(any(feature = "devices", feature = "multimedia", feature = "home"))]
pub use usb::*;
#[cfg(feature = "account")]
pub use user::*;
#[cfg(feature = "account")]
pub use user_check::*;
#[cfg(feature = "account")]
pub use user_cog::*;
#[cfg(any(feature = "account", feature = "security"))]
pub use user_lock::*;
#[cfg(feature = "account")]
pub use user_minus::*;
#[cfg(feature = "account")]
pub use user_pen::*;
#[cfg(feature = "account")]
pub use user_plus::*;
#[cfg(feature = "account")]
pub use user_round::*;
#[cfg(feature = "account")]
pub use user_round_check::*;
#[cfg(feature = "account")]
pub use user_round_cog::*;
#[cfg(feature = "account")]
pub use user_round_minus::*;
#[cfg(feature = "account")]
pub use user_round_pen::*;
#[cfg(feature = "account")]
pub use user_round_plus::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use user_round_search::*;
#[cfg(feature = "account")]
pub use user_round_x::*;
#[cfg(any(feature = "account", feature = "social"))]
pub use user_search::*;
#[cfg(feature = "account")]
pub use user_x::*;
#[cfg(feature = "account")]
pub use users::*;
#[cfg(feature = "account")]
pub use users_round::*;
#[cfg(any(feature = "food-beverage", feature = "travel", feature = "navigation"))]
pub use utensils::*;
#[cfg(any(feature = "food-beverage", feature = "travel", feature = "navigation"))]
pub use utensils_crossed::*;
#[cfg(any(feature = "buildings", feature = "home", feature = "sustainability"))]
pub use utility_pole::*;
#[cfg(any(feature = "development", feature = "math"))]
pub use variable::*;
#[cfg(any(feature = "security", feature = "travel", feature = "home"))]
pub use vault::*;
#[cfg(any(feature = "food-beverage", feature = "sustainability"))]
pub use vegan::*;
#[cfg(any(feature = "account", feature = "gaming"))]
pub use venetian_mask::*;
#[cfg(feature = "medical")]
pub use venus::*;
#[cfg(feature = "medical")]
pub use venus_and_mars::*;
#[cfg(any(
    feature = "devices",
    feature = "connectivity",
    feature = "account",
    feature = "notifications"
))]
pub use vibrate::*;
#[cfg(any(feature = "devices", feature = "connectivity", feature = "account"))]
pub use vibrate_off::*;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
pub use video::*;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography"
))]
pub use video_off::*;
#[cfg(any(
    feature = "devices",
    feature = "communication",
    feature = "connectivity",
    feature = "photography",
    feature = "files"
))]
pub use videotape::*;
#[cfg(any(feature = "design", feature = "photography"))]
pub use view::*;
#[cfg(any(feature = "connectivity", feature = "devices", feature = "social"))]
pub use voicemail::*;
#[cfg(any(feature = "sports", feature = "gaming", feature = "travel"))]
pub use volleyball::*;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
pub use volume::*;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
pub use volume_1::*;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
pub use volume_2::*;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
pub use volume_off::*;
#[cfg(any(
    feature = "connectivity",
    feature = "communication",
    feature = "multimedia"
))]
pub use volume_x::*;
#[cfg(feature = "social")]
pub use vote::*;
#[cfg(any(feature = "account", feature = "finance"))]
pub use wallet::*;
#[cfg(any(feature = "account", feature = "finance"))]
pub use wallet_cards::*;
#[cfg(any(feature = "account", feature = "finance"))]
pub use wallet_minimal::*;
#[cfg(any(feature = "account", feature = "devices"))]
pub use wallpaper::*;
#[cfg(any(
    feature = "design",
    feature = "gaming",
    feature = "cursors",
    feature = "photography"
))]
pub use wand::*;
#[cfg(any(
    feature = "design",
    feature = "gaming",
    feature = "cursors",
    feature = "photography"
))]
pub use wand_sparkles::*;
#[cfg(any(feature = "buildings", feature = "navigation"))]
pub use warehouse::*;
#[cfg(any(feature = "home", feature = "devices", feature = "travel"))]
pub use washing_machine::*;
#[cfg(feature = "time")]
pub use watch::*;
#[cfg(any(
    feature = "weather",
    feature = "navigation",
    feature = "multimedia",
    feature = "sustainability"
))]
pub use waves::*;
#[cfg(any(feature = "sports", feature = "home"))]
pub use waves_ladder::*;
#[cfg(any(
    feature = "security",
    feature = "account",
    feature = "navigation",
    feature = "development",
    feature = "social"
))]
pub use waypoints::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "communication"
))]
pub use webcam::*;
#[cfg(any(feature = "development", feature = "social", feature = "account"))]
pub use webhook::*;
#[cfg(any(feature = "development", feature = "social", feature = "account"))]
pub use webhook_off::*;
#[cfg(feature = "math")]
pub use weight::*;
#[cfg(feature = "food-beverage")]
pub use wheat::*;
#[cfg(feature = "food-beverage")]
pub use wheat_off::*;
#[cfg(feature = "text")]
pub use whole_word::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi_high::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi_low::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi_off::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi_pen::*;
#[cfg(any(feature = "connectivity", feature = "devices"))]
pub use wifi_zero::*;
#[cfg(any(feature = "weather", feature = "sustainability"))]
pub use wind::*;
#[cfg(any(feature = "weather", feature = "sustainability"))]
pub use wind_arrow_down::*;
#[cfg(feature = "food-beverage")]
pub use wine::*;
#[cfg(feature = "food-beverage")]
pub use wine_off::*;
#[cfg(feature = "development")]
pub use workflow::*;
#[cfg(any(feature = "animals", feature = "security"))]
pub use worm::*;
#[cfg(any(feature = "text", feature = "arrows"))]
pub use wrap_text::*;
#[cfg(any(feature = "account", feature = "development", feature = "tools"))]
pub use wrench::*;
#[cfg(any(feature = "notifications", feature = "math"))]
pub use x::*;
#[cfg(any(feature = "multimedia", feature = "social", feature = "brands"))]
pub use youtube::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "photography",
    feature = "weather"
))]
pub use zap::*;
#[cfg(any(
    feature = "connectivity",
    feature = "devices",
    feature = "photography",
    feature = "weather"
))]
pub use zap_off::*;
#[cfg(any(
    feature = "accessibility",
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "photography"
))]
pub use zoom_in::*;
#[cfg(any(
    feature = "accessibility",
    feature = "layout",
    feature = "design",
    feature = "text",
    feature = "photography"
))]
pub use zoom_out::*;
