CREATE TABLE user_devices
(
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    id           UUID PRIMARY KEY,
    user_id      UUID         NOT NULL,
    device_token VARCHAR(255) NOT NULL,
    os           VARCHAR(10)  NOT NULL
);