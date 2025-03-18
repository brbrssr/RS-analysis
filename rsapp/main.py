import tkinter as tk
import gui as gui


class App(tk.Tk):

    def __init__(self) -> None:
        super().__init__()

        self.title("RS Analysis")
        self.geometry("360x640")
        #self.resizable(False, False)

        icon = tk.PhotoImage(file="./gui_assets/rs_anal_icon.png")
        self.iconphoto(True, icon)

        self.gui = gui.GUI_elements(self).pack()


if __name__ == "__main__":
    app = App()
    app.mainloop()
