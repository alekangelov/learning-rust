create table "users" (
    id uuid primary key default uuid_generate_v1mc(),
    username text not null unique collate "case_insensitive",
    password text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz                            
);

create table "todos" (
    id uuid primary key default uuid_generate_v1mc(),
    user_id uuid not null references "users" (id) on delete cascade,
    title text not null,
    description text,
    image text,
    completed boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz                            
)

SELECT trigger_updated_at('"users"');
SELECT trigger_updated_at('"todos"');