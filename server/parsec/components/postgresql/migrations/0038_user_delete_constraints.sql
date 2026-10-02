-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Enable cascade deletion on foreign key related to user
--
-------------------------------------------------------

ALTER TABLE user_ DROP CONSTRAINT user__human_fkey;

ALTER TABLE user_ ADD CONSTRAINT user_human_fkey FOREIGN KEY (human) REFERENCES human (_id) ON DELETE CASCADE;


ALTER TABLE profile DROP CONSTRAINT profile_user__fkey;

ALTER TABLE profile ADD CONSTRAINT profile_user_fkey FOREIGN KEY (user_) REFERENCES user_ (_id) ON DELETE CASCADE;


ALTER TABLE device DROP CONSTRAINT device_user__fkey;

ALTER TABLE device ADD CONSTRAINT device_user_fkey FOREIGN KEY (user_) REFERENCES user_ (_id) ON DELETE CASCADE;


ALTER TABLE invitation DROP CONSTRAINT invitation_device_invitation_claimer_fkey;

ALTER TABLE invitation ADD CONSTRAINT invitation_device_invitation_claimer_fkey FOREIGN KEY (
    device_invitation_claimer
) REFERENCES user_ (_id) ON DELETE CASCADE;


ALTER TABLE greeting_session DROP CONSTRAINT greeting_session_greeter_fkey;

ALTER TABLE greeting_session ADD CONSTRAINT greeting_session_greeter_fkey FOREIGN KEY (greeter) REFERENCES user_ (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_user_role DROP CONSTRAINT realm_user_role_user__fkey;

ALTER TABLE realm_user_role ADD CONSTRAINT realm_user_role_user_fkey FOREIGN KEY (user_) REFERENCES user_ (
    _id
) ON DELETE CASCADE;


ALTER TABLE realm_keys_bundle_access DROP CONSTRAINT realm_keys_bundle_access_user__fkey;

ALTER TABLE realm_keys_bundle_access ADD CONSTRAINT realm_keys_bundle_access_user_fkey FOREIGN KEY (
    user_
) REFERENCES user_ (_id) ON DELETE CASCADE;
