import customtkinter as ctk
import vpn_core  # Импортируем наш модуль, написанный на Rust!

class VpnApp(ctk.CTk):
    def __init__(self):
        super().__init__()
        self.title("VPN Client")
        self.geometry("300x200")

        self.label = ctk.CTkLabel(self, text="Скорость: 0 Кб/с", font=("Arial", 16))
        self.label.pack(pady=30)

        self.btn = ctk.CTkButton(self, text="Запустить ядро Rust", command=self.start_vpn)
        self.btn.pack(pady=10)

    def on_speed_update(self, speed):
        # Эту функцию вызывает Rust из фонового потока
        self.label.configure(text=f"Скорость от Rust: {speed} Кб/с")

    def start_vpn(self):
        self.btn.configure(state="disabled", text="Ядро работает")
        # Передаем ссылку на функцию-коллбэк в Rust
        vpn_core.start_core(self.on_speed_update)

if __name__ == "__main__":
    app = VpnApp()
    app.mainloop()