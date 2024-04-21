create table "user"
(
    id         bigserial primary key not null,
    first_name varchar(50),
    last_name  varchar(50)           not null,
    email      varchar(255)          not null,
    created_at timestamptz           not null default now(),
    updated_at timestamptz           not null default now(),
    deleted_at timestamptz
);

create table address
(
    id         bigserial primary key not null,
    user_id    bigint                not null,
    line_one   varchar(255)          not null,
    line_two   varchar(255)          not null,
    city       varchar(50)           not null,
    state      varchar(50)           not null,
    zip        varchar(10)           not null,
    created_at timestamptz           not null default now(),
    updated_at timestamptz           not null default now(),
    deleted_at timestamptz,
    constraint fk_user foreign key (user_id) references "user" (id)
);

create type event_type as enum();
create type event_name as enum();

create table event
(
    id         bigserial primary key not null,
    user_id    bigint                not null,
    event_type event_type            not null,
    event_name event_name            not null,
    event_time timestamptz           not null default now(),
    constraint fk_user foreign key (user_id) references "user" (id)
);