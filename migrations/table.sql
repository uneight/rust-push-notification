create table if not exists user_devices
(
    created_at   timestamp default current_timestamp,
    user_id      uuid         not null,
    device_token varchar(255) not null,
    os           varchar(10)  not null,
    primary key (user_id, device_token)
);