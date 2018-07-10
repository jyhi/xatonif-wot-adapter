table! {
    device_actions (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    device_assoc_actions (id) {
        device_id -> Unsigned<Integer>,
        action_id -> Unsigned<Integer>,
        id -> Integer,
    }
}

table! {
    device_assoc_events (id) {
        device_id -> Unsigned<Integer>,
        event_id -> Unsigned<Integer>,
        id -> Integer,
    }
}

table! {
    device_assoc_properties (id) {
        device_id -> Unsigned<Integer>,
        prop_id -> Unsigned<Integer>,
        id -> Integer,
    }
}

table! {
    device_events (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    device_event_logs (id) {
        id -> Unsigned<Integer>,
        time -> Datetime,
        dev_id -> Unsigned<Integer>,
        event_id -> Unsigned<Integer>,
        task_id -> Unsigned<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    device_list (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        description -> Nullable<Text>,
        assoc_properties_id -> Unsigned<Integer>,
        assoc_actions_id -> Unsigned<Integer>,
        assoc_events_id -> Unsigned<Integer>,
    }
}

table! {
    device_properties (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        description -> Nullable<Text>,
        href -> Nullable<Text>,
    }
}

table! {
    task_assoc_devs (id) {
        task_id -> Unsigned<Integer>,
        dev_id -> Unsigned<Integer>,
        id -> Integer,
    }
}

table! {
    task_list (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        start_time -> Datetime,
        description -> Nullable<Text>,
    }
}

table! {
    user_action_logs (id) {
        id -> Unsigned<Integer>,
        time -> Datetime,
        user_id -> Unsigned<Integer>,
        action_id -> Unsigned<Integer>,
        dev_id -> Unsigned<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    user_list (user_id) {
        user_id -> Unsigned<Integer>,
        user_name -> Text,
        password -> Varchar,
    }
}

joinable!(device_assoc_actions -> device_actions (action_id));
joinable!(device_assoc_actions -> device_list (device_id));
joinable!(device_assoc_events -> device_events (event_id));
joinable!(device_assoc_events -> device_list (device_id));
joinable!(device_assoc_properties -> device_list (device_id));
joinable!(device_assoc_properties -> device_properties (prop_id));
joinable!(device_event_logs -> device_events (event_id));
joinable!(device_event_logs -> device_list (dev_id));
joinable!(device_event_logs -> task_list (task_id));
joinable!(task_assoc_devs -> device_list (dev_id));
joinable!(task_assoc_devs -> task_list (task_id));
joinable!(task_list -> user_list (user_id));
joinable!(user_action_logs -> device_actions (action_id));
joinable!(user_action_logs -> device_list (dev_id));
joinable!(user_action_logs -> user_list (user_id));

allow_tables_to_appear_in_same_query!(
    device_actions,
    device_assoc_actions,
    device_assoc_events,
    device_assoc_properties,
    device_events,
    device_event_logs,
    device_list,
    device_properties,
    task_assoc_devs,
    task_list,
    user_action_logs,
    user_list,
);
