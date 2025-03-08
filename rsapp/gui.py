import tkinter as tk


# Главный класс с элементами ГПИ
class GUI_elements(tk.Frame):

    # Инициализиция всех финтефлюшек
    def __init__(self, parent) -> None:
        super().__init__(parent)

        # Создание заголовка
        self.title_frame = self.Title(self)
        self.title_frame.pack(pady=10)

        # Создание виджета ввода даты
        self.date_input_frame = self.Date_Input(self)
        self.date_input_frame.pack(pady=10)

        # Создание виджета ввода времени
        self.time_input_frame = self.Time_Input(self)
        self.time_input_frame.pack(pady=10)

        # Создание виджета ввода торговой пары
        self.trade_pair_input_frame = self.Trade_Pair_Input(self)
        self.trade_pair_input_frame.pack(pady=10)

        # Создание виджета ввода временного шага
        self.time_step_input_frame = self.Time_Step_Input(self)
        self.time_step_input_frame.pack(pady=10)


    # Заголовок
    class Title(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            # Заголовок
            self.label = tk.Label(self, text="Enter input data", font=("Calibri", 32, "bold"))
            self.label.pack(pady=10)


    # Ввод даты
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
    

    # Ввод времени
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


    # Ввод торговой пары
    class Trade_Pair_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.trade_pair_label = tk.Label(self, text="Trade Pair", font=("Calibri", 10, "bold"))
            self.trade_pair_label.grid(row=0, column=0, columnspan=5)

            self.trade_pair = tk.Entry(self, font=("Calibri", 10), width=32)
            self.trade_pair.grid(row=1, column=0)
    

    # Ввод интервала
    class Time_Step_Input(tk.Frame):

        def __init__(self, parent) -> None:
            super().__init__(parent)

            self.time_step_label = tk.Label(self, text="Time Step", font=("Calibri", 10, "bold"))
            self.time_step_label.grid(row=0, column=0, columnspan=5)

            self.value = tk.Entry(self, font=("Calibri", 10), width=10)
            self.value.grid(row=1, column=0)

            self.dimension = tk.Entry(self, font=("Calibri", 10), width=5)
            self.dimension.grid(row=1, column=1)


# Главный цикл
if __name__ == "__main__":
    root = tk.Tk()

    gui_el = GUI_elements(root)
    gui_el.pack(pady=10)

    root.mainloop()
