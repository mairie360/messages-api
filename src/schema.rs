// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        #[max_length = 320]
        sender -> Varchar,
        #[max_length = 320]
        recipient -> Varchar,
        content -> Nullable<Text>,
        is_read -> Nullable<Bool>,
        sent_at -> Nullable<Timestamp>,
    }
}
