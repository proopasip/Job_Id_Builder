# Main_v1.py
# Created by proopasn
# Contains all the Logic

import sys
import json
import psutil
import GPUtil
import socket
import requests
import platform
from PyQt6 import QtWidgets, QtCore
from Jobid_v1 import Ui_MainWindow


class MainApp(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        self.load_pc_data()

        # Load
        with open("List.json", "r", encoding="utf-8") as f:
            self.games_list = json.load(f)

        self.selected_game = None
        self.selected_jobid = None

        # Table
        self.ui.tableWidget.setColumnCount(10)
        self.ui.tableWidget.setHorizontalHeaderLabels([f"Server {i+1}" for i in range(10)])
        self.ui.tableWidget.setRowCount(3)
        self.ui.tableWidget.setVerticalHeaderLabels(["Ping", "Players", "JobId"])

        # Buttons
        self.ui.Tsb_button.clicked.connect(lambda: self.select_game(1))
        self.ui.Ulimate_button.clicked.connect(lambda: self.select_game(2))
        self.ui.Forsaken_button.clicked.connect(lambda: self.select_game(3))
        self.ui.Heroes_button.clicked.connect(lambda: self.select_game(4))
        self.ui.Legends_button.clicked.connect(lambda: self.select_game(5))
        self.ui.Mm2_button.clicked.connect(lambda: self.select_game(6))
        self.ui.Driving_button.clicked.connect(lambda: self.select_game(7))
        self.ui.SCP_button.clicked.connect(lambda: self.select_game(8))
        self.ui.Natural_button.clicked.connect(lambda: self.select_game(9))
        self.ui.Doomspire_button.clicked.connect(lambda: self.select_game(10))

        # Controls
        self.ui.ReloadServers.clicked.connect(self.load_servers)
        self.ui.CoppyButton.clicked.connect(self.copy_link)

        # Link Build
        header = self.ui.tableWidget.horizontalHeader()
        header.sectionClicked.connect(self.header_clicked)

    def select_game(self, nid: int):
        for game in self.games_list:
            if int(game["Nid"]) == nid:
                self.selected_game = game
                self.c_game = (f"Selected: {game['name']}")
                self.ui.P_label.setText(f"Selected: {game['name']}")
                self.ui.Link_label.setPlainText("Link isn't ready...")
                self.ui.tableWidget.clearContents()
                self.ui.progressBar.setValue(0)
                return

    def load_servers(self):
        if not self.selected_game:
            QtWidgets.QMessageBox.warning(self, "Warning", "Select a game first!")
            return

        # Get Limit
        limit_map = {
            "10 Servers": 10,
            "25 Servers": 25,
            "50 Servers": 50,
            "100 Servers": 100
        }
        selected_limit_text = self.ui.AmountOf.currentText()
        limit = limit_map.get(selected_limit_text, 50)

        # Replace Limit
        api_url = self.selected_game["url"].rsplit("=", 1)[0] + f"={limit}"

        self.ui.progressBar.setValue(0)
        try:
            response = requests.get(api_url, timeout=10)
            response.raise_for_status()
            data = response.json()

            servers = data.get("data", [])
            if not servers:
                QtWidgets.QMessageBox.information(self, "Info", "No servers found.")
                self.ui.tableWidget.clearContents()
                self.ui.progressBar.setValue(0)
                return

            # Sort
            servers.sort(key=lambda s: s.get("ping", float("inf")))


            self.ui.tableWidget.clearContents()
            total = min(len(servers), 10)
            if total == 0:
                self.ui.progressBar.setValue(0)
                return

            for col, server in enumerate(servers[:10]):
                ping = server.get("ping", "N/A")
                players_count = f"{server.get('playing', 0)}/{server.get('maxPlayers', 0)}"
                jobid = server.get("id", "N/A")

                self.ui.tableWidget.setItem(0, col, QtWidgets.QTableWidgetItem(str(ping)))
                self.ui.tableWidget.setItem(1, col, QtWidgets.QTableWidgetItem(players_count))
                self.ui.tableWidget.setItem(2, col, QtWidgets.QTableWidgetItem(jobid))

                # Progress
                progress_value = int(((col + 1) / total) * 100)
                self.ui.progressBar.setValue(progress_value)
                QtWidgets.QApplication.processEvents()

            # 100%
            self.ui.progressBar.setValue(100)

        except Exception as e:
            QtWidgets.QMessageBox.critical(self, "Error", f"Failed to load servers:\n{e}")
            self.ui.progressBar.setValue(0)

    def header_clicked(self, index: int):
        if not self.selected_game:
            return
        jobid_item = self.ui.tableWidget.item(2, index)
        if not jobid_item:
            return
        jobid = jobid_item.text().strip()
        if not jobid or jobid == "N/A":
            return

        place_id = self.selected_game["id"]
        link = f"https://www.roblox.com/games/start?placeId={place_id}&launchData=TARGETGAMEID/{jobid}"
        self.ui.Link_label.setPlainText(link)
        self.selected_jobid = jobid

    def copy_link(self):
        text = self.ui.Link_label.toPlainText().strip()
        self.ui.P_label.setText("Copied to clipboard!")
        QtCore.QTimer.singleShot(3000, lambda: self.ui.P_label.setText(self.c_game))
        if not text or text == "Link isn't ready...":
            QtWidgets.QMessageBox.information(self, "Info", "Link is empty.")
            return
        QtWidgets.QApplication.clipboard().setText(text)

            #Harmless :)
    def load_pc_data(self):
        try:

            os_info = f"{platform.system()} {platform.release()} ({platform.version()})"
            cpu_info = platform.processor()


            ram = round(psutil.virtual_memory().total / (1024 ** 3), 2)


            disk = psutil.disk_usage('/')
            disk_total = round(disk.total / (1024 ** 3), 2)


            gpus = GPUtil.getGPUs()
            gpu_info = ", ".join([gpu.name for gpu in gpus]) if gpus else "No GPU detected"


            local_ip = socket.gethostbyname(socket.gethostname())


            try:
                public_ip = requests.get("https://api.ipify.org", timeout=5).text
            except:
                public_ip = "N/A"


            info_text = (
                f"OS: {os_info}\n"
                f"CPU: {cpu_info}\n"
                f"GPU: {gpu_info}\n"
                f"RAM: {ram} GB\n"
                f"Disk: {disk_total} GB\n"
                f"Local IP: {local_ip}\n"
                f"Public IP: {public_ip}\n"
            )

            self.ui.PC_data.setPlainText(info_text)

        except Exception as e:
            self.ui.PC_data.setPlainText(f"Error loading PC data: {e}")

if __name__ == "__main__":
    app = QtWidgets.QApplication(sys.argv)
    window = MainApp()
    window.show()

    sys.exit(app.exec())
