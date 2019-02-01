# Used only for translations

SOURCES += app.py \
           main_window.py \
           files_widget.py \
           login_widget.py \
           settings_widget.py \
           users_widget.py \
           file_size.py \
           custom_widgets.py \
           devices_widget.py \
           workspaces_widget.py


TRANSLATIONS += tr/parsec_fr.ts \
                tr/parsec_en.ts


FORMS += forms/main_window.ui \
         forms/files_widget.ui \
         forms/users_widget.ui \
         forms/settings_widget.ui \
         forms/login_widget.ui \
         forms/register_device_dialog.ui \
         forms/devices_widget.ui \
         forms/login_login_widget.ui \
         forms/claim_device_widget.ui \
         forms/claim_user_widget.ui \
         forms/mount_widget.ui \
         forms/workspaces_widget.ui \
         forms/workspace_button.ui \
         forms/message_dialog.ui \
         forms/user_button.ui \
         forms/input_dialog.ui \
         forms/question_dialog.ui \
         forms/shared_dialog.ui \
         forms/device_button.ui \
         forms/network_settings_widget.ui \
         forms/global_settings_widget.ui \
         forms/register_user_dialog.ui \
         forms/settings_dialog.ui \
         forms/starting_guide_dialog.ui \
         forms/central_widget.ui \
         forms/notification_center_widget.ui \
         forms/menu_widget.ui \
         forms/notification_widget.ui \
    forms/bootstrap_organization_widget.ui

RESOURCES += rc/resources.qrc
