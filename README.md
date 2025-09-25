# 🛠 Job Id Builder 
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Python](https://img.shields.io/badge/PyQT-76AB37?style=for-the-badge&logo=gear&logoColor=white)
![Python](https://img.shields.io/badge/API-48474F?style=for-the-badge&logo=gear&logoColor=white)
## 🌐What is Job id builder?

- An executable programm, to sort and find the best one roblox's curent game server.
- Helpful tool, to get rid of 400ms in game

## 🧐 What is job id?

Job id is a unique server identifier on Roblox.

It typically takes the form xxxxxxxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx, where each "x" represents a hexadecimal value (0 through 9 and a, b, c, d, e, f).

## 🔧 How it works?
### 1. Asks for a json, via API

  Example link: https://games.roblox.com/v1/games/10449761463/servers/Public?sortOrder=Asc&limit=50

  Instead of 10449761463 you can enter your own game id. Look to the list.json

  
### 2. Receives Json

```json
[{"id":"f53e5181-3264-4536-b564-727de65a04e5","maxPlayers":15,"playing":1,"playerTokens":["9C39B894E6AF667C5E7D723047EA0AF9"],"players":[],"fps":59.925491,"ping":94},{"id":"4de2fc9e-bdc9-426f-9da4-c751b0f1329e","maxPlayers":15,"playing":1,"playerTokens":["EE2DD158A8EC4554546E337BFB293E26"],"players":[],"fps":59.992165,"ping":237},{"id":"46904c41-3640-4458-ab57-563c5391d939","maxPlayers":15,"playing":1,"playerTokens":["4ECFDCD21912FB999CF74D7C566E55A5"],"players":[],"fps":59.866371,"ping":242},{"id":"111220eb-9c3f-4545-b4ed-892aed3f8ae1","maxPlayers":15,"playing":1,"playerTokens":["4ED8B6BF2DC379020A07387E8AAD5907"],"players":[],"fps":59.990593,"ping":250},{"id":"c219b748-f8b5-4e55-9dc3-a28a1a5caab3","maxPlayers":15,"playing":1,"playerTokens":["E9A7D82DA89678B8C649BBF417D874B0"],"players":[],"fps":59.940697,"ping":241}
```

### 3. Script sorts recived json and shows everything in a table

| **Server 1** | **Server 2** | **Server 3** |
|-------------|-------------|-------------|
| ping        | ping        | ping        |
| players     | players     | players     |
| job id      | job id      | job id      |

select a server by clicking on top of the column

### 4. Then it build a link to join selected roblox's exact server

Example join link: 

https://www.roblox.com/games/start?placeId=10449761463&launchData=TARGETGAMEID/ad2bf658-325f-49a2-aa67-64778e8c712c

After you paste it you will join immediately

### 5. Paste it in any browser to join.

## 🛒 Requirements
- Roblox player
- Windows 10

## ⚙ Set up
1. Download .exe
2. Download Preview
3. Download List.json
4. Create a folder and paste everything in it
5. Launch .exe

### 👍 If everythig done well. this window will show up

<img width="699" height="630" alt="App" src="https://github.com/user-attachments/assets/9b44e08c-3c71-4045-a62f-206d16f2bce9" />

### ♨ Select and click on top of the column, to build a link.

<img width="700" height="629" alt="App1" src="https://github.com/user-attachments/assets/121de756-3391-4ac4-a2b6-146322f296f5" />
