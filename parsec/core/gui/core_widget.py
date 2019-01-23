from PyQt5.QtWidgets import QWidget


class CoreWidget(QWidget):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._core = None
        self._portal = None

    def set_core_attributes(self, core, portal):
        self.core = core
        self.portal = portal

    @property
    def core(self):
        return self._core

    @core.setter
    def core(self, val):
        self._core = val

    @property
    def portal(self):
        return self._portal

    @portal.setter
    def portal(self, val):
        self._portal = val

    def get_taskbar_buttons(self):
        return []

    def reset(self):
        raise NotImplementedError()
