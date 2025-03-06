import tkinter as tk


class App(tk.Tk):

    def __init__(self) -> None:
        super().__init__()

        self.title("RS Analysis")
        self.minsize(640, 360)
        self.maxsize(1920, 1080)
        self.geometry("1280x720")
        self.config(background="#727f93")

        icon = tk.PhotoImage(file="./gui_assets/rs_anal_icon.png")
        self.iconphoto(True, icon)


if __name__ == "__main__":
    app = App()
    app.mainloop()
