SELECT EXISTS (SELECT name FROM sqlite_master WHERE type='table' AND name='testtable');
create table 'kv:user_settings' (key text, value text);
drop table  'kv:user_settings';
insert into 'kv:user_settings' (key, value) values ('testkey3', 'testvalue3');
select value from "kv:user_settings" where key='testkey1';
select * from "kv:user_settings";
insert or replace into "kv:user_settings" values ('testkey3', 'removeit');
update "kv:user_settings" set value='testvalue3' where key='testkey3'