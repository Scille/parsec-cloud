# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtCore import QTimer, pyqtSignal
from PyQt5.QtWidgets import QWidget

from parsec.api.protocol import UserID, UserProfile
from parsec.core.gui.lang import translate as T
from parsec.core.gui.trio_jobs import QtToTrioJob, QtToTrioJobScheduler
from parsec.core.gui.ui.search_user_widget import Ui_SearchUserWidget
from parsec.core.gui.ui.select_user_widget import Ui_SelectUserWidget
from parsec.core.logged_core import LoggedCore
from parsec.core.types import UserInfo


class SelectUserWidget(QWidget, Ui_SelectUserWidget):
    selected = pyqtSignal(UserInfo)

    def __init__(self, user_info: UserInfo) -> None:
        super().__init__()
        self.setupUi(self)
        self.user_info = user_info
        self.button_select.apply_style()
        self.button_select.clicked.connect(self._on_button_select_clicked)
        self.label_name.setText(
            f"{self.user_info.human_handle.label} <{self.user_info.human_handle.email}>"
            if self.user_info.human_handle
            else self.user_info.user_id
        )
        profiles_text = {
            UserProfile.OUTSIDER: T("TEXT_USER_PROFILE_OUTSIDER"),
            UserProfile.STANDARD: T("TEXT_USER_PROFILE_STANDARD"),
            UserProfile.ADMIN: T("TEXT_USER_PROFILE_ADMIN"),
        }
        self.label_profile.setText(profiles_text[user_info.profile])

    def _on_button_select_clicked(self) -> None:
        self.selected.emit(self.user_info)


class SearchUserWidget(QWidget, Ui_SearchUserWidget):
    user_selected = pyqtSignal(UserInfo)

    def __init__(
        self,
        core: LoggedCore,
        jobs_ctx: QtToTrioJobScheduler,
        exclude_users: list[UserID] | None = None,
    ) -> None:
        super().__init__()
        self.setupUi(self)
        self.core = core
        self.jobs_ctx = jobs_ctx
        self.last_search_user_job: QtToTrioJob[None] | None = None
        self.search_timer = QTimer()
        self.search_timer.setInterval(200)
        self.search_timer.setSingleShot(True)
        self.search_timer.timeout.connect(self._on_search_timer_timeout)
        self.line_search.textChanged.connect(self._on_search_changed)
        self.label_not_found.hide()
        self.user_list_scroll.hide()
        self.exclude_users = exclude_users

    def _on_search_timer_timeout(self) -> None:
        self.last_search_user_job = self.jobs_ctx.submit_job(
            None, None, self._search_users, self.line_search.text()
        )

    def _on_search_changed(self, text: str) -> None:
        if not text:
            self.search_timer.stop()
            self.user_list_scroll.hide()
            self._clear_users()
        else:
            self.search_timer.start()

    def _clear_users(self) -> None:
        while self.user_list.layout().count():
            item = self.user_list.layout().takeAt(0)
            if item and item.widget():
                item.widget().hide()
                item.widget().setParent(None)

    async def _search_users(self, text: str) -> None:
        users, _ = await self.core.find_humans(page=1, per_page=100, query=text, omit_revoked=True)
        if self.exclude_users:
            users = [user for user in users if user.user_id not in self.exclude_users]
        if not len(users):
            self.label_not_found.show()
            self.user_list_scroll.hide()
            return

        self._clear_users()
        for user in users:
            user_select = SelectUserWidget(user)
            user_select.selected.connect(lambda user_info: self.user_selected.emit(user_info))
            self.user_list.layout().addWidget(user_select)
        self.user_list_scroll.show()
