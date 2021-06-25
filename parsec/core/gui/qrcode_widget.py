# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import QRectF
from PyQt5.QtWidgets import QWidget, QVBoxLayout
from PyQt5.QtGui import QPixmap, QPainter, QImage, QColor
from PyQt5.QtSvg import QSvgRenderer

import qrcode
import qrcode.image.svg

import io

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
            logo = QImage(img.width() + 30, img.height() + 20, QImage.Format_ARGB32)
            logo.fill(QColor(0xF4, 0xF4, 0xF4))
            painter = QPainter()
            painter.begin(logo)
            painter.setCompositionMode(QPainter.CompositionMode_SourceOver)
            painter.drawImage(15, 10, img)
            painter.end()
            PARSEC_LOGO = logo

    qr = qrcode.QRCode(
        version=None, error_correction=qrcode.constants.ERROR_CORRECT_H, border=4, box_size=10
    )

    qr.add_data(text)
    qr.make(fit=True)
    # No idea why but SvgPathImage ignores the back_color and fill_color arguments and uses a const style.
    # So we replace the QT_PATH_STYLE string by our own.
    qrcode.image.svg.SvgPathImage.QR_PATH_STYLE = (
        "fill:#5193FF;fill-opacity:1;fill-rule:nonzero;stroke:none"
    )
    qr_img = qr.make_image(image_factory=qrcode.image.svg.SvgPathImage)
    stream = io.BytesIO()
    qr_img.save(stream)
    renderer = QSvgRenderer()
    renderer.load(stream.getvalue())

    final_img = QImage(600, 600, QImage.Format_ARGB32)
    final_img.fill(QColor(0xF4, 0xF4, 0xF4))

    painter = QPainter()
    painter.begin(final_img)
    painter.setCompositionMode(QPainter.CompositionMode_SourceOver)
    renderer.render(painter, QRectF(0, 0, final_img.rect().width(), final_img.rect().height()))

    if PARSEC_LOGO:
        x = int(final_img.width() / 2 - PARSEC_LOGO.width() / 2)
        y = int(final_img.height() / 2 - PARSEC_LOGO.height() / 2)
        painter.drawImage(x, y, PARSEC_LOGO)

    painter.end()

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
