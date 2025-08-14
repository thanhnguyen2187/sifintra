// @generated automatically by Diesel CLI.

diesel::table! {
    raw__sepay (id) {
        gateway -> Nullable<Text>,
        transactionDate -> Nullable<Text>,
        accountNumber -> Nullable<Text>,
        subAccount -> Nullable<Text>,
        code -> Nullable<Text>,
        content -> Nullable<Text>,
        transferType -> Nullable<Text>,
        description -> Nullable<Text>,
        transferAmount -> Nullable<Integer>,
        referenceCode -> Nullable<Text>,
        accumulated -> Nullable<Integer>,
        id -> Nullable<Integer>,
    }
}

diesel::table! {
    user__category (id) {
        id -> Nullable<Text>,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user__transaction (id) {
        id -> Nullable<Text>,
        date_timestamp -> Integer,
        description -> Text,
        amount -> Integer,
        category_id -> Nullable<Text>,
        source_id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    raw__sepay,
    user__category,
    user__transaction,
);
