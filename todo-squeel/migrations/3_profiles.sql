
create table "profiles" (
  id         uuid primary key default uuid_generate_v1mc(),
  user_id    uuid not null references "users" (id) on delete cascade,
  avatar     text,
  name       text, 
  bio        text,
  created_at timestamptz not null default now(),
  updated_at timestamptz    
);


SELECT trigger_updated_at('"profiles"');
