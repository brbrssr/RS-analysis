import flet as fl
import gui as gui


class App(fl.UserControl):
    def __inti__(self):
        super().__init__()
        
    def build(self):
        return 


def main(page: fl.Page):
    page.title = "RS-analysis"
    app = App()
    page.add(app)

fl.app(target=main)