// @generated automatically by Diesel CLI.

table! {
    appointments (appointment_id) {
        appointment_id -> Uuid,
        at -> Date,
        user_id -> Nullable<Uuid>,
        picture_consent -> Nullable<Bool>,
        analysis_consent -> Nullable<Bool>,
        data_consent -> Bool,
        analysis -> Nullable<Text>,
        picture -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        confirmed_at -> Nullable<Timestamp>,
        token -> Nullable<Uuid>,
    }
}

table! {
    feedback (feedback_id) {
        feedback_id -> Uuid,
        text -> Text,
        rating -> Int2,
        nps -> Int2,
        created_at -> Nullable<Timestamp>,
        user_id -> Uuid,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        name -> Text,
        surname -> Text,
        fiscal_number -> Bpchar,
        gender -> Bpchar,
        category -> Varchar,
        email -> Text,
        phone_number -> Text,
        education -> Nullable<Text>,
        job -> Nullable<Text>,
        workplace -> Nullable<Text>,
        other_associations -> Nullable<Text>,
        birthday -> Date,
        birthplace -> Text,
        birthplace_province -> Bpchar,
        birthplace_istat_code -> Bpchar,
        address -> Text,
        city -> Text,
        province -> Bpchar,
        zip_code -> Bpchar,
        istat_code -> Bpchar,
    }
}

joinable!(appointments -> users (user_id));
joinable!(feedback -> users (user_id));

allow_tables_to_appear_in_same_query!(
    appointments,
    feedback,
    users,
);