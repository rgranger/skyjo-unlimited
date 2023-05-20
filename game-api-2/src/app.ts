import { Application } from "https://deno.land/x/oak/mod.ts";

export const app = new Application()

const socketIdMap: Map<WebSocket, string> = new Map()

app.use((ctx, next) => {
    if (!ctx.isUpgradable) {
        ctx.throw(501);
    }
    
    const ws = ctx.upgrade();
      
    ws.addEventListener("open", () => handleOpen(ws));
    ws.addEventListener("close", () => handleClose(ws));
    ws.addEventListener("error", (e) => handleError(ws, e));
    ws.addEventListener("message", (event) => handleMessage(ws, event));      
})

function handleError(ws: WebSocket, e: any) {
    console.log(e.message);
}

function handleOpen(ws: WebSocket) {
    socketIdMap.set(ws, crypto.randomUUID())
    console.log('Connected clients : ', socketIdMap.size)
}

function handleClose(ws: WebSocket) {
    socketIdMap.delete(ws)
    console.log('Connected clients : ', socketIdMap.size)
}
  
function handleMessage(ws: WebSocket, event: any) {
    const socketId = socketIdMap.get(ws)
    console.log(`${socketId} >> ${event.data}`);
}
