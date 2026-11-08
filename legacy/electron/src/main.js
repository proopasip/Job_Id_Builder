const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const axios = require('axios');

function createWindow() {
  const win = new BrowserWindow({
    width: 1100,
    height: 800,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      nodeIntegration: false,
      contextIsolation: true,
    },
    title: "Job ID Builder Modern"
  });

  win.loadFile('src/ui/index.html');
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

// API Logic
ipcMain.handle('fetch-servers', async (event, { placeId, limit }) => {
  try {
    const url = `https://games.roblox.com/v1/games/${placeId}/servers/Public?sortOrder=Asc&limit=${limit}`;
    const response = await axios.get(url, { timeout: 10000 });
    return { success: true, data: response.data.data };
  } catch (error) {
    return { 
      success: false, 
      error: error.response?.data?.message || error.message || "Unknown error occurred" 
    };
  }
});

ipcMain.handle('fetch-trending', async () => {
  try {
    // Using the more reliable games list API for popular games
    const url = 'https://games.roblox.com/v1/games/list?model.sort=PopularAsc';
    const response = await axios.get(url);
    return { success: true, data: response.data.games };
  } catch (error) {
    return { success: false, error: error.message };
  }
});

ipcMain.handle('search-games', async (event, { keyword }) => {
  try {
    // Roblox search API
    const url = `https://games.roblox.com/v1/games/list?model.keyword=${encodeURIComponent(keyword)}&model.sort=RelevanceDesc`;
    const response = await axios.get(url);
    return { success: true, data: response.data.games };
  } catch (error) {
    return { success: false, error: error.message };
  }
});
