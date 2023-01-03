create table accounts
(
    email        text                               not null,
    password     bytea                              not null,
    id           bigserial                          not null
        constraint accounts_pk
            primary key,
    created_at   timestamp default now()
);
