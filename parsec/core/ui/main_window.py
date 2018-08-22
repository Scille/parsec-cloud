from PyQt5.QtWidgets import QMainWindow, QPushButton, QHBoxLayout, QWidget


class MainWindow(QMainWindow):
    def __init__(self, parsec_core, portal, cancel_scope, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self._parsec_core = parsec_core
        self._portal = portal
        self._cancel_scope = cancel_scope

        self.central_widget = QWidget(parent=self)
        self.central_widget.setLayout(QHBoxLayout())
        self.exit_button = QPushButton("Exit", parent=self.central_widget)
        self.stat_button = QPushButton("Stat", parent=self.central_widget)
        self.setCentralWidget(self.central_widget)
        self.central_widget.layout().addWidget(self.exit_button)
        self.central_widget.layout().addWidget(self.stat_button)
        self.exit_button.clicked.connect(self.on_exit_clicked)
        self.stat_button.clicked.connect(self.on_stat_clicked)

    def closeEvent(self, event):
        self._portal.run_sync(self._cancel_scope.cancel)

    def on_exit_clicked(self):
        self.close()

    def on_stat_clicked(self):
        print(self._portal.run(self._parsec_core.fs.stat, "/"))
