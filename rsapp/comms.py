import tkinter as tk


class SubmitForm():

    def __init__(self):
        self.day = tk.StringVar()
        self.month = tk.StringVar()
        self.year = tk.StringVar()

        self.hour = tk.StringVar()
        self.minute = tk.StringVar()
        self.second = tk.StringVar()

        self.trade_pair = tk.StringVar()

        self.duration = tk.StringVar()
        self.units = tk.StringVar()


    def submit_form(self):
        datetime = self.year.get() + self.month.get() + self.day.get() + "T" + self.hour.get() + self.minute.get() + self.second.get() + "Z"
        timestep = self.duration.get() + self.units.get()
        tradepair = self.trade_pair.get()

        print("Form is correct! \nDatetime: " + datetime + "\nTrade Pair: " + tradepair + "\nTime Step^ " + timestep)


if __name__ == "__main__":
    root = tk.Tk()
    submit_form = SubmitForm()
    root.mainloop()
