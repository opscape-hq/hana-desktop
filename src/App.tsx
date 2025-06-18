import "./App.css";
import React, { useState, useEffect } from "react";
import {
  Terminal,
  Folder,
  Settings,
  Plus,
  X,
  Minimize2,
  Maximize2,
} from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

// Tab types
const TAB_TYPES = {
  TERMINAL: "terminal",
  SFTP: "sftp",
  SETTINGS: "settings",
};

// Mock data for demonstration
const mockConnections = [
  {
    id: 1,
    name: "Production Server",
    host: "192.168.1.100",
    status: "connected",
  },
  { id: 2, name: "Development", host: "192.168.1.101", status: "disconnected" },
  { id: 3, name: "Staging", host: "192.168.1.102", status: "connected" },
];

const mockFiles = [
  { name: "home", type: "folder", size: "", modified: "2024-01-15" },
  { name: "var", type: "folder", size: "", modified: "2024-01-14" },
  { name: "etc", type: "folder", size: "", modified: "2024-01-13" },
  { name: "config.json", type: "file", size: "2.3 KB", modified: "2024-01-15" },
  { name: "server.log", type: "file", size: "45.7 MB", modified: "2024-01-15" },
];

// Components
const DragBar = () => {
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
    <div
      className="h-8 flex bg-background/80 backdrop-blur-xl border-b border-foreground/10"
      data-tauri-drag-region
    >
      <div className="flex ml-auto">
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-foreground/10 transition-colors"
          onClick={handleMinimize}
        >
          <Minimize2 size={14} />
        </button>
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-foreground/10 transition-colors"
          onClick={handleMaximize}
        >
          <Maximize2 size={14} />
        </button>
        <button
          className="w-12 h-full flex items-center justify-center hover:bg-red-400 hover:text-white transition-colors"
          onClick={handleClose}
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
};

const Logo = () => (
  <div className="flex items-center gap-2 px-4 py-2">
    <div className="w-6 h-6 bg-gradient-to-br from-blue-500 to-purple-600 rounded-md flex items-center justify-center">
      <Terminal size={14} className="text-white" />
    </div>
    <span className="font-display font-semibold text-sm">hana</span>
  </div>
);

const TabButton = ({ tab, isActive, onClick, onClose }) => (
  <div
    className={`
      group flex items-center gap-2 px-3 py-2 text-sm transition-all duration-200 cursor-pointer
      border-r border-foreground/10 hover:bg-foreground/5 relative
      ${isActive ? "bg-background/50 backdrop-blur-sm" : ""}
    `}
    onClick={onClick}
  >
    {tab.type === TAB_TYPES.TERMINAL && <Terminal size={14} />}
    {tab.type === TAB_TYPES.SFTP && <Folder size={14} />}
    {tab.type === TAB_TYPES.SETTINGS && <Settings size={14} />}

    <span className="max-w-32 truncate">{tab.title}</span>

    {tab.status && (
      <div
        className={`w-2 h-2 rounded-full ${
          tab.status === "connected" ? "bg-green-400" : "bg-red-400"
        }`}
      />
    )}

    <button
      className="opacity-0 group-hover:opacity-100 hover:bg-foreground/20 rounded p-0.5 transition-all"
      onClick={(e) => {
        e.stopPropagation();
        onClose(tab.id);
      }}
    >
      <X size={12} />
    </button>

    {isActive && (
      <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-500" />
    )}
  </div>
);

const NewTabButton = ({ onClick }) => (
  <button
    className="flex items-center justify-center w-10 h-10 hover:bg-foreground/10 transition-colors border-r border-foreground/10"
    onClick={onClick}
  >
    <Plus size={16} />
  </button>
);

