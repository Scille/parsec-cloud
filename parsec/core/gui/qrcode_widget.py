# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from PyQt5.QtWidgets import QWidget, QVBoxLayout
from PyQt5.QtGui import QPixmap, QPainter, QImage, QColor
from PIL.ImageQt import ImageQt

import qrcode

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import OverlayLabel
from parsec.core.gui.custom_dialogs import GreyedDialog

from parsec.core.gui.ui.qrcode_widget import Ui_QRCodeWidget


PARSEC_LOGO = None


def generate_qr_code(text):
    global PARSEC_LOGO

    if not PARSEC_LOGO:
        img = QImage(":/logos/images/logos/parsec2.png")
        if img:
            img = img.convertToFormat(QImage.Format_ARGB32)
            PARSEC_LOGO = QImage(img.width() + 30, img.height() + 20, QImage.Format_ARGB32)
            PARSEC_LOGO.fill(QColor(0xF4, 0xF4, 0xF4))
            painter = QPainter()
            painter.begin(PARSEC_LOGO)
            painter.setCompositionMode(QPainter.CompositionMode_SourceOver)
            painter.drawImage(15, 10, img)
            painter.end()

    qr = qrcode.QRCode(
        version=None, error_correction=qrcode.constants.ERROR_CORRECT_H, border=4, box_size=10
    )
    qr.add_data(text)
    qr.make(fit=True)
    # img = qr.make_image(back_color="#F4F4F4", fill_color="#000000")
    img = qr.make_image(back_color="#F4F4F4", fill_color="#5193FF")
    qimg = ImageQt(img)
    final_img = None
    if PARSEC_LOGO:
        final_img = QImage(qimg.width(), qimg.height(), QImage.Format_ARGB32)
        x = int(final_img.width() / 2 - PARSEC_LOGO.width() / 2)
        y = int(final_img.height() / 2 - PARSEC_LOGO.height() / 2)
        qimg = qimg.convertToFormat(QImage.Format_ARGB32)
        painter = QPainter()
        painter.begin(final_img)
        painter.setCompositionMode(QPainter.CompositionMode_SourceOver)
        painter.drawImage(0, 0, qimg)
        painter.drawImage(x, y, PARSEC_LOGO)
        painter.end()
    else:
        final_img = qimg
    pix = QPixmap.fromImage(final_img)
    return pix


class _QRCodeWidget(QWidget, Ui_QRCodeWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setupUi(self)


class SmallQRCodeWidget(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setLayout(QVBoxLayout())
        self.qrcode = _QRCodeWidget()
        self.qrcode.label_text.setText(_("TEXT_CLICK_TO_ENLARGE"))
        self.qrcode.label_qrcode.clicked.connect(self._on_image_clicked)
        self.qrcode.label_qrcode.set_mode(OverlayLabel.ClickMode.OpenFullScreen, True)
        self.qrcode.label_qrcode.setFixedSize(100, 100)
        self.layout().addWidget(self.qrcode)
        self.image = None

    def set_image(self, img):
        self.image = img
        self.qrcode.label_qrcode.setPixmap(img)

    def _on_image_clicked(self):
        if not self.image:
            return
        _show_large_qrcode(self, self.image)


class LargeQRCodeWidget(QWidget):
    def __init__(self, image, parent=None):
        super().__init__(parent)
        self.dialog = None
        self.setLayout(QVBoxLayout())
        self.qrcode = _QRCodeWidget()
        self.qrcode.label_text.setText(_("TEXT_CLICK_TO_CLOSE"))
        self.qrcode.label_qrcode.clicked.connect(self._on_image_clicked)
        self.qrcode.label_qrcode.set_mode(OverlayLabel.ClickMode.CloseFullScreen, False)
        self.qrcode.label_qrcode.setPixmap(image)
        self.qrcode.label_qrcode.setFixedSize(400, 400)
        self.layout().addWidget(self.qrcode)

    def _on_image_clicked(self):
        self.dialog.accept()


def _show_large_qrcode(parent, image):
    w = LargeQRCodeWidget(image)
    d = GreyedDialog(w, title=None, parent=parent, hide_close=True, close_on_click=True)
    w.dialog = d
    return d.open()
