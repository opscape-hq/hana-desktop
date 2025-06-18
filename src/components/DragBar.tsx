import { getCurrentWindow } from "@tauri-apps/api/window";

export default function DragBar() {
  const handleMinimize = () => {
    const appWindow = getCurrentWindow();
    appWindow.minimize();
  };

  const handleMaximize = () => {
    const appWindow = getCurrentWindow();
    appWindow.maximize();
  };

  const handleClose = () => {
    const appWindow = getCurrentWindow();
    appWindow.close();
  };

  return (
    <div className="h-8 bg-card flex" data-tauri-drag-region>
      <div className="flex ml-auto">
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-foreground/10"
          onClick={handleMinimize}
        >
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
          />
        </button>
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-foreground/10"
          onClick={handleMaximize}
        >
          <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
          />
        </button>
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-red-400"
          onClick={handleClose}
        >
          <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </button>
      </div>
    </div>
  );
}
