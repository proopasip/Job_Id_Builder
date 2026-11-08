const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
    fetchServers: (data) => ipcRenderer.invoke('fetch-servers', data),
    fetchTrending: () => ipcRenderer.invoke('fetch-trending'),
});
