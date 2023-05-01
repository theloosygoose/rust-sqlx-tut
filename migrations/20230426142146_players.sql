-- Add migration script here
CREATE TABLE IF NOT EXISTS players 
(
  id            INTEGER PRIMARY KEY NOT NULL,
  player_id     VARCHAR(250)        NOT NULL,
  name          VARCHAR(100)        NOT NULL,
  active        BOOLEAN             NOT NULL DEFAULT 0,
  work_ethic    INTEGER             NOT NULL DEFAULT 0,
  dog           INTEGER             NOT NULL DEFAULT 0,
  loyalty       INTEGER             NOT NULL DEFAULT 0
)
