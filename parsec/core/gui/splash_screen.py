from PyQt5.QtWidgets import QSplashScreen, QPixmap


class SplashScreen:
    def __init__(self):
        splash_screen = QSplashScreen(QPixmap(":/logos/parsec.png"))
        splash_screen.showMessage("Starting up...")
