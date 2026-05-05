import tkinter as tk
import tkinter.messagebox as messagebox
import subprocess
import threading
import webbrowser
import os
import sys
import ctypes

try:
    ctypes.windll.shcore.SetProcessDpiAwareness(1)
except Exception:
    pass

def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin()
    except:
        return False

class WinActivationCheckApp:
    def __init__(self, root):
        self.root = root
        self.root.title("WinActivationCheck_1.0.0")
        self.root.geometry("1176x490")
        
        self.slmgr_path = os.path.join(os.environ.get("SystemRoot", "C:\\Windows"), "System32", "slmgr.vbs")
        
        self.root.grid_rowconfigure(0, weight=0)
        self.root.grid_rowconfigure(1, weight=1)
        self.root.grid_columnconfigure(0, weight=1)
        self.root.grid_columnconfigure(1, weight=1)
        self.root.grid_columnconfigure(2, weight=1)
        
        self.menu_frame = tk.Frame(self.root, bg="#f0f0f0")
        self.menu_frame.grid(row=0, column=0, columnspan=3, sticky="ew")
        
        self.lbl_requery = self.create_menu_item("重新查询（F1）", self.requery_info)
        self.create_menu_item("卸载激活（F2）", self.uninstall_activation)
        self.create_menu_item("项目地址（F3）", self.open_project_url)
        
        self.root.bind("<F1>", lambda event: self.requery_info())
        self.root.bind("<F2>", lambda event: self.uninstall_activation())
        self.root.bind("<F3>", lambda event: self.open_project_url())
        
        self.text_dlv = tk.Text(self.root, bg="#555555", fg="white", font=("Consolas", 10), padx=10, pady=10)
        self.text_dlv.grid(row=1, column=0, sticky="nsew", padx=5, pady=5)
        
        self.text_dli = tk.Text(self.root, bg="#555555", fg="white", font=("Consolas", 10), padx=10, pady=10)
        self.text_dli.grid(row=1, column=1, sticky="nsew", padx=5, pady=5)
        
        self.text_xpr = tk.Text(self.root, bg="#555555", fg="white", font=("Consolas", 10), padx=10, pady=10)
        self.text_xpr.grid(row=1, column=2, sticky="nsew", padx=5, pady=5)
        
        self.requery_info()

    def create_menu_item(self, text, command):
        lbl = tk.Label(self.menu_frame, text=text, bg="#f0f0f0", fg="black", padx=15, pady=5, cursor="hand2")
        lbl.pack(side=tk.LEFT)
        lbl.bind("<Enter>", lambda e, l=lbl: l.config(bg="#d3d3d3") if str(l.cget("state")) == "normal" else None)
        lbl.bind("<Leave>", lambda e, l=lbl: l.config(bg="#f0f0f0"))
        lbl.bind("<Button-1>", lambda e, l=lbl: command() if str(l.cget("state")) == "normal" else None)
        return lbl

    def run_command(self, args):
        try:
            result = subprocess.run(args, capture_output=True, text=True, encoding='gbk', creationflags=subprocess.CREATE_NO_WINDOW)
            return result.stdout.strip() if result.stdout else result.stderr.strip()
        except Exception as e:
            return str(e)

    def requery_info(self):
        if hasattr(self, 'lbl_requery') and str(self.lbl_requery.cget("state")) == "disabled":
            return
            
        if hasattr(self, 'lbl_requery'):
            self.lbl_requery.config(state=tk.DISABLED, fg="gray", cursor="arrow", bg="#f0f0f0")
            
        self.text_dlv.delete(1.0, tk.END)
        self.text_dlv.insert(tk.END, "等待查询 DLV 信息...\n")
        
        self.text_dli.delete(1.0, tk.END)
        self.text_dli.insert(tk.END, "等待查询 DLI 信息...\n")
        
        self.text_xpr.delete(1.0, tk.END)
        self.text_xpr.insert(tk.END, "等待查询 XPR 信息...\n")
        
        threading.Thread(target=self.query_info, daemon=True).start()

    def query_info(self):
        dlv_out = self.run_command(["cscript", "//nologo", self.slmgr_path, "-dlv"])
        self.root.after(0, self.update_text, self.text_dlv, dlv_out)
        
        dli_out = self.run_command(["cscript", "//nologo", self.slmgr_path, "-dli"])
        self.root.after(0, self.update_text, self.text_dli, dli_out)
        
        xpr_out = self.run_command(["cscript", "//nologo", self.slmgr_path, "-xpr"])
        self.root.after(0, self.update_text, self.text_xpr, xpr_out)
        
        self.root.after(1000, self.enable_requery)

    def enable_requery(self):
        if hasattr(self, 'lbl_requery'):
            self.lbl_requery.config(state=tk.NORMAL, fg="black", cursor="hand2")

    def update_text(self, widget, text):
        widget.delete(1.0, tk.END)
        widget.insert(tk.END, text)

    def uninstall_activation(self):
        if messagebox.askyesno("二次确认", "您确定要卸载系统激活状态吗？"):
            self.text_dlv.delete(1.0, tk.END)
            self.text_dlv.insert(tk.END, "正在执行卸载操作，请稍候...\n")
            self.text_dli.delete(1.0, tk.END)
            self.text_xpr.delete(1.0, tk.END)
            threading.Thread(target=self.do_uninstall, daemon=True).start()

    def do_uninstall(self):
        self.run_command(["cscript", "//nologo", self.slmgr_path, "/upk"])
        self.run_command(["cscript", "//nologo", self.slmgr_path, "/rearm"])
        self.root.after(0, self.on_uninstall_done)

    def on_uninstall_done(self):
        messagebox.showinfo("完成", "系统激活状态卸载完毕。")
        self.requery_info()

    def open_project_url(self):
        webbrowser.open("https://github.com/NeetheCheeBao/WinActivationCheck")

if __name__ == "__main__":
    if is_admin():
        root = tk.Tk()
        app = WinActivationCheckApp(root)
        root.mainloop()
    else:
        params = " ".join([f'"{arg}"' for arg in sys.argv])
        ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, params, None, 1)
        sys.exit()