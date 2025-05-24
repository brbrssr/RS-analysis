import flet as ft


class TradePairInput:
    # TradePairInput class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change

        self.trade_pair_tf = ft.TextField(label="Trade Pair", width=450)

        self.controls = ft.Row([self.trade_pair_tf])
    
        self.trade_pair_tf.on_change = self._on_any_change
    

    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def get_trade_pair(self):
        if self.trade_pair_tf.value:
            return f"{self.trade_pair_tf.value}".upper()
        return None
    

    def render(self):
        return self.controls
