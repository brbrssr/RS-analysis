import tkinter as tk


# Main class with GUI elements
class GUI_elements(tk.Frame):

    # Initializing all that stuff
    def __init__(self, parent) -> None:
        super().__init__(parent)

        # Title creation
        self.title_frame = self.Title(self)
        self.title_frame.pack(pady=10)

        # Date input widget creation
        self.date_input_frame = self.Date_Input(self)
        self.date_input_frame.pack(pady=10)

        # Time input widget creation
        self.time_input_frame = self.Time_Input(self)
        self.time_input_frame.pack(pady=10)

        # Trade pair input widget creation
        self.trade_pair_input_frame = self.Trade_Pair_Input(self)
        self.trade_pair_input_frame.pack(pady=10)

        # Time step inout widget creation
        self.time_step_input_frame = self.Time_Step_Input(self)
        self.time_step_input_frame.pack(pady=10)


    # Title
    class Title(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.label = tk.Label(self, text="Enter input data", font=("Calibri", 32, "bold"))
            self.label.pack(pady=10)


    # Date input
    class Date_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.date_label = tk.Label(self, text="Date", font=("Calibri", 10, "bold"))
            self.date_label.grid(row=0, column=0, columnspan=5)

            self.day = tk.Entry(self, font=("Calibri", 10), width=5)
            self.day.grid(row=1, column=0)

            self.sl_div1 = tk.Label(self, text="/", font=("Calibri", 10))
            self.sl_div1.grid(row=1, column=1)

            self.month = tk.Entry(self, font=("Calibri", 10), width=5)
            self.month.grid(row=1, column=2)

            self.sl_div2 = tk.Label(self, text="/", font=("Calibri", 10))
            self.sl_div2.grid(row=1, column=3)

            self.year = tk.Entry(self, font=("Calibri", 10), width=8)
            self.year.grid(row=1, column=4)
    

    # Time input
    class Time_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.time_label = tk.Label(self, text="Time", font=("Calibri", 10, "bold"))
            self.time_label.grid(row=0, column=0, columnspan = 5)

            self.hour = tk.Entry(self, font=("Calibri", 10), width=5)
            self.hour.grid(row=1, column=0)

            self.col_div1 = tk.Label(self, text=":", font=("Calibri", 10))
            self.col_div1.grid(row=1, column=1)

            self.minute = tk.Entry(self, font=("Calibri", 10), width=5)
            self.minute.grid(row=1, column=2)

            self.col_div2 = tk.Label(self, text=":", font=("Calibri", 10))
            self.col_div2.grid(row=1, column=3)

            self.second = tk.Entry(self, font=("Calibri", 10), width=5)
            self.second.grid(row=1, column=4)


    # Trade pair input
    class Trade_Pair_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.trade_pair_label = tk.Label(self, text="Trade Pair", font=("Calibri", 10, "bold"))
            self.trade_pair_label.grid(row=0, column=0, columnspan=5)

            self.trade_pair = tk.Entry(self, font=("Calibri", 10), width=32)
            self.trade_pair.grid(row=1, column=0)
    

    # Time step input
    class Time_Step_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.time_step_label = tk.Label(self, text="Time Step", font=("Calibri", 10, "bold"))
            self.time_step_label.grid(row=0, column=0, columnspan=5)

            self.value = tk.Entry(self, font=("Calibri", 10), width=10)
            self.value.grid(row=1, column=0)

            self.dimension = tk.Entry(self, font=("Calibri", 10), width=5)
            self.dimension.grid(row=1, column=1)


# Main cycle
if __name__ == "__main__":
    root = tk.Tk()

    gui_el = GUI_elements(root)
    gui_el.pack(pady=10)

    root.mainloop()
