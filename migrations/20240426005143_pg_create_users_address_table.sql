create table if not exists "user"
(
    id            char(24) primary key not null,
    first_name    varchar(50),
    last_name     varchar(50)          not null,
    email         varchar(255)         not null,
    password_hash varchar(255)         not null,
    phone_number  varchar(15),
    created_at    timestamptz          not null default now(),
    updated_at    timestamptz          not null default now(),
    deleted_at    timestamptz
);

create table if not exists address
(
    id         char(24) primary key not null,
    user_id    char(24)             not null,
    line_one   varchar(255)         not null,
    line_two   varchar(255),
    city       varchar(50)          not null,
    state      varchar(50)          not null,
    zip        varchar(10),
    country    varchar(50)          not null,
    created_at timestamptz          not null default now(),
    updated_at timestamptz          not null default now(),
    deleted_at timestamptz,
    constraint fk_user foreign key (user_id) references "user" (id)
);

create type event_type as enum ('User', 'Auth', 'Other');
create type event_name as enum ('CreateUser', 'RegisterUser', 'UpdateEmail', 'ConfirmEmailAddress', 'ResetPassword', 'UpdatePassword', 'ForgotPassword', 'InvalidPassword');

create table if not exists event
(
    id         bigserial primary key not null,
    user_id    char(24)              not null,
    event_type event_type            not null,
    event_name event_name            not null,
    event_time timestamptz           not null default now(),
    constraint fk_user foreign key (user_id) references "user" (id)
);