const TerminalView = ({ tab }) => {
  const [terminalHistory, setTerminalHistory] = useState([
    `Connected to ${tab.connection?.name || "server"}`,
    "user@server:~$ ls -la",
    "total 48",
    "drwxr-xr-x  8 user user 4096 Jan 15 10:30 .",
    "drwxr-xr-x  3 root root 4096 Jan 10 09:15 ..",
    "-rw-r--r--  1 user user  220 Jan 10 09:15 .bash_logout",
    "-rw-r--r--  1 user user 3771 Jan 10 09:15 .bashrc",
    "drwxr-xr-x  2 user user 4096 Jan 12 14:20 Documents",
    "drwxr-xr-x  2 user user 4096 Jan 12 14:20 Downloads",
    "user@server:~$ ",
  ]);

  return (
    <div className="flex-1 bg-black/80 backdrop-blur-sm rounded-lg m-4 p-4 font-mono text-sm text-green-400 overflow-hidden">
      <div className="h-full overflow-y-auto">
        {terminalHistory.map((line, index) => (
          <div key={index} className="whitespace-pre-wrap">
            {line}
            {index === terminalHistory.length - 1 && (
              <span className="animate-pulse bg-green-400 w-2 h-4 inline-block ml-1" />
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

const SFTPView = ({ tab }) => (
  <div className="flex-1 p-4">
    <div className="bg-card/50 backdrop-blur-sm rounded-lg h-full p-4">
      <div className="flex items-center gap-2 mb-4 pb-2 border-b border-foreground/10">
        <Folder size={16} />
        <span className="font-medium">
          File Explorer - {tab.connection?.name}
        </span>
      </div>

      <div className="grid grid-cols-12 gap-4 text-sm font-medium text-foreground/70 mb-2">
        <div className="col-span-6">Name</div>
        <div className="col-span-2">Size</div>
        <div className="col-span-4">Modified</div>
      </div>

      <div className="space-y-1">
        {mockFiles.map((file, index) => (
          <div
            key={index}
            className="grid grid-cols-12 gap-4 text-sm p-2 rounded hover:bg-foreground/5 transition-colors cursor-pointer"
          >
            <div className="col-span-6 flex items-center gap-2">
              {file.type === "folder" ? (
                <Folder size={16} className="text-blue-500" />
              ) : (
                <div className="w-4 h-4 bg-foreground/20 rounded" />
              )}
              <span>{file.name}</span>
            </div>
            <div className="col-span-2 text-foreground/70">{file.size}</div>
            <div className="col-span-4 text-foreground/70">{file.modified}</div>
          </div>
        ))}
      </div>
    </div>
  </div>
);

const SettingsView = () => (
  <div className="flex-1 p-4">
    <div className="bg-card/50 backdrop-blur-sm rounded-lg h-full p-6">
      <div className="flex items-center gap-2 mb-6">
        <Settings size={20} />
        <h2 className="text-xl font-display font-semibold">Settings</h2>
      </div>

      <div className="space-y-6">
        <div>
          <h3 className="font-medium mb-3">Connections</h3>
          <div className="space-y-2">
            {mockConnections.map((conn) => (
              <div
                key={conn.id}
                className="flex items-center justify-between p-3 bg-background/50 rounded-lg"
              >
                <div>
                  <div className="font-medium">{conn.name}</div>
                  <div className="text-sm text-foreground/70">{conn.host}</div>
                </div>
                <div
                  className={`px-2 py-1 rounded text-xs ${
                    conn.status === "connected"
                      ? "bg-green-400/20 text-green-600"
                      : "bg-red-400/20 text-red-600"
                  }`}
                >
                  {conn.status}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div>
          <h3 className="font-medium mb-3">Appearance</h3>
          <div className="space-y-3">
            <label className="flex items-center gap-2">
              <input type="checkbox" className="rounded" defaultChecked />
              <span className="text-sm">Use transparency effects</span>
            </label>
            <label className="flex items-center gap-2">
              <input type="checkbox" className="rounded" />
              <span className="text-sm">Dark mode</span>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
);

const renderTabContent = (tab) => {
  switch (tab.type) {
    case TAB_TYPES.TERMINAL:
      return <TerminalView tab={tab} />;
    case TAB_TYPES.SFTP:
      return <SFTPView tab={tab} />;
    case TAB_TYPES.SETTINGS:
      return <SettingsView />;
    default:
      return (
        <div className="flex-1 flex items-center justify-center">
          Unknown tab type
        </div>
      );
  }
};

export default function App() {
  const [tabs, setTabs] = useState([
    {
      id: 1,
      type: TAB_TYPES.TERMINAL,
      title: "Terminal - Production",
      connection: mockConnections[0],
      status: "connected",
    },
    {
      id: 2,
      type: TAB_TYPES.SFTP,
      title: "SFTP - Production",
      connection: mockConnections[0],
      status: "connected",
    },
  ]);

  const [activeTabId, setActiveTabId] = useState(1);
  const [nextTabId, setNextTabId] = useState(3);

  const activeTab = tabs.find((tab) => tab.id === activeTabId);

  const createNewTab = () => {
    const newTab = {
      id: nextTabId,
      type: TAB_TYPES.TERMINAL,
      title: `Terminal ${nextTabId}`,
      connection: null,
      status: "disconnected",
    };

    setTabs((prev) => [...prev, newTab]);
    setActiveTabId(nextTabId);
    setNextTabId((prev) => prev + 1);
  };

  const closeTab = (tabId) => {
    setTabs((prev) => {
      const newTabs = prev.filter((tab) => tab.id !== tabId);
      if (activeTabId === tabId && newTabs.length > 0) {
        setActiveTabId(newTabs[0].id);
      }
      return newTabs;
    });
  };

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.ctrlKey || e.metaKey) {
        if (e.key === "t") {
          e.preventDefault();
          createNewTab();
        } else if (e.key === "w") {
          e.preventDefault();
          if (tabs.length > 1) {
            closeTab(activeTabId);
          }
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [activeTabId, tabs.length]);

  return (
    <div className="h-screen flex flex-col bg-background text-foreground">
      <DragBar />

      {/* Header with logo and tabs */}
      <div className="flex items-center bg-card/30 backdrop-blur-xl border-b border-foreground/10">
        <Logo />

        <div className="flex-1 flex items-center overflow-x-auto">
          {tabs.map((tab) => (
            <TabButton
              key={tab.id}
              tab={tab}
              isActive={tab.id === activeTabId}
              onClick={() => setActiveTabId(tab.id)}
              onClose={closeTab}
            />
          ))}
          <NewTabButton onClick={createNewTab} />
        </div>
      </div>

      {/* Main content area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {activeTab ? (
          <div className="flex-1 flex flex-col animate-in fade-in duration-200">
            {renderTabContent(activeTab)}
          </div>
        ) : (
          <div className="flex-1 flex items-center justify-center text-foreground/50">
            <div className="text-center">
              <h2 className="text-xl font-display font-semibold mb-2">
                No tabs open
              </h2>
              <p className="text-sm">Press Ctrl+T to create a new tab</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
