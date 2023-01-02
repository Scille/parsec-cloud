# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtWidgets import QWidget

from parsec._parsec import OrganizationConfig, OrganizationStats
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.core.gui import desktop, file_size
from parsec.core.gui.custom_dialogs import GreyedDialog
from parsec.core.gui.lang import translate
from parsec.core.gui.snackbar_widget import SnackbarManager
from parsec.core.gui.ui.organization_info_widget import Ui_OrganizationInfoWidget


class OrganizationInfoWidget(QWidget, Ui_OrganizationInfoWidget):
    def __init__(
        self,
        profile: UserProfile,
        org_addr: str,
        stats: OrganizationStats | None = None,
        config: OrganizationConfig | None = None,
    ):
        super().__init__()
        self.setupUi(self)

        self.dialog: GreyedDialog | None = None
        self.label_backend_addr.setText(org_addr)
        self.label_backend_addr.setCursorPosition(0)
        self.button_copy_to_clipboard.clicked.connect(lambda: self._on_copy_addr_clicked(org_addr))

        if profile != UserProfile.ADMIN:
            self.widget_size.hide()
            self.widget_users.hide()

        if stats:
            self.label_data_size.setText(
                translate("TEXT_ORG_INFO_DATA_SIZE-size").format(
                    size=file_size.get_filesize(stats.data_size)
                )
            )
            self.label_metadata_size.setText(
                translate("TEXT_ORG_INFO_METADATA_SIZE-size").format(
                    size=file_size.get_filesize(stats.metadata_size)
                )
            )
            self.label_total_size.setText(
                translate("TEXT_ORG_INFO_DATA_TOTAL_SIZE-size").format(
                    size=file_size.get_filesize(stats.data_size + stats.metadata_size)
                )
            )

            self.label_user_active.setText(
                translate("TEXT_ORG_INFO_ACTIVE_USERS_COUNT-count").format(count=stats.active_users)
            )
            self.label_user_revoked.setText(
                translate("TEXT_ORG_INFO_REVOKED_USERS_COUNT-count").format(
                    count=stats.users - stats.active_users
                )
            )
            for details in stats.users_per_profile_detail:
                if details.profile == UserProfile.ADMIN:
                    self.label_user_admin.setText(
                        translate("TEXT_ORG_INFO_ADMIN_COUNT-count").format(count=details.active)
                    )
                elif details.profile == UserProfile.STANDARD:
                    self.label_user_standard.setText(
                        translate("TEXT_ORG_INFO_STANDARD_COUNT-count").format(count=details.active)
                    )
                elif details.profile == UserProfile.OUTSIDER:
                    self.label_user_outsider.setText(
                        translate("TEXT_ORG_INFO_OUTSIDER_COUNT-count").format(count=details.active)
                    )

        if config:
            if config.user_profile_outsider_allowed:
                self.label_outsider_allowed.setText(translate("TEXT_ORG_INFO_OUTSIDER_ALLOWED"))
            else:
                self.label_outsider_allowed.setText(translate("TEXT_ORG_INFO_OUTSIDER_NOT_ALLOWED"))
            if not config.active_users_limit:
                self.label_user_limit.setText(translate("TEXT_ORG_INFO_USER_LIMIT_UNLIMITED"))
            else:
                self.label_user_limit.setText(
                    translate("TEXT_ORG_INFO_USER_LIMIT-limit").format(
                        limit=config.active_users_limit
                    )
                )
            self.label_sequestration_state.setToolTip(
                translate("TEXT_ORG_INFO_SEQUESTRATION_TOOLTIP")
            )
            if getattr(config, "sequester_authority", None):
                self.label_sequestration_state.setText(
                    translate("TEXT_ORG_INFO_SEQUESTRATION_ACTIVATED")
                )
            else:
                self.label_sequestration_state.setVisible(False)

    def _on_copy_addr_clicked(self, org_addr: str) -> None:
        desktop.copy_to_clipboard(org_addr)
        SnackbarManager.inform(translate("TEXT_BACKEND_ADDR_COPIED_TO_CLIPBOARD"))

    @classmethod
    async def show_modal(
        cls,
        profile: UserProfile,
        org_id: OrganizationID,
        org_addr: str,
        stats: OrganizationStats | None = None,
        config: OrganizationConfig | None = None,
        parent: QWidget | None = None,
    ) -> GreyedDialog:
        w = cls(profile, org_addr=org_addr, stats=stats, config=config)
        d = GreyedDialog(w, title=org_id.str, parent=parent, width=800)
        w.dialog = d

        d.show()
        return d
